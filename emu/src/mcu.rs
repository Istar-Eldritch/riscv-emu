use crate::cpu::{CSRs, CPU};
use crate::instructions::Instruction;
use crate::instructions::{Exception, ExceptionInterrupt, Interrupt};
use crate::memory::ClockedMemory;
use riscv_isa_types::{privileged::RVPrivileged, rv32i::RV32i};

// Micro controller unit
pub struct MCU {
    cpu: CPU,
    mem: Box<dyn ClockedMemory>,
}

impl MCU {
    pub fn new(cpu: CPU, mem: Box<dyn ClockedMemory>) -> Self {
        Self { cpu, mem }
    }

    pub fn flash(&mut self, mem: Vec<u8>) {
        for i in 0..mem.len() {
            self.mem.wb(i as u32, mem[i]).unwrap();
        }
    }

    fn run_instruction(&mut self, word: u32) -> Result<u32, ExceptionInterrupt> {
        let v = if let Ok(v) = RVPrivileged::try_from(word) {
            log::trace!("instruction: {:?}", v);
            let cost = v.execute(&mut self.cpu, self.mem.as_mut_mem())?;
            v.update_pc(&mut self.cpu);
            cost
        } else if let Ok(v) = RV32i::try_from(word) {
            log::trace!("instruction: {:?}", v);
            let cost = v.execute(&mut self.cpu, self.mem.as_mut_mem())?;
            v.update_pc(&mut self.cpu);
            cost
        } else {
            log::error!("error decoding instruction: {word:b} at {:x}", self.cpu.pc);
            return Err(ExceptionInterrupt::Exception(Exception::IllegalInstruction));
        };

        Ok(v)
    }

    /// Software interrupt pending check
    fn update_mip_msip(&mut self) {
        let software_interrupt = self.mem.rw(0x200_0000).unwrap();

        let mip = self.cpu.get_csr(CSRs::mip as u32).unwrap();
        let mip_msi = if software_interrupt > 0 {
            self.mem.ww(0x200_0000, 0).unwrap();
            mip | (1 << Interrupt::MSoftInterrupt as u32)
        } else {
            mip & !(1 << Interrupt::MSoftInterrupt as u32)
        };

        self.cpu.set_csr(CSRs::mip as u32, mip_msi).unwrap();
    }

    /// Timer interrupt pending check
    fn update_mip_mtip(&mut self) {
        use Interrupt::*;

        let cmp_time: u64 = self.mem.rw(0x200_4000).unwrap() as u64;

        let cmp_time: u64 = cmp_time | ((self.mem.rw(0x200_4004).unwrap() as u64) << 4);

        let time: u64 = self.mem.rw(0x200_bff8).unwrap() as u64;
        let time: u64 = time | ((self.mem.rw(0x200_bffc).unwrap() as u64) << 4);

        let mip = self.cpu.get_csr(CSRs::mip as u32).unwrap();
        let mip_mti = if cmp_time != 0 && time >= cmp_time {
            mip | (1 << MTimerInterrupt as u32)
        } else {
            mip & !(1 << MTimerInterrupt as u32)
        };
        self.cpu.set_csr(CSRs::mip as u32, mip_mti).unwrap();
    }

    /// External interrupt pending bit check
    fn update_mip_meip(&mut self) {
        let external_interrupts = self.mem.rw(0x0c00_1000).unwrap() as u64
            | ((self.mem.rw(0x0c00_1004).unwrap() as u64) << 4);
        let mip = self.cpu.get_csr(CSRs::mip as u32).unwrap();
        let mip_mei = if external_interrupts != 0 {
            mip | (1 << Interrupt::MExternalInterrupt as u32)
        } else {
            mip & !(1 << Interrupt::MExternalInterrupt as u32)
        };

        self.cpu.set_csr(CSRs::mip as u32, mip_mei).unwrap();
    }

    fn get_interrupt(&mut self) -> Option<Interrupt> {
        use Interrupt::*;
        let mie = self.cpu.get_csr(CSRs::mie as u32).unwrap();
        let mie_msi = (mie & (1 << MSoftInterrupt as u32)) != 0;
        let mie_mti = mie & (1 << MTimerInterrupt as u32) != 0;
        let mie_mei = mie & (1 << MExternalInterrupt as u32) != 0;

        let mip = self.cpu.get_csr(CSRs::mip as u32).unwrap();
        let mip_msi = mip & (1 << MSoftInterrupt as u32) != 0;
        let mip_mti = mip & (1 << MTimerInterrupt as u32) != 0;
        let mip_mei = mip & (1 << MExternalInterrupt as u32) != 0;

        match mip {
            // TODO: Handle interrupts for other privilege modes
            _v if mie_msi && mip_msi => {
                self.cpu
                    .set_csr(CSRs::mip as u32, mip & !(1 << MSoftInterrupt as u32))
                    .unwrap();
                Some(MSoftInterrupt)
            }
            _v if mie_mti && mip_mti => Some(MTimerInterrupt),
            _v if mie_mei && mip_mei => Some(MExternalInterrupt),
            _ => None,
        }
    }

    pub fn tick(&mut self) -> TickResult {
        self.mem.tick(());
        let pc = self.cpu.pc;
        let word = self.mem.rw(pc);
        let mstatus = self.cpu.get_csr(CSRs::mstatus as u32).unwrap();
        let mstatus_mie = (mstatus & (1 << 3)) != 0;

        let mut interrupt = None;
        if mstatus_mie {
            self.update_mip_msip();
            self.update_mip_mtip();
            self.update_mip_meip();
            interrupt = self.get_interrupt();
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

    // Handles interrupts and exceptions
    fn handle_exception(&mut self, exc: ExceptionInterrupt) -> TickResult {
        log::trace!("excp: {:?}", exc);
        let mstatus = self.cpu.get_csr(CSRs::mstatus as u32).unwrap();
        let mstatus_mie = mstatus & (1 << 3);
        let mie = self.cpu.get_csr(CSRs::mie as u32).unwrap();
        let mip = self.cpu.get_csr(CSRs::mip as u32).unwrap();

        let _mcause = match exc {
            ExceptionInterrupt::Interrupt(i) => {
                self.cpu.wfi = false;
                if mstatus_mie != 0 && (mie & (1 << i as u32) != 0) {
                    self.cpu
                        .set_csr(CSRs::mcause as u32, i as u32 | (1 << 31))
                        .unwrap();
                    if i == Interrupt::MExternalInterrupt {
                        // XXX: For external interrupts, this implementation resets the PLIC pending
                        // bit, and sets the interrupt value to mtval which may not be the correct vehaviour.
                        let plic_int = self.mem.rw(0x0c20_0004).unwrap();
                        self.cpu.set_csr(CSRs::mtval as u32, plic_int).unwrap();
                        self.mem.ww(0x0c20_0004, plic_int).unwrap();
                    }
                    // XXX: This implementation reset the pending interrupt bit before the handler
                    // executes it, I'm not sure this is the correct behaviour
                    self.cpu
                        .set_csr(CSRs::mip as u32, mip & !(i as u32))
                        .unwrap();
                } else {
                    return TickResult::Cycles(4);
                };
                i as u32
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
