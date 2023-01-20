use super::{Exception, ExceptionInterrupt, Instruction};
use crate::mcu::MCU;
use crate::utils::*;
use riscv_isa_types::rv32i::*;

impl Instruction for SRAI {
    fn execute(&self, mcu: &mut MCU) -> Result<u32, ExceptionInterrupt> {
        let shamt = self.imm & 0b11111;
        if shamt & 0b100000 != 0 {
            return Err(ExceptionInterrupt::Exception(Exception::IllegalInstruction));
        }

        let rs1 = mcu.cpu.get_x(self.rs1);
        mcu.cpu
            .set_x(self.rd, (rs1 as i32).wrapping_shr(shamt) as u32);
        Ok(1)
    }
}

impl Instruction for SRLI {
    fn execute(&self, mcu: &mut MCU) -> Result<u32, ExceptionInterrupt> {
        let shamt = self.imm & 0b11111;
        // TODO: shamt[5] should be 0, otherwise is an illegal instruction
        mcu.cpu
            .set_x(self.rd, mcu.cpu.get_x(self.rs1).wrapping_shr(shamt));
        Ok(1)
    }
}

impl Instruction for SLLI {
    fn execute(&self, mcu: &mut MCU) -> Result<u32, ExceptionInterrupt> {
        let shamt = self.imm & 0b11111;
        // TODO: shamt[5] should be 0, otherwise is an illegal instruction
        mcu.cpu
            .set_x(self.rd, mcu.cpu.get_x(self.rs1).wrapping_shl(shamt));
        Ok(1)
    }
}

impl Instruction for ANDI {
    fn execute(&self, mcu: &mut MCU) -> Result<u32, ExceptionInterrupt> {
        mcu.cpu
            .set_x(self.rd, mcu.cpu.get_x(self.rs1) & sext(self.imm, 12, 32));
        Ok(1)
    }
}

impl Instruction for ORI {
    fn execute(&self, mcu: &mut MCU) -> Result<u32, ExceptionInterrupt> {
        mcu.cpu
            .set_x(self.rd, mcu.cpu.get_x(self.rs1) | sext(self.imm, 12, 32));
        Ok(1)
    }
}

impl Instruction for XORI {
    fn execute(&self, mcu: &mut MCU) -> Result<u32, ExceptionInterrupt> {
        mcu.cpu
            .set_x(self.rd, mcu.cpu.get_x(self.rs1) ^ sext(self.imm, 12, 32));
        Ok(1)
    }
}

impl Instruction for SLTIU {
    fn execute(&self, mcu: &mut MCU) -> Result<u32, ExceptionInterrupt> {
        let v = if mcu.cpu.get_x(self.rs1) < sext(self.imm, 12, 32) {
            1
        } else {
            0
        };

        mcu.cpu.set_x(self.rd, v);
        Ok(1)
    }
}

impl Instruction for SLTI {
    fn execute(&self, mcu: &mut MCU) -> Result<u32, ExceptionInterrupt> {
        let v = if (mcu.cpu.get_x(self.rs1) as i32) < sext(self.imm, 12, 32) as i32 {
            1
        } else {
            0
        };

        mcu.cpu.set_x(self.rd, v);
        Ok(1)
    }
}

impl Instruction for ADDI {
    fn execute(&self, mcu: &mut MCU) -> Result<u32, ExceptionInterrupt> {
        mcu.cpu.set_x(
            self.rd,
            (mcu.cpu.get_x(self.rs1) as i32).wrapping_add(sext(self.imm, 12, 32) as i32) as u32,
        );
        Ok(1)
    }
}

impl Instruction for Immediate {
    fn execute(&self, mcu: &mut MCU) -> Result<u32, ExceptionInterrupt> {
        match self {
            Immediate::ADDI(i) => i.execute(mcu),
            Immediate::SLTI(i) => i.execute(mcu),
            Immediate::SLTIU(i) => i.execute(mcu),
            Immediate::XORI(i) => i.execute(mcu),
            Immediate::ORI(i) => i.execute(mcu),
            Immediate::ANDI(i) => i.execute(mcu),
            Immediate::SLLI(i) => i.execute(mcu),
            Immediate::SRLI(i) => i.execute(mcu),
            Immediate::SRAI(i) => i.execute(mcu),
        }
    }
}
