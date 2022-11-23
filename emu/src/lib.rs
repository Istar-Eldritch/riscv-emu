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
        rv32i(&mut self.cpu, &mut self.mem, word)
    }

    pub fn run_program(&mut self) -> Result<(), CPUException> {
        loop {
            let pc = self.cpu.pc;
            let word = self.mem.rw(pc);
            self.run_instruction(word)?;
        }
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
