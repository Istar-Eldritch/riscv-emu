use crate::{ExceptionInterrupt, Memory, CPU};
pub mod format;
pub mod privileged;
pub mod rv32i;

pub trait Instruction {
    fn execute(&self, cpu: &mut CPU, mem: &mut dyn Memory) -> Result<u32, ExceptionInterrupt>;
}