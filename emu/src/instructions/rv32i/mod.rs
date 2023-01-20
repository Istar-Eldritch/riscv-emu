mod branch;
mod immediate;
mod load;
mod miscmem;
mod op;
mod store;
mod system;

use super::{Exception, ExceptionInterrupt, Instruction};
use crate::mcu::MCU;
use crate::utils::*;
use riscv_isa_types::rv32i::*;

use ExceptionInterrupt::*;

impl Instruction for RV32i {
    fn execute(&self, mcu: &mut MCU) -> Result<u32, ExceptionInterrupt> {
        use RV32i::*;
        match *self {
            LUI(i) => i.execute(mcu),
            AUIPC(i) => i.execute(mcu),
            JAL(i) => i.execute(mcu),
            JALR(i) => i.execute(mcu),
            Branch(i) => i.execute(mcu),
            Load(i) => i.execute(mcu),
            Store(i) => i.execute(mcu),
            Immediate(i) => i.execute(mcu),
            Op(i) => i.execute(mcu),
            MiscMem(i) => i.execute(mcu),
            System(i) => i.execute(mcu),
        }
    }

    fn update_pc(&self, mcu: &mut MCU) {
        use RV32i::*;
        match *self {
            JAL(_) => (),
            JALR(_) => (),
            Branch(_) => (),
            _ => mcu.cpu.pc += 4,
        }
    }
}

impl Instruction for LUI {
    fn execute(&self, mcu: &mut MCU) -> Result<u32, ExceptionInterrupt> {
        mcu.cpu.set_x(self.rd, sext(self.imm << 12, 32, 32));
        Ok(1)
    }
    fn update_pc(&self, mcu: &mut MCU) {
        mcu.cpu.pc += 4;
    }
}

impl Instruction for AUIPC {
    fn execute(&self, mcu: &mut MCU) -> Result<u32, ExceptionInterrupt> {
        mcu.cpu.set_x(
            self.rd,
            ((mcu.cpu.pc as i32) + (sext(self.imm << 12, 32, 32)) as i32) as u32,
        );
        Ok(1)
    }
    fn update_pc(&self, mcu: &mut MCU) {
        mcu.cpu.pc += 4;
    }
}

impl Instruction for JAL {
    fn execute(&self, mcu: &mut MCU) -> Result<u32, ExceptionInterrupt> {
        mcu.cpu.set_x(self.rd, mcu.cpu.pc + 4);
        mcu.cpu.pc = (mcu.cpu.pc as i32 + (sext(self.offset, 20, 32) as i32)) as u32;
        Ok(1)
    }
    fn update_pc(&self, _mcu: &mut MCU) {}
}

impl Instruction for JALR {
    fn execute(&self, mcu: &mut MCU) -> Result<u32, ExceptionInterrupt> {
        let t = mcu.cpu.get_x(self.rs1);
        mcu.cpu.set_x(self.rd, mcu.cpu.pc + 4);
        mcu.cpu.pc = (((t as i32) + (sext(self.imm, 12, 32) as i32)) & !(0b1 as i32)) as u32;
        Ok(1)
    }
    fn update_pc(&self, _mcu: &mut MCU) {}
}
