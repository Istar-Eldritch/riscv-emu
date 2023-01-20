use super::{ExceptionInterrupt, Instruction};
use crate::mcu::MCU;
use riscv_isa_types::rv32i::*;

impl Instruction for FENCE {
    fn execute(&self, _mcu: &mut MCU) -> Result<u32, ExceptionInterrupt> {
        Ok(0)
    }
}

impl Instruction for FENCEI {
    fn execute(&self, _mcu: &mut MCU) -> Result<u32, ExceptionInterrupt> {
        Ok(0)
    }
}

impl Instruction for MiscMem {
    fn execute(&self, mcu: &mut MCU) -> Result<u32, ExceptionInterrupt> {
        match self {
            MiscMem::FENCE(i) => i.execute(mcu),
            MiscMem::FENCEI(i) => i.execute(mcu),
        }
    }
}
