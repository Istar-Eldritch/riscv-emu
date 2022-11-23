mod format;
mod memory;
mod rv32i;
mod utils;

use memory::Memory;

pub struct Emulator {
    cpu: CPU,
}

impl Emulator {
    fn new() -> Self {
        Emulator { cpu: CPU::new() }
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

pub struct Instruction {
    opcode: u32,
    op: fn(cpu: &mut CPU, &mut Memory, word: u32) -> (),
}

impl Instruction {
    pub fn new(opcode: u32, op: fn(cpu: &mut CPU, &mut Memory, word: u32) -> ()) -> Self {
        Instruction { opcode, op }
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
