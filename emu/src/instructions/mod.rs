use crate::{memory::Memory, CPU};
pub mod privileged;
pub mod rv32i;

pub trait Instruction: TryFrom<u32> + Into<u32> {
    fn execute(&self, cpu: &mut CPU, mem: &mut dyn Memory) -> Result<u32, ExceptionInterrupt>;
    fn update_pc(&self, cpu: &mut CPU) -> ();
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Interrupt {
    #[allow(dead_code)]
    SSoftInterrupt = 1,
    MSoftInterrupt = 3,
    #[allow(dead_code)]
    STimerInterrupt = 5,
    MTimerInterrupt = 7,
    #[allow(dead_code)]
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
