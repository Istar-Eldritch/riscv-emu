use crate::{ExceptionInterrupt, Memory, CPU};
pub mod privileged;
pub mod rv32i;

pub trait Instruction: TryFrom<u32> + Into<u32> {
    fn execute(&self, cpu: &mut CPU, mem: &mut dyn Memory) -> Result<u32, ExceptionInterrupt>;
    fn update_pc(&self, cpu: &mut CPU) -> ();
}
