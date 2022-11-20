mod format;
mod utils;

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
    pc: u32,
    // x regisers, ignoring x0
    registers: [u32; 32],
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            pc: 0,
            registers: [0; 32],
        }
    }
}

pub struct Memory();

struct Instruction<F> {
    name: &'static str,
    operation: fn(cpu: &mut CPU, &mut Memory, word: F) -> (),
}
