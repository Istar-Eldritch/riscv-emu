mod instruction_set;
mod memory;
mod utils;

use instruction_set::{privileged::RVPrivileged, rv32i::RV32i, Instruction};
use memory::{Memory, MMU};

#[derive(Debug, Clone, Copy)]
pub enum Interrupt {
    SSoftInterrupt = 1,
    MSoftInterrupt = 3,
    STimerInterrupt = 5,
    MTimerInterrupt = 7,
    SExternalInterrupt = 9,
    MExternalInterrupt = 11,
}

#[derive(Debug, Clone, Copy)]
pub enum Exception {
    InstructionAddressMissaligned = 0,
    InstructionAccessFault = 1,
    IllegalInstruction = 2,
    Breakpoint = 3,
    LoadAddressMisaligned = 4,
    LoadAccessFault = 5,
    StoreAddressMisaligned = 6,
    StoreAccessFault = 7,
    UEnvironmentCall = 8,
    SEnvironmentCall = 9,
    MEnvironmentCall = 11,
    InstructionPageFault = 12,
    LoadPageFault = 13,
    StorePageFault = 14,
}

#[derive(Debug)]
pub enum ExceptionInterrupt {
    Interrupt(Interrupt),
    Exception(Exception),
}

impl std::fmt::Display for ExceptionInterrupt {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        fmt.write_fmt(format_args!("{:?}", self))
    }
}

impl std::error::Error for ExceptionInterrupt {}

pub struct Emulator {
    cpu: CPU,
    mem: Box<dyn Memory>,
    speed: u32, // speed in hz
}

fn wait_cycles(hz: u32, cycles: u32) {
    std::thread::sleep(std::time::Duration::from_nanos(
        (1e9 / hz as f64).round() as u64 * cycles as u64,
    ))
}

pub enum TickResult {
    WFI,
    HALT,
    Cycles(u32),
}

impl Emulator {
    pub fn new(speed: u32) -> Self {
        Emulator {
            cpu: CPU::new(),
            mem: Box::new(MMU::new()),
            speed,
        }
    }

    pub fn flash(&mut self, mem: Vec<u8>) {
        for i in 0..mem.len() {
            self.mem.wb(i as u32, mem[i]);
        }
    }

    fn run_instruction(&mut self, word: u32) -> Result<u32, ExceptionInterrupt> {
        let v = if let Ok(v) = RVPrivileged::try_from(word) {
            log::debug!("instruction: {:?}", v);
            v.execute(&mut self.cpu, &mut *self.mem)?
        } else if let Ok(v) = RV32i::try_from(word) {
            log::debug!("instruction: {:?}", v);
            v.execute(&mut self.cpu, &mut *self.mem)?
        } else {
            log::debug!("error decoding instruction: {word}");
            return Err(ExceptionInterrupt::Exception(Exception::IllegalInstruction));
        };

        self.cpu.pc += 4;
        Ok(v)
    }

    fn generate_clint_interrupts(&mut self) {
        use Interrupt::*;
        let mie = self.cpu.get_csr(CSRs::mie as u32).unwrap();
        let msi = (mie & (1 << MSoftInterrupt as u32)) != 0;
        let software_interrupt = self.mem.rw(0x200_0000);
        if msi && software_interrupt > 0 {
            let mip = self.cpu.get_csr(CSRs::mip as u32).unwrap();
            let mip = mip | (1 << MSoftInterrupt as u32);
            self.cpu.set_csr(CSRs::mip as u32, mip).unwrap();
            self.mem.ww(0x200_0000, 0);
        }

        let mti = (mie & (1 << MTimerInterrupt as u32)) != 0;

        let cmp_time: u64 = self.mem.rw(0x200_4000) as u64;

        let cmp_time: u64 = cmp_time | ((self.mem.rw(0x200_4004) as u64) << 4);

        let time: u64 = self.mem.rw(0x200_bff8) as u64;
        let time: u64 = time | ((self.mem.rw(0x200_bffc) as u64) << 4);
        let time = time + 1;

        if mti && time >= cmp_time {
            let mip = self.cpu.get_csr(CSRs::mip as u32).unwrap();
            let mip = mip | (1 << MTimerInterrupt as u32);
            self.cpu.set_csr(CSRs::mip as u32, mip).unwrap();
        }

        let time32: u32 = time as u32;
        self.mem.ww(0x200_bff8, time32);
        let time32: u32 = (time >> 32) as u32;
        self.mem.ww(0x200_bffc, time32);
    }

    fn get_interrupt(&mut self) -> Option<Interrupt> {
        use Interrupt::*;
        let mip = self.cpu.get_csr(CSRs::mip as u32).unwrap();
        let msi = mip & (1 << MSoftInterrupt as u32);
        match mip {
            // TODO: Handle interrupts for other privilege modes
            _v if msi != 0 => {
                self.cpu
                    .set_csr(CSRs::mip as u32, mip & !(1 << MSoftInterrupt as u32))
                    .unwrap();
                Some(MSoftInterrupt)
            }
            v if (v & (1 << MTimerInterrupt as u32)) != 0 => Some(MTimerInterrupt),
            v if (v & (1 << MExternalInterrupt as u32)) != 0 => Some(MExternalInterrupt),
            _ => None,
        }
    }

    pub fn tick(&mut self) -> TickResult {
        let pc = self.cpu.pc;
        let word = self.mem.rw(pc);
        let mstatus = self.cpu.get_csr(CSRs::mstatus as u32).unwrap();
        let mstatus_mie = (mstatus & (1 << 3)) != 0;

        let mut interrupt = None;
        if mstatus_mie {
            self.generate_clint_interrupts();
            interrupt = self.get_interrupt();
        }

        if let Some(exc) = interrupt {
            log::debug!("interrupt - mstatus: {mstatus}, exc: {exc:?}");
            self.handle_exception(ExceptionInterrupt::Interrupt(exc))
        } else if self.cpu.wfi {
            TickResult::WFI
        } else if word == 0 {
            self.handle_exception(ExceptionInterrupt::Exception(Exception::IllegalInstruction))
        } else {
            log::debug!("executing - mstatus: {mstatus:b}, pc: {pc}");
            match self.run_instruction(word) {
                Ok(v) => TickResult::Cycles(v),
                Err(err) => self.handle_exception(err),
            }
        }
    }

    // Handles interrupts and exceptions
    fn handle_exception(&mut self, exc: ExceptionInterrupt) -> TickResult {
        let mstatus = self.cpu.get_csr(CSRs::mstatus as u32).unwrap();

        let mstatus_mie = mstatus & (1 << 3);
        let mie = self.cpu.get_csr(CSRs::mie as u32).unwrap();

        let _mcause = match exc {
            ExceptionInterrupt::Interrupt(i) => {
                self.cpu.wfi = false;
                if mstatus_mie != 0 && (mie & (1 << i as u32) != 0) {
                    self.cpu
                        .set_csr(CSRs::mcause as u32, i as u32 | (1 << 31))
                        .unwrap();
                } else {
                    return TickResult::Cycles(4);
                };
                i as u32
            }
            ExceptionInterrupt::Exception(e) => {
                match e {
                    Exception::MEnvironmentCall if self.cpu.x[15] == 255 => {
                        return TickResult::HALT
                    }
                    _ => self.cpu.set_csr(CSRs::mcause as u32, e as u32).unwrap(),
                };
                e as u32
            }
        };

        let mstatus = (mstatus & !(1 << 3)) | mstatus_mie << 4; // move mie to mpie and set mie to 0 (disable interrupts)
        self.cpu.set_csr(CSRs::mstatus as u32, mstatus).unwrap();
        self.cpu.set_csr(CSRs::mepc as u32, self.cpu.pc).unwrap();
        self.cpu.set_csr(CSRs::mtval as u32, self.cpu.pc).unwrap(); // TODO: How do we bubble up suporting data?
                                                                    // TODO: Set privilege mode on msatus.MPP
        let mtvec = self.cpu.get_csr(CSRs::mtvec as u32).unwrap();
        self.cpu.pc = mtvec;
        TickResult::Cycles(4)
    }

    pub fn run_program(&mut self) -> Result<(), ExceptionInterrupt> {
        let mut cycles = 0;
        loop {
            cycles += 1;
            log::debug!("cycle: {cycles}");
            match self.tick() {
                TickResult::Cycles(_v) => wait_cycles(self.speed, 1),
                TickResult::WFI => wait_cycles(self.speed, 1), // TODO: This should actually block on a callback  instead of doing polling
                TickResult::HALT => return Ok(()),
            }
        }
    }

    pub fn dump(&self) -> Vec<u8> {
        use std::mem::transmute;
        let mut dump: Vec<u8> = Vec::new();
        for w in 0..self.cpu.x.len() {
            let bytes: [u8; 4] = unsafe { transmute(self.cpu.x[w]) };
            for b in bytes {
                dump.push(b);
            }
        }
        dump.push(255);
        dump.push(255);
        dump.push(255);
        dump.push(255);

        for idx in 0..self.mem.size() {
            dump.push(self.mem.rb(idx));
        }

        dump.push(255);
        dump.push(255);
        dump.push(255);
        dump.push(255);

        let bytes: [u8; 4] = unsafe { transmute(self.mem.rw(0x200_0000)) };

        dump.append(&mut Vec::from(bytes));

        let bytes: [u8; 4] = unsafe { transmute(self.mem.rw(0x200_4000)) };
        dump.append(&mut Vec::from(bytes));
        let bytes: [u8; 4] = unsafe { transmute(self.mem.rw(0x200_4004)) };
        dump.append(&mut Vec::from(bytes));

        let bytes: [u8; 4] = unsafe { transmute(self.mem.rw(0x200_bff8)) };
        dump.append(&mut Vec::from(bytes));
        let bytes: [u8; 4] = unsafe { transmute(self.mem.rw(0x200_bffc)) };
        dump.append(&mut Vec::from(bytes));

        dump
    }
}

pub struct CPU {
    // program counter
    pub pc: u32,
    // x regisers, ignoring x0
    pub x: [u32; 32],
    // waiting for interrupt
    pub wfi: bool,
    // csr registers
    csr: [u32; 8], // TODO: Implement only the CSRs I want.
}

#[derive(Copy, Clone)]
#[allow(non_camel_case_types)]
#[repr(u32)]
enum CSRs {
    mstatus = 0x300,
    mip = 0x344,
    mie = 0x304,
    mcause = 0x342,
    mtvec = 0x305,
    mtval = 0x343,
    mepc = 0x341,
    mscratch = 0x340,
    // TODO: Supervisor mode
    // sepc = 0
    // sstatus = 0,
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            pc: 0,
            x: [0; 32],
            csr: [0; 8],
            wfi: false,
        }
    }

    fn csr_idx_map(v: u32) -> Result<usize, Exception> {
        let m = match v {
            _ if CSRs::mstatus as u32 == v => 0,
            _ if CSRs::mip as u32 == v => 1,
            _ if CSRs::mie as u32 == v => 2,
            _ if CSRs::mcause as u32 == v => 3,
            _ if CSRs::mtvec as u32 == v => 4,
            _ if CSRs::mtval as u32 == v => 5,
            _ if CSRs::mepc as u32 == v => 6,
            _ if CSRs::mscratch as u32 == v => 7,
            _ => return Err(Exception::IllegalInstruction),
        };
        Ok(m)
    }

    pub fn get_csr(&self, addr: u32) -> Result<u32, Exception> {
        let idx = Self::csr_idx_map(addr)?;
        Ok(self.csr[idx])
    }

    pub fn set_csr(&mut self, addr: u32, v: u32) -> Result<(), Exception> {
        let idx = Self::csr_idx_map(addr)?;
        self.csr[idx] = v;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use macros::mask;
    #[test]
    fn mask_macro_works() {
        let m = mask!(3);
        assert_eq!(m, 0b111);
    }
}
