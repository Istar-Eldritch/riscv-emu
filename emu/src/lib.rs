mod format;
mod memory;
mod rv32i;
mod utils;

use memory::Memory;
use rv32i::rv32i;

#[derive(Debug)]
pub enum CPUException {
    UnrecognizedInstruction,
    EnvironmentCall,
    Breakpoint,
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
    pub csr: [u32; 0x0b3], // TODO: Implement only the CSRs I want.
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            pc: 0,
            x: [0; 32],
            csr: [0; 0x0b3],
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
