use super::{Exception, ExceptionInterrupt, Instruction};
use crate::mcu::MCU;
use crate::memory::Memory;
use crate::utils::*;
use riscv_isa_types::rv32i::*;
use Exception::*;

impl Instruction for SB {
    fn execute(&self, mcu: &mut MCU) -> Result<u32, ExceptionInterrupt> {
        let addr =
            (mcu.cpu.get_x(self.rs1) as i32).wrapping_add(sext(self.offset, 12, 32) as i32) as u32;
        mcu.mmu
            .wb(addr, mcu.cpu.get_x(self.rs2) as u8)
            .map_err(|_| Exception(StoreAccessFault))?;
        Ok(1)
    }
}

impl Instruction for SH {
    fn execute(&self, mcu: &mut MCU) -> Result<u32, ExceptionInterrupt> {
        let addr =
            (mcu.cpu.get_x(self.rs1) as i32).wrapping_add(sext(self.offset, 12, 32) as i32) as u32;
        mcu.mmu
            .whw(addr, mcu.cpu.get_x(self.rs2) as u16)
            .map_err(|_| Exception(StoreAccessFault))?;
        Ok(1)
    }
}

impl Instruction for SW {
    fn execute(&self, mcu: &mut MCU) -> Result<u32, ExceptionInterrupt> {
        let addr = ((mcu.cpu.get_x(self.rs1) as i32) + (sext(self.offset, 12, 32) as i32)) as u32;
        let value = mcu.cpu.get_x(self.rs2);
        mcu.mmu
            .ww(addr, value)
            .map_err(|_| Exception(StoreAccessFault))?;
        Ok(1)
    }
}

impl Instruction for Store {
    fn execute(&self, mcu: &mut MCU) -> Result<u32, ExceptionInterrupt> {
        match self {
            Store::SB(i) => i.execute(mcu),
            Store::SH(i) => i.execute(mcu),
            Store::SW(i) => i.execute(mcu),
        }
    }
    fn update_pc(&self, mcu: &mut MCU) {
        match self {
            Store::SB(i) => i.update_pc(mcu),
            Store::SH(i) => i.update_pc(mcu),
            Store::SW(i) => i.update_pc(mcu),
        }
    }
}
