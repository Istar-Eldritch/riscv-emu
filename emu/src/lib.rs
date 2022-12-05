mod instruction_set;
pub mod memory;
mod terminal;
mod utils;

use instruction_set::{privileged::RVPrivileged, rv32i::RV32i, Instruction};
use memory::{uart::UARTDevice, ClockedMemory, Memory, MMU};
pub use terminal::TermEmulator;

#[derive(Debug, Clone, Copy, PartialEq)]
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
    mem: Box<dyn ClockedMemory>,
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
    pub fn new(speed: u32, terminal: Option<Box<dyn UARTDevice>>) -> Self {
        Emulator {
            cpu: CPU::new(),
            mem: Box::new(MMU::new(terminal)),
            speed,
        }
    }

    pub fn flash(&mut self, mem: Vec<u8>) {
        for i in 0..mem.len() {
            self.mem.wb(i as u32, mem[i]).unwrap();
        }
    }

    fn run_instruction(&mut self, word: u32) -> Result<u32, ExceptionInterrupt> {
        let v = if let Ok(v) = RVPrivileged::try_from(word) {
            log::debug!("instruction: {:?}", v);
            v.execute(&mut self.cpu, self.mem.as_mut_mem())?
        } else if let Ok(v) = RV32i::try_from(word) {
            log::debug!("instruction: {:?}", v);
            v.execute(&mut self.cpu, self.mem.as_mut_mem())?
        } else {
            log::error!("error decoding instruction: {word:b}");
            return Err(ExceptionInterrupt::Exception(Exception::IllegalInstruction));
        };

        self.cpu.pc += 4;
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
            log::debug!("interrupt - mstatus: {mstatus}, exc: {exc:?}, pc: {pc:x}");
            self.handle_exception(ExceptionInterrupt::Interrupt(exc))
        } else if self.cpu.wfi {
            TickResult::WFI
        } else if let Ok(0) = word {
            self.handle_exception(ExceptionInterrupt::Exception(Exception::IllegalInstruction))
        } else if let Ok(addr) = word {
            log::debug!(
                "executing - mstatus: {mstatus:b}, pc: {pc:x}, x1: {:x}",
                self.cpu.get_x(1)
            );
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
                    Exception::MEnvironmentCall if self.cpu.get_x(15) == 255 => {
                        return TickResult::HALT
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

    pub fn run_program(&mut self) -> Result<(), ExceptionInterrupt> {
        let mut cycles = 0;
        let mut pending_work = 0;
        loop {
            if pending_work > 0 {
                cycles += 1;
                pending_work -= 1;
                wait_cycles(self.speed, 1);
            } else {
                match self.tick() {
                    TickResult::Cycles(v) => {
                        log::debug!("cycle: {cycles}");
                        pending_work += v
                    }
                    TickResult::WFI => pending_work += 1, // TODO: This should actually block on a callback  instead of doing polling
                    TickResult::HALT => return Ok(()),
                }
            }
        }
    }

    pub fn dump(&self) -> Vec<u8> {
        use std::mem::transmute;
        let mut dump: Vec<u8> = Vec::new();
        for w in 0..32 {
            let bytes: [u8; 4] = unsafe { transmute(self.cpu.get_x(w)) };
            for b in bytes {
                dump.push(b);
            }
        }
        dump.push(255);
        dump.push(255);
        dump.push(255);
        dump.push(255);

        for idx in 0..0x32000 {
            dump.push(self.mem.rb(idx).unwrap());
        }

        dump.push(255);
        dump.push(255);
        dump.push(255);
        dump.push(255);

        let bytes: [u8; 4] = unsafe { transmute(self.mem.rw(0x200_0000).unwrap()) };

        dump.append(&mut Vec::from(bytes));

        let bytes: [u8; 4] = unsafe { transmute(self.mem.rw(0x200_4000).unwrap()) };
        dump.append(&mut Vec::from(bytes));
        let bytes: [u8; 4] = unsafe { transmute(self.mem.rw(0x200_4004).unwrap()) };
        dump.append(&mut Vec::from(bytes));

        let bytes: [u8; 4] = unsafe { transmute(self.mem.rw(0x200_bff8).unwrap()) };
        dump.append(&mut Vec::from(bytes));
        let bytes: [u8; 4] = unsafe { transmute(self.mem.rw(0x200_bffc).unwrap()) };
        dump.append(&mut Vec::from(bytes));

        dump
    }
}

pub struct CPU {
    // program counter
    pub pc: u32,
    // x regisers, ignoring x0
    x: [u32; 32],
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

    pub fn get_x(&self, idx: u32) -> u32 {
        self.x[idx as usize]
    }

    pub fn set_x(&mut self, idx: u32, val: u32) {
        if idx > 0 {
            self.x[idx as usize] = val;
        }
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
