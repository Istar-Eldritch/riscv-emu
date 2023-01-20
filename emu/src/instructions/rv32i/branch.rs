use super::{ExceptionInterrupt, Instruction};
use crate::mcu::MCU;
use crate::utils::*;
use riscv_isa_types::rv32i::*;

impl Instruction for BEQ {
    fn execute(&self, mcu: &mut MCU) -> Result<u32, ExceptionInterrupt> {
        if mcu.cpu.get_x(self.rs1) == mcu.cpu.get_x(self.rs2) {
            mcu.cpu.pc = (mcu.cpu.pc as i32).wrapping_add(sext(self.offset, 12, 32) as i32) as u32;
        } else {
            mcu.cpu.pc += 4;
        }
        Ok(1)
    }
    fn update_pc(&self, _mcu: &mut MCU) {}
}

impl Instruction for BGE {
    fn execute(&self, mcu: &mut MCU) -> Result<u32, ExceptionInterrupt> {
        if mcu.cpu.get_x(self.rs1) as i32 >= mcu.cpu.get_x(self.rs2) as i32 {
            mcu.cpu.pc = (mcu.cpu.pc as i32).wrapping_add(sext(self.offset, 12, 32) as i32) as u32;
        } else {
            mcu.cpu.pc += 4;
        }
        Ok(1)
    }
    fn update_pc(&self, _mcu: &mut MCU) {}
}

impl Instruction for BGEU {
    fn execute(&self, mcu: &mut MCU) -> Result<u32, ExceptionInterrupt> {
        if mcu.cpu.get_x(self.rs1) >= mcu.cpu.get_x(self.rs2) {
            mcu.cpu.pc = (mcu.cpu.pc as i32).wrapping_add(sext(self.offset, 12, 32) as i32) as u32;
        } else {
            mcu.cpu.pc += 4;
        }
        Ok(1)
    }
    fn update_pc(&self, _mcu: &mut MCU) {}
}

impl Instruction for BLT {
    fn execute(&self, mcu: &mut MCU) -> Result<u32, ExceptionInterrupt> {
        if (mcu.cpu.get_x(self.rs1) as i32) < (mcu.cpu.get_x(self.rs2) as i32) {
            mcu.cpu.pc = (mcu.cpu.pc as i32).wrapping_add(sext(self.offset, 12, 32) as i32) as u32;
        } else {
            mcu.cpu.pc += 4;
        }
        Ok(1)
    }
    fn update_pc(&self, _mcu: &mut MCU) {}
}

impl Instruction for BLTU {
    fn execute(&self, mcu: &mut MCU) -> Result<u32, ExceptionInterrupt> {
        if mcu.cpu.get_x(self.rs1) < mcu.cpu.get_x(self.rs2) {
            mcu.cpu.pc = (mcu.cpu.pc as i32).wrapping_add(sext(self.offset, 12, 32) as i32) as u32;
        } else {
            mcu.cpu.pc += 4;
        }
        Ok(1)
    }
    fn update_pc(&self, _mcu: &mut MCU) {}
}

impl Instruction for BNE {
    fn execute(&self, mcu: &mut MCU) -> Result<u32, ExceptionInterrupt> {
        if mcu.cpu.get_x(self.rs1) != mcu.cpu.get_x(self.rs2) {
            mcu.cpu.pc = (mcu.cpu.pc as i32).wrapping_add(sext(self.offset, 12, 32) as i32) as u32;
        } else {
            mcu.cpu.pc += 4;
        }
        Ok(1)
    }
    fn update_pc(&self, _mcu: &mut MCU) {}
}

impl Instruction for Branch {
    fn execute(&self, mcu: &mut MCU) -> Result<u32, ExceptionInterrupt> {
        match self {
            Branch::BEQ(i) => i.execute(mcu),
            Branch::BGE(i) => i.execute(mcu),
            Branch::BGEU(i) => i.execute(mcu),
            Branch::BLT(i) => i.execute(mcu),
            Branch::BLTU(i) => i.execute(mcu),
            Branch::BNE(i) => i.execute(mcu),
        }
    }
    fn update_pc(&self, _mcu: &mut MCU) {}
}
