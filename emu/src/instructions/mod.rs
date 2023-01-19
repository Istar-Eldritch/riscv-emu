use crate::mcu::MCU;
pub mod privileged;
pub mod rv32i;

pub trait Instruction: TryFrom<u32> + Into<u32> {
    fn execute(&self, mcu: &mut MCU) -> Result<u32, ExceptionInterrupt>;
    fn update_pc(&self, mcu: &mut MCU) -> ();
}

// external
// software
// timer

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
    #[allow(dead_code)]
    InstructionAddressMissaligned = 0,
    InstructionAccessFault = 1,
    IllegalInstruction = 2,
    Breakpoint = 3,
    #[allow(dead_code)]
    LoadAddressMisaligned = 4,
    LoadAccessFault = 5,
    #[allow(dead_code)]
    StoreAddressMisaligned = 6,
    StoreAccessFault = 7,
    #[allow(dead_code)]
    UEnvironmentCall = 8,
    #[allow(dead_code)]
    SEnvironmentCall = 9,
    MEnvironmentCall = 11,
    #[allow(dead_code)]
    InstructionPageFault = 12,
    #[allow(dead_code)]
    LoadPageFault = 13,
    #[allow(dead_code)]
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
