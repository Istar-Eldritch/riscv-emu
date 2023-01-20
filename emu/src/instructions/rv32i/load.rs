use super::{Exception, ExceptionInterrupt, Instruction};
use crate::mcu::MCU;
use crate::memory::Memory;
use crate::utils::*;
use riscv_isa_types::rv32i::*;

impl Instruction for LB {
    fn execute(&self, mcu: &mut MCU) -> Result<u32, ExceptionInterrupt> {
        let addr =
            (mcu.cpu.get_x(self.rs1) as i32).wrapping_add(sext(self.imm, 12, 32) as i32) as u32;
        let byte = mcu
            .mmu
            .rb(addr)
            .map_err(|_| Exception(Exception::LoadAccessFault))?;
        mcu.cpu.set_x(self.rd, sext(byte as u32, 8, 32));
        Ok(1)
    }
    fn update_pc(&self, mcu: &mut MCU) {
        mcu.cpu.pc += 4;
    }
}

impl Instruction for LBU {
    fn execute(&self, mcu: &mut MCU) -> Result<u32, ExceptionInterrupt> {
        let addr =
            (mcu.cpu.get_x(self.rs1) as i32).wrapping_add(sext(self.imm, 12, 32) as i32) as u32;
        mcu.cpu.set_x(
            self.rd,
            mcu.mmu
                .rb(addr)
                .map_err(|_| Exception(Exception::LoadAccessFault))? as u32,
        );
        Ok(1)
    }
    fn update_pc(&self, mcu: &mut MCU) {
        mcu.cpu.pc += 4;
    }
}

impl Instruction for LH {
    fn execute(&self, mcu: &mut MCU) -> Result<u32, ExceptionInterrupt> {
        let addr =
            (mcu.cpu.get_x(self.rs1) as i32).wrapping_add(sext(self.imm, 12, 32) as i32) as u32;
        mcu.cpu.set_x(
            self.rd,
            sext(
                mcu.mmu
                    .rhw(addr)
                    .map_err(|_| Exception(Exception::LoadAccessFault))? as u32,
                16,
                32,
            ),
        );
        Ok(1)
    }
    fn update_pc(&self, mcu: &mut MCU) {
        mcu.cpu.pc += 4;
    }
}

impl Instruction for LHU {
    fn execute(&self, mcu: &mut MCU) -> Result<u32, ExceptionInterrupt> {
        let addr =
            (mcu.cpu.get_x(self.rs1) as i32).wrapping_add(sext(self.imm, 12, 32) as i32) as u32;
        mcu.cpu.set_x(
            self.rd,
            mcu.mmu
                .rhw(addr)
                .map_err(|_| Exception(Exception::LoadAccessFault))? as u32,
        );
        Ok(1)
    }
    fn update_pc(&self, mcu: &mut MCU) {
        mcu.cpu.pc += 4;
    }
}

impl Instruction for LW {
    fn execute(&self, mcu: &mut MCU) -> Result<u32, ExceptionInterrupt> {
        let addr = ((mcu.cpu.get_x(self.rs1) as i32) + (sext(self.imm, 12, 32) as i32)) as u32;
        mcu.cpu.set_x(
            self.rd,
            mcu.mmu
                .rw(addr)
                .map_err(|_| Exception(Exception::LoadAccessFault))?,
        );
        Ok(1)
    }
    fn update_pc(&self, mcu: &mut MCU) {
        mcu.cpu.pc += 4;
    }
}

impl Instruction for LWU {
    fn execute(&self, mcu: &mut MCU) -> Result<u32, ExceptionInterrupt> {
        let addr =
            (mcu.cpu.get_x(self.rs1) as i32).wrapping_add(sext(self.imm, 12, 32) as i32) as u32;
        mcu.cpu.set_x(
            self.rd,
            mcu.mmu
                .rw(addr)
                .map_err(|_| Exception(Exception::LoadAccessFault))? as u32,
        );
        Ok(1)
    }
    fn update_pc(&self, mcu: &mut MCU) {
        mcu.cpu.pc += 4;
    }
}

impl Instruction for Load {
    fn execute(&self, mcu: &mut MCU) -> Result<u32, ExceptionInterrupt> {
        match self {
            Load::LB(i) => i.execute(mcu),
            Load::LBU(i) => i.execute(mcu),
            Load::LH(i) => i.execute(mcu),
            Load::LHU(i) => i.execute(mcu),
            Load::LW(i) => i.execute(mcu),
            Load::LWU(i) => i.execute(mcu),
        }
    }
    fn update_pc(&self, mcu: &mut MCU) {
        match self {
            Load::LB(i) => i.update_pc(mcu),
            Load::LBU(i) => i.update_pc(mcu),
            Load::LH(i) => i.update_pc(mcu),
            Load::LHU(i) => i.update_pc(mcu),
            Load::LW(i) => i.update_pc(mcu),
            Load::LWU(i) => i.update_pc(mcu),
        }
    }
}
