use crate::cpu::{CSRs, CPU};
use crate::instructions::{Exception, ExceptionInterrupt};
use crate::instructions::{Instruction, Interrupt};
use crate::interrupt_controller::InterruptController;
use crate::memory::DeviceMap;
use crate::memory::{DeviceMeta, Memory, MMU};
use crate::peripherals::{Peripheral, PeripheralWrapper};
use riscv_isa_types::{privileged::RVPrivileged, rv32i::RV32i};
use std::collections::BTreeMap;

pub struct DeviceDef {
    pub identifier: String,
    pub memory_start: u32,
    pub memory_end: u32,
    pub device: Box<dyn Peripheral>,
}

// Micro controller unit
pub struct MCU {
    pub cpu: CPU,
    pub int_ctrl: InterruptController,
    pub mmu: MMU,
    pub devices: DeviceMap,
}

impl MCU {
    pub fn new() -> Self {
        let devices = std::rc::Rc::new(std::cell::RefCell::new(BTreeMap::new()));
        Self {
            cpu: CPU::new(),
            int_ctrl: InterruptController::new(std::rc::Rc::clone(&devices)),
            mmu: MMU::new(std::rc::Rc::clone(&devices)),
            devices,
        }
    }

    pub fn add_device(&mut self, device: DeviceDef) -> Result<&mut Self, ()> {
        self.mmu.insert_device(DeviceMeta::new(
            device.identifier.clone(),
            device.memory_start,
            device.memory_end,
        ))?;
        self.devices
            .borrow_mut()
            .insert(device.identifier, std::cell::RefCell::new(device.device));
        Ok(self)
    }

    pub fn flash(&mut self, mem: Vec<u8>) {
        for i in 0..mem.len() {
            self.mmu.wb(i as u32, mem[i]).unwrap();
        }
    }

    fn run_instruction(&mut self, word: u32) -> Result<u32, ExceptionInterrupt> {
        let v = if let Ok(v) = RVPrivileged::try_from(word) {
            log::trace!("instruction: {:?}", v);
            let cost = v.execute(self)?;
            v.update_pc(self);
            cost
        } else if let Ok(v) = RV32i::try_from(word) {
            log::trace!("instruction: {:?}", v);
            let cost = v.execute(self)?;
            v.update_pc(self);
            cost
        } else {
            log::error!("error decoding instruction: {word:b} at {:x}", self.cpu.pc);
            return Err(ExceptionInterrupt::Exception(Exception::IllegalInstruction));
        };

        Ok(v)
    }

    pub fn tick(&mut self) -> TickResult {
        {
            let devices = std::rc::Rc::clone(&self.devices);
            for (_k, device) in devices.borrow().iter() {
                let deviceref = &mut *device.borrow_mut();
                deviceref.tick(&mut self.int_ctrl);
            }
            self.int_ctrl.notify_cpu(&mut self.cpu);
        };
        let pc = self.cpu.pc;
        let word = self.mmu.rw(pc);
        let mstatus = self.cpu.get_csr(CSRs::mstatus as u32).unwrap();
        let mstatus_mie = (mstatus & (1 << 3)) != 0;

        let mut interrupt = None;
        if mstatus_mie {
            interrupt = self.cpu.get_interrupt();
        }

        if let Some(exc) = interrupt {
            log::trace!("interrupt - mstatus: {mstatus}, exc: {exc:?}, pc: {pc:x}");
            self.handle_exception(ExceptionInterrupt::Interrupt(exc))
        } else if self.cpu.wfi {
            TickResult::WFI
        } else if let Ok(0) = word {
            self.handle_exception(ExceptionInterrupt::Exception(Exception::IllegalInstruction))
        } else if let Ok(addr) = word {
            self.cpu.log_registers();
            match self.run_instruction(addr) {
                Ok(v) => TickResult::Cycles(v),
                Err(err) => self.handle_exception(err),
            }
        } else {
            self.handle_exception(ExceptionInterrupt::Exception(
                Exception::InstructionAccessFault,
            ))
        }
    }

    pub fn interrupt(&mut self, interrupt: Interrupt, id: u32) {
        // If the PLIC is present, trigger external interrupts through it,
        // otherwise do it directly
        if interrupt == Interrupt::MExternalInterrupt {
            if let Some(Ok(mut device)) = self
                .devices
                .borrow()
                .get("PLIC")
                .map(|d| d.try_borrow_mut())
            {
                let plic = <&mut dyn Peripheral as TryInto<
                    PeripheralWrapper<&mut crate::peripherals::plic::PLIC>,
                >>::try_into(&mut **device)
                .unwrap();
                let mut pending = plic.pending.borrow_mut();
                *pending |= id as u64;
            }
        }

        let mip = self.cpu.get_csr(CSRs::mip as u32).unwrap();
        let mip_mei = mip | (1 << interrupt as u32);
        self.cpu.set_csr(CSRs::mip as u32, mip_mei).unwrap();
    }

    // Handles interrupts and exceptions
    fn handle_exception(&mut self, exc: ExceptionInterrupt) -> TickResult {
        log::trace!("excp: {:?}", exc);
        let mstatus = self.cpu.get_csr(CSRs::mstatus as u32).unwrap();
        let mstatus_mie = mstatus & (1 << 3);
        let mie = self.cpu.get_csr(CSRs::mie as u32).unwrap();

        let _mcause = match exc {
            ExceptionInterrupt::Interrupt(i) => {
                self.cpu.wfi = false;
                let cause = 1 << i as u32;
                if mstatus_mie != 0 && (mie & cause != 0) {
                    self.cpu
                        .set_csr(CSRs::mcause as u32, cause | (1 << 31))
                        .unwrap();
                } else {
                    return TickResult::Cycles(4);
                };
                cause
            }
            ExceptionInterrupt::Exception(e) => {
                match e {
                    Exception::MEnvironmentCall if self.cpu.get_x(10) == 255 => {
                        return TickResult::HALT
                    }
                    Exception::MEnvironmentCall if self.cpu.get_x(10) == 254 => {
                        let start = self.cpu.get_x(11);
                        let end = self.cpu.get_x(12);
                        let range = std::ops::RangeInclusive::new(start, end);
                        return TickResult::Dump(range);
                    }
                    _ => {
                        // XXX: Exceptions add the pc to the mtval, which may not be the correct
                        // way to pass the pc to the handler
                        self.cpu.set_csr(CSRs::mcause as u32, e as u32).unwrap();
                        self.cpu.set_csr(CSRs::mtval as u32, self.cpu.pc).unwrap();
                    }
                };
                e as u32
            }
        };

        let mstatus = (mstatus & !(1 << 3)) | mstatus_mie << 4; // move mie to mpie and set mie to 0 (disable interrupts)
        self.cpu.set_csr(CSRs::mstatus as u32, mstatus).unwrap();
        self.cpu.set_csr(CSRs::mepc as u32, self.cpu.pc).unwrap();
        // TODO: Set privilege mode on msatus.MPP
        let mtvec = self.cpu.get_csr(CSRs::mtvec as u32).unwrap();
        self.cpu.pc = mtvec;
        TickResult::Cycles(4)
    }
}

pub enum TickResult {
    WFI,
    HALT,
    Dump(std::ops::RangeInclusive<u32>),
    Cycles(u32),
}
