mod format;
mod memory;
mod rv32i;
mod utils;

use memory::Memory;
use rv32i::rv32i;

#[derive(Debug)]
pub enum Interrupts {
    SSoftInterrupt = 1,
    MSoftInterrupt = 3,
    STimerInterrupt = 5,
    MTimerInterrupt = 7,
    SExternalInterrupt = 9,
    MExternalInterrupt = 11,
}

#[derive(Debug)]
pub enum CPUException {
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

impl std::fmt::Display for CPUException {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        fmt.write_fmt(format_args!("{:?}", self))
    }
}

impl std::error::Error for CPUException {}

pub struct Emulator {
    cpu: CPU,
    mem: Memory,
}

impl Emulator {
    pub fn new(mem_capacity: u32) -> Self {
        Emulator {
            cpu: CPU::new(),
            mem: Memory::new(mem_capacity),
        }
    }

    pub fn flash(&mut self, mem: Vec<u8>) {
        for i in 0..mem.len() {
            self.mem.wb(i as u32, mem[i]);
        }
    }

    fn run_instruction(&mut self, word: u32) -> Result<(), CPUException> {
        rv32i(&mut self.cpu, &mut self.mem, word)?;
        self.cpu.pc += 4;
        Ok(())
    }

    pub fn run_program(&mut self) -> Result<(), CPUException> {
        loop {
            let pc = self.cpu.pc;
            let word = self.mem.rw(pc);
            if word == 0 {
                break;
            }
            self.run_instruction(word)?;
        }
        Ok(())
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
        for idx in 0..self.mem.size() {
            dump.push(self.mem.rb(idx));
        }
        dump
    }
}

pub struct CPU {
    // program counter
    pub pc: u32,
    // x regisers, ignoring x0
    pub x: [u32; 32],
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
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            pc: 0,
            x: [0; 32],
            csr: [0; 8],
        }
    }

    fn csr_idx_map(v: u32) -> Result<usize, CPUException> {
        let m = match v {
            _ if CSRs::mstatus as u32 == v => 0,
            _ if CSRs::mip as u32 == v => 1,
            _ if CSRs::mie as u32 == v => 2,
            _ if CSRs::mcause as u32 == v => 3,
            _ if CSRs::mtvec as u32 == v => 4,
            _ if CSRs::mtval as u32 == v => 5,
            _ if CSRs::mepc as u32 == v => 6,
            _ if CSRs::mscratch as u32 == v => 7,
            _ => return Err(CPUException::IllegalInstruction),
        };
        Ok(m)
    }

    pub fn get_csr(&self, addr: u32) -> Result<u32, CPUException> {
        let idx = Self::csr_idx_map(addr)?;
        Ok(self.csr[idx])
    }

    pub fn set_csr(&mut self, addr: u32, v: u32) -> Result<(), CPUException> {
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
