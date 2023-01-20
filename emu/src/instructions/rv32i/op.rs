use super::{ExceptionInterrupt, Instruction};
use crate::mcu::MCU;
use riscv_isa_types::rv32i::*;

impl Instruction for AND {
    fn execute(&self, mcu: &mut MCU) -> Result<u32, ExceptionInterrupt> {
        mcu.cpu
            .set_x(self.rd, mcu.cpu.get_x(self.rs1) & mcu.cpu.get_x(self.rs2));
        Ok(1)
    }
}
impl Instruction for OR {
    fn execute(&self, mcu: &mut MCU) -> Result<u32, ExceptionInterrupt> {
        mcu.cpu
            .set_x(self.rd, mcu.cpu.get_x(self.rs1) | mcu.cpu.get_x(self.rs2));
        Ok(1)
    }
}
impl Instruction for SRA {
    fn execute(&self, mcu: &mut MCU) -> Result<u32, ExceptionInterrupt> {
        let shamt = mcu.cpu.get_x(self.rs2) & 0b11111;
        let rs1 = mcu.cpu.get_x(self.rs1);
        mcu.cpu
            .set_x(self.rd, (rs1 as i32).wrapping_shr(shamt) as u32);
        Ok(1)
    }
}
impl Instruction for SRL {
    fn execute(&self, mcu: &mut MCU) -> Result<u32, ExceptionInterrupt> {
        mcu.cpu.set_x(
            self.rd,
            mcu.cpu
                .get_x(self.rs1)
                .wrapping_shr(mcu.cpu.get_x(self.rs2) & 0b11111),
        );
        Ok(1)
    }
}
impl Instruction for XOR {
    fn execute(&self, mcu: &mut MCU) -> Result<u32, ExceptionInterrupt> {
        mcu.cpu
            .set_x(self.rd, mcu.cpu.get_x(self.rs1) ^ mcu.cpu.get_x(self.rs2));
        Ok(1)
    }
}
impl Instruction for SLTU {
    fn execute(&self, mcu: &mut MCU) -> Result<u32, ExceptionInterrupt> {
        let v = if mcu.cpu.get_x(self.rs1) < mcu.cpu.get_x(self.rs2) {
            1
        } else {
            0
        };

        mcu.cpu.set_x(self.rd, v);
        Ok(1)
    }
}
impl Instruction for SLT {
    fn execute(&self, mcu: &mut MCU) -> Result<u32, ExceptionInterrupt> {
        let v = if (mcu.cpu.get_x(self.rs1) as i32) < mcu.cpu.get_x(self.rs2) as i32 {
            1
        } else {
            0
        };

        mcu.cpu.set_x(self.rd, v);
        Ok(1)
    }
}
impl Instruction for SLL {
    fn execute(&self, mcu: &mut MCU) -> Result<u32, ExceptionInterrupt> {
        mcu.cpu.set_x(
            self.rd,
            mcu.cpu
                .get_x(self.rs1)
                .wrapping_shl(mcu.cpu.get_x(self.rs2) & 0b11111),
        );
        Ok(1)
    }
}
impl Instruction for SUB {
    fn execute(&self, mcu: &mut MCU) -> Result<u32, ExceptionInterrupt> {
        mcu.cpu.set_x(
            self.rd,
            mcu.cpu
                .get_x(self.rs1)
                .wrapping_sub(mcu.cpu.get_x(self.rs2)),
        );
        Ok(1)
    }
}
impl Instruction for ADD {
    fn execute(&self, mcu: &mut MCU) -> Result<u32, ExceptionInterrupt> {
        mcu.cpu.set_x(
            self.rd,
            mcu.cpu
                .get_x(self.rs1)
                .wrapping_add(mcu.cpu.get_x(self.rs2)),
        );
        Ok(1)
    }
}

impl Instruction for Op {
    fn execute(&self, mcu: &mut MCU) -> Result<u32, ExceptionInterrupt> {
        match self {
            Op::ADD(i) => i.execute(mcu),
            Op::SUB(i) => i.execute(mcu),
            Op::SLT(i) => i.execute(mcu),
            Op::SLTU(i) => i.execute(mcu),
            Op::XOR(i) => i.execute(mcu),
            Op::OR(i) => i.execute(mcu),
            Op::AND(i) => i.execute(mcu),
            Op::SLL(i) => i.execute(mcu),
            Op::SRL(i) => i.execute(mcu),
            Op::SRA(i) => i.execute(mcu),
        }
    }
}
