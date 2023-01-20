use super::{Exception, ExceptionInterrupt, Instruction};
use crate::mcu::MCU;
use crate::memory::Memory;
use crate::utils::*;
use riscv_isa_types::format::*;
use riscv_isa_types::rv32i::*;

use Exception::*;
use ExceptionInterrupt::*;

impl Instruction for RV32i {
    fn execute(&self, mcu: &mut MCU) -> Result<u32, ExceptionInterrupt> {
        use RV32i::*;
        match *self {
            LUI(i) => i.execute(mcu),
            AUIPC(i) => i.execute(mcu),
            JAL(f) => jal(mcu, f),
            JALR(f) => jalr(mcu, f),
            BEQ(f) => beq(mcu, f),
            BNE(f) => bne(mcu, f),
            BLT(f) => blt(mcu, f),
            BGE(f) => bge(mcu, f),
            BLTU(f) => bltu(mcu, f),
            BGEU(f) => bgeu(mcu, f),
            LB(f) => lb(mcu, f),
            LH(f) => lh(mcu, f),
            LW(f) => lw(mcu, f),
            LBU(f) => lbu(mcu, f),
            LHU(f) => lhu(mcu, f),
            LWU(f) => lwu(mcu, f),
            SB(f) => sb(mcu, f),
            SH(f) => sh(mcu, f),
            SW(f) => sw(mcu, f),
            ADDI(f) => addi(mcu, f),
            SLTI(f) => slti(mcu, f),
            SLTIU(f) => sltiu(mcu, f),
            XORI(f) => xori(mcu, f),
            ORI(f) => ori(mcu, f),
            ANDI(f) => andi(mcu, f),
            SLLI(f) => slli(mcu, f),
            SRLI(f) => srli(mcu, f),
            SRAI(f) => srai(mcu, f),
            ADD(f) => add(mcu, f),
            SUB(f) => sub(mcu, f),
            SLL(f) => sll(mcu, f),
            SLT(f) => slt(mcu, f),
            SLTU(f) => sltu(mcu, f),
            XOR(f) => xor(mcu, f),
            SRL(f) => srl(mcu, f),
            SRA(f) => sra(mcu, f),
            OR(f) => or(mcu, f),
            AND(f) => and(mcu, f),
            FENCE(f) => fence(mcu, f),
            FENCEI(f) => fence(mcu, f),
            ECALL(f) => ecall(mcu, f),
            EBREAK(f) => ebreak(mcu, f),
            CSRRW(f) => csrrw(mcu, f),
            CSRRS(f) => csrrs(mcu, f),
            CSRRC(f) => csrrc(mcu, f),
            CSRRWI(f) => csrrwi(mcu, f),
            CSRRSI(f) => csrrsi(mcu, f),
            CSRRCI(f) => csrrci(mcu, f),
        }
    }

    fn update_pc(&self, mcu: &mut MCU) {
        use RV32i::*;
        match *self {
            JAL(_) => (),
            JALR(_) => (),
            BEQ(_) => (),
            BNE(_) => (),
            BLT(_) => (),
            BGE(_) => (),
            BLTU(_) => (),
            BGEU(_) => (),
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

fn ecall(_mcu: &mut MCU, _parsed: IFormat) -> Result<u32, ExceptionInterrupt> {
    // TODO: Environemnt calls on other privilege modes
    Err(ExceptionInterrupt::Exception(Exception::MEnvironmentCall))
}

fn ebreak(_mcu: &mut MCU, _parsed: IFormat) -> Result<u32, ExceptionInterrupt> {
    Err(ExceptionInterrupt::Exception(Exception::Breakpoint))
}

/// Jump and Link
fn jal(mcu: &mut MCU, parsed: JFormat) -> Result<u32, ExceptionInterrupt> {
    mcu.cpu.set_x(parsed.rd, mcu.cpu.pc + 4);
    let offset =
        (parsed.imm0 << 12) | (parsed.imm1 << 11) | (parsed.imm2 << 1) | (parsed.imm3 << 20);
    mcu.cpu.pc = (mcu.cpu.pc as i32 + (sext(offset, 20, 32) as i32)) as u32;
    Ok(1)
}

/// Jump and Link Register
fn jalr(mcu: &mut MCU, parsed: IFormat) -> Result<u32, ExceptionInterrupt> {
    let t = mcu.cpu.get_x(parsed.rs1);
    mcu.cpu.set_x(parsed.rd, mcu.cpu.pc + 4);
    mcu.cpu.pc = (((t as i32) + (sext(parsed.imm, 12, 32) as i32)) & !(0b1 as i32)) as u32;
    Ok(1)
}

fn beq(mcu: &mut MCU, parsed: BFormat) -> Result<u32, ExceptionInterrupt> {
    if mcu.cpu.get_x(parsed.rs1) == mcu.cpu.get_x(parsed.rs2) {
        let offset = parsed.imm0 << 11 | parsed.imm1 << 1 | parsed.imm2 << 5 | parsed.imm3 << 12;
        mcu.cpu.pc = (mcu.cpu.pc as i32).wrapping_add(sext(offset, 12, 32) as i32) as u32;
    } else {
        mcu.cpu.pc += 4;
    }
    Ok(1)
}

fn bge(mcu: &mut MCU, parsed: BFormat) -> Result<u32, ExceptionInterrupt> {
    if mcu.cpu.get_x(parsed.rs1) as i32 >= mcu.cpu.get_x(parsed.rs2) as i32 {
        let offset = parsed.imm0 << 11 | parsed.imm1 << 1 | parsed.imm2 << 5 | parsed.imm3 << 12;
        mcu.cpu.pc = (mcu.cpu.pc as i32).wrapping_add(sext(offset, 12, 32) as i32) as u32;
    } else {
        mcu.cpu.pc += 4;
    }
    Ok(1)
}

fn bgeu(mcu: &mut MCU, parsed: BFormat) -> Result<u32, ExceptionInterrupt> {
    if mcu.cpu.get_x(parsed.rs1) >= mcu.cpu.get_x(parsed.rs2) {
        let offset = parsed.imm0 << 11 | parsed.imm1 << 1 | parsed.imm2 << 5 | parsed.imm3 << 12;
        mcu.cpu.pc = (mcu.cpu.pc as i32).wrapping_add(sext(offset, 12, 32) as i32) as u32;
    } else {
        mcu.cpu.pc += 4;
    }
    Ok(1)
}

fn blt(mcu: &mut MCU, parsed: BFormat) -> Result<u32, ExceptionInterrupt> {
    if (mcu.cpu.get_x(parsed.rs1) as i32) < (mcu.cpu.get_x(parsed.rs2) as i32) {
        let offset = parsed.imm0 << 11 | parsed.imm1 << 1 | parsed.imm2 << 5 | parsed.imm3 << 12;
        mcu.cpu.pc = (mcu.cpu.pc as i32).wrapping_add(sext(offset, 12, 32) as i32) as u32;
    } else {
        mcu.cpu.pc += 4;
    }
    Ok(1)
}

fn bltu(mcu: &mut MCU, parsed: BFormat) -> Result<u32, ExceptionInterrupt> {
    if mcu.cpu.get_x(parsed.rs1) < mcu.cpu.get_x(parsed.rs2) {
        let offset = parsed.imm0 << 11 | parsed.imm1 << 1 | parsed.imm2 << 5 | parsed.imm3 << 12;
        mcu.cpu.pc = (mcu.cpu.pc as i32).wrapping_add(sext(offset, 12, 32) as i32) as u32;
    } else {
        mcu.cpu.pc += 4;
    }
    Ok(1)
}

fn bne(mcu: &mut MCU, parsed: BFormat) -> Result<u32, ExceptionInterrupt> {
    if mcu.cpu.get_x(parsed.rs1) != mcu.cpu.get_x(parsed.rs2) {
        let offset = parsed.imm0 << 11 | parsed.imm1 << 1 | parsed.imm2 << 5 | parsed.imm3 << 12;
        mcu.cpu.pc = (mcu.cpu.pc as i32).wrapping_add(sext(offset, 12, 32) as i32) as u32;
    } else {
        mcu.cpu.pc += 4;
    }
    Ok(1)
}

/// Branch parsing

fn lb(mcu: &mut MCU, parsed: IFormat) -> Result<u32, ExceptionInterrupt> {
    let addr =
        (mcu.cpu.get_x(parsed.rs1) as i32).wrapping_add(sext(parsed.imm, 12, 32) as i32) as u32;
    let byte = mcu.mmu.rb(addr).map_err(|_| Exception(LoadAccessFault))?;
    mcu.cpu.set_x(parsed.rd, sext(byte as u32, 8, 32));
    Ok(1)
}

fn lbu(mcu: &mut MCU, parsed: IFormat) -> Result<u32, ExceptionInterrupt> {
    let addr =
        (mcu.cpu.get_x(parsed.rs1) as i32).wrapping_add(sext(parsed.imm, 12, 32) as i32) as u32;
    mcu.cpu.set_x(
        parsed.rd,
        mcu.mmu.rb(addr).map_err(|_| Exception(LoadAccessFault))? as u32,
    );
    Ok(1)
}

fn lh(mcu: &mut MCU, parsed: IFormat) -> Result<u32, ExceptionInterrupt> {
    let addr =
        (mcu.cpu.get_x(parsed.rs1) as i32).wrapping_add(sext(parsed.imm, 12, 32) as i32) as u32;
    mcu.cpu.set_x(
        parsed.rd,
        sext(
            mcu.mmu.rhw(addr).map_err(|_| Exception(LoadAccessFault))? as u32,
            16,
            32,
        ),
    );
    Ok(1)
}

fn lhu(mcu: &mut MCU, parsed: IFormat) -> Result<u32, ExceptionInterrupt> {
    let addr =
        (mcu.cpu.get_x(parsed.rs1) as i32).wrapping_add(sext(parsed.imm, 12, 32) as i32) as u32;
    mcu.cpu.set_x(
        parsed.rd,
        mcu.mmu.rhw(addr).map_err(|_| Exception(LoadAccessFault))? as u32,
    );
    Ok(1)
}

fn lw(mcu: &mut MCU, parsed: IFormat) -> Result<u32, ExceptionInterrupt> {
    let addr = ((mcu.cpu.get_x(parsed.rs1) as i32) + (sext(parsed.imm, 12, 32) as i32)) as u32;
    mcu.cpu.set_x(
        parsed.rd,
        mcu.mmu.rw(addr).map_err(|_| Exception(LoadAccessFault))?,
    );
    Ok(1)
}

fn lwu(mcu: &mut MCU, parsed: IFormat) -> Result<u32, ExceptionInterrupt> {
    let addr =
        (mcu.cpu.get_x(parsed.rs1) as i32).wrapping_add(sext(parsed.imm, 12, 32) as i32) as u32;
    mcu.cpu.set_x(
        parsed.rd,
        mcu.mmu.rw(addr).map_err(|_| Exception(LoadAccessFault))? as u32,
    );
    Ok(1)
}

fn sb(mcu: &mut MCU, parsed: SFormat) -> Result<u32, ExceptionInterrupt> {
    let offset = parsed.imm0 | (parsed.imm1 << 5);
    let addr = (mcu.cpu.get_x(parsed.rs1) as i32).wrapping_add(sext(offset, 12, 32) as i32) as u32;
    mcu.mmu
        .wb(addr, mcu.cpu.get_x(parsed.rs2) as u8)
        .map_err(|_| Exception(StoreAccessFault))?;
    Ok(1)
}

fn sh(mcu: &mut MCU, parsed: SFormat) -> Result<u32, ExceptionInterrupt> {
    let offset = parsed.imm0 | (parsed.imm1 << 5);
    let addr = (mcu.cpu.get_x(parsed.rs1) as i32).wrapping_add(sext(offset, 12, 32) as i32) as u32;
    mcu.mmu
        .whw(addr, mcu.cpu.get_x(parsed.rs2) as u16)
        .map_err(|_| Exception(StoreAccessFault))?;
    Ok(1)
}

fn sw(mcu: &mut MCU, parsed: SFormat) -> Result<u32, ExceptionInterrupt> {
    let offset = parsed.imm0 | (parsed.imm1 << 5);
    let addr = ((mcu.cpu.get_x(parsed.rs1) as i32) + (sext(offset, 12, 32) as i32)) as u32;
    let value = mcu.cpu.get_x(parsed.rs2);
    mcu.mmu
        .ww(addr, value)
        .map_err(|_| Exception(StoreAccessFault))?;
    Ok(1)
}

fn addi(mcu: &mut MCU, parsed: IFormat) -> Result<u32, ExceptionInterrupt> {
    mcu.cpu.set_x(
        parsed.rd,
        (mcu.cpu.get_x(parsed.rs1) as i32).wrapping_add(sext(parsed.imm, 12, 32) as i32) as u32,
    );
    Ok(1)
}

fn slti(mcu: &mut MCU, parsed: IFormat) -> Result<u32, ExceptionInterrupt> {
    let v = if (mcu.cpu.get_x(parsed.rs1) as i32) < sext(parsed.imm, 12, 32) as i32 {
        1
    } else {
        0
    };

    mcu.cpu.set_x(parsed.rd, v);
    Ok(1)
}

fn sltiu(mcu: &mut MCU, parsed: IFormat) -> Result<u32, ExceptionInterrupt> {
    let v = if mcu.cpu.get_x(parsed.rs1) < sext(parsed.imm, 12, 32) {
        1
    } else {
        0
    };

    mcu.cpu.set_x(parsed.rd, v);
    Ok(1)
}

fn xori(mcu: &mut MCU, parsed: IFormat) -> Result<u32, ExceptionInterrupt> {
    mcu.cpu.set_x(
        parsed.rd,
        mcu.cpu.get_x(parsed.rs1) ^ sext(parsed.imm, 12, 32),
    );
    Ok(1)
}

fn ori(mcu: &mut MCU, parsed: IFormat) -> Result<u32, ExceptionInterrupt> {
    mcu.cpu.set_x(
        parsed.rd,
        mcu.cpu.get_x(parsed.rs1) | sext(parsed.imm, 12, 32),
    );
    Ok(1)
}

fn andi(mcu: &mut MCU, parsed: IFormat) -> Result<u32, ExceptionInterrupt> {
    mcu.cpu.set_x(
        parsed.rd,
        mcu.cpu.get_x(parsed.rs1) & sext(parsed.imm, 12, 32),
    );
    Ok(1)
}

fn slli(mcu: &mut MCU, parsed: IFormat) -> Result<u32, ExceptionInterrupt> {
    let shamt = parsed.imm & 0b11111;
    // TODO: shamt[5] should be 0, otherwise is an illegal instruction
    mcu.cpu
        .set_x(parsed.rd, mcu.cpu.get_x(parsed.rs1).wrapping_shl(shamt));
    Ok(1)
}

fn srli(mcu: &mut MCU, parsed: IFormat) -> Result<u32, ExceptionInterrupt> {
    let shamt = parsed.imm & 0b11111;
    // TODO: shamt[5] should be 0, otherwise is an illegal instruction
    mcu.cpu
        .set_x(parsed.rd, mcu.cpu.get_x(parsed.rs1).wrapping_shr(shamt));
    Ok(1)
}

fn srai(mcu: &mut MCU, parsed: IFormat) -> Result<u32, ExceptionInterrupt> {
    let shamt = parsed.imm & 0b11111;
    if shamt & 0b100000 != 0 {
        return Err(ExceptionInterrupt::Exception(Exception::IllegalInstruction));
    }

    let rs1 = mcu.cpu.get_x(parsed.rs1);
    mcu.cpu
        .set_x(parsed.rd, (rs1 as i32).wrapping_shr(shamt) as u32);
    Ok(1)
}

fn add(mcu: &mut MCU, parsed: RFormat) -> Result<u32, ExceptionInterrupt> {
    mcu.cpu.set_x(
        parsed.rd,
        mcu.cpu
            .get_x(parsed.rs1)
            .wrapping_add(mcu.cpu.get_x(parsed.rs2)),
    );
    Ok(1)
}

fn sub(mcu: &mut MCU, parsed: RFormat) -> Result<u32, ExceptionInterrupt> {
    mcu.cpu.set_x(
        parsed.rd,
        mcu.cpu
            .get_x(parsed.rs1)
            .wrapping_sub(mcu.cpu.get_x(parsed.rs2)),
    );
    Ok(1)
}

fn sll(mcu: &mut MCU, parsed: RFormat) -> Result<u32, ExceptionInterrupt> {
    mcu.cpu.set_x(
        parsed.rd,
        mcu.cpu
            .get_x(parsed.rs1)
            .wrapping_shl(mcu.cpu.get_x(parsed.rs2) & 0b11111),
    );
    Ok(1)
}

fn slt(mcu: &mut MCU, parsed: RFormat) -> Result<u32, ExceptionInterrupt> {
    let v = if (mcu.cpu.get_x(parsed.rs1) as i32) < mcu.cpu.get_x(parsed.rs2) as i32 {
        1
    } else {
        0
    };

    mcu.cpu.set_x(parsed.rd, v);
    Ok(1)
}

fn sltu(mcu: &mut MCU, parsed: RFormat) -> Result<u32, ExceptionInterrupt> {
    let v = if mcu.cpu.get_x(parsed.rs1) < mcu.cpu.get_x(parsed.rs2) {
        1
    } else {
        0
    };

    mcu.cpu.set_x(parsed.rd, v);
    Ok(1)
}

fn xor(mcu: &mut MCU, parsed: RFormat) -> Result<u32, ExceptionInterrupt> {
    mcu.cpu.set_x(
        parsed.rd,
        mcu.cpu.get_x(parsed.rs1) ^ mcu.cpu.get_x(parsed.rs2),
    );
    Ok(1)
}

fn srl(mcu: &mut MCU, parsed: RFormat) -> Result<u32, ExceptionInterrupt> {
    mcu.cpu.set_x(
        parsed.rd,
        mcu.cpu
            .get_x(parsed.rs1)
            .wrapping_shr(mcu.cpu.get_x(parsed.rs2) & 0b11111),
    );
    Ok(1)
}

fn sra(mcu: &mut MCU, parsed: RFormat) -> Result<u32, ExceptionInterrupt> {
    let shamt = mcu.cpu.get_x(parsed.rs2) & 0b11111;
    let rs1 = mcu.cpu.get_x(parsed.rs1);
    mcu.cpu
        .set_x(parsed.rd, (rs1 as i32).wrapping_shr(shamt) as u32);
    Ok(1)
}

fn or(mcu: &mut MCU, parsed: RFormat) -> Result<u32, ExceptionInterrupt> {
    mcu.cpu.set_x(
        parsed.rd,
        mcu.cpu.get_x(parsed.rs1) | mcu.cpu.get_x(parsed.rs2),
    );
    Ok(1)
}

fn and(mcu: &mut MCU, parsed: RFormat) -> Result<u32, ExceptionInterrupt> {
    mcu.cpu.set_x(
        parsed.rd,
        mcu.cpu.get_x(parsed.rs1) & mcu.cpu.get_x(parsed.rs2),
    );
    Ok(1)
}

fn fence(_mcu: &mut MCU, _parsed: IFormat) -> Result<u32, ExceptionInterrupt> {
    Ok(0)
}

fn csrrw(mcu: &mut MCU, parsed: IFormat) -> Result<u32, ExceptionInterrupt> {
    let t = match mcu.cpu.get_csr(parsed.imm) {
        Ok(v) => v,
        Err(err) => return Err(ExceptionInterrupt::Exception(err)),
    };
    match mcu.cpu.set_csr(parsed.imm, mcu.cpu.get_x(parsed.rs1)) {
        Err(err) => return Err(ExceptionInterrupt::Exception(err)),
        _ => {}
    };
    mcu.cpu.set_x(parsed.rd, t as u32);
    Ok(1)
}

fn csrrs(mcu: &mut MCU, parsed: IFormat) -> Result<u32, ExceptionInterrupt> {
    let t = match mcu.cpu.get_csr(parsed.imm) {
        Ok(v) => v,
        Err(err) => return Err(ExceptionInterrupt::Exception(err)),
    };
    match mcu.cpu.set_csr(parsed.imm, mcu.cpu.get_x(parsed.rs1) | t) {
        Err(err) => return Err(ExceptionInterrupt::Exception(err)),
        _ => {}
    };
    mcu.cpu.set_x(parsed.rd, t as u32);
    Ok(1)
}

fn csrrc(mcu: &mut MCU, parsed: IFormat) -> Result<u32, ExceptionInterrupt> {
    let t = match mcu.cpu.get_csr(parsed.imm) {
        Ok(v) => v,
        Err(err) => return Err(ExceptionInterrupt::Exception(err)),
    };
    match mcu.cpu.set_csr(parsed.imm, t & !mcu.cpu.get_x(parsed.rs1)) {
        Err(err) => return Err(ExceptionInterrupt::Exception(err)),
        _ => {}
    };
    mcu.cpu.set_x(parsed.rd, t as u32);
    Ok(1)
}

fn csrrwi(mcu: &mut MCU, parsed: IFormat) -> Result<u32, ExceptionInterrupt> {
    mcu.cpu.set_x(
        parsed.rd,
        match mcu.cpu.get_csr(parsed.imm) {
            Ok(v) => v,
            Err(err) => return Err(ExceptionInterrupt::Exception(err)),
        },
    );
    match mcu
        .cpu
        .set_csr(parsed.imm, mcu.cpu.get_x(parsed.rs1) & 0b11111)
    {
        Err(err) => return Err(ExceptionInterrupt::Exception(err)),
        _ => (),
    };
    Ok(1)
}

fn csrrsi(mcu: &mut MCU, parsed: IFormat) -> Result<u32, ExceptionInterrupt> {
    let t = match mcu.cpu.get_csr(parsed.imm) {
        Ok(v) => v,
        Err(err) => {
            return Err(ExceptionInterrupt::Exception(err));
        }
    };
    let new_value = mcu.cpu.get_x(parsed.rs1) | t;
    match mcu.cpu.set_csr(parsed.imm, new_value) {
        Err(err) => {
            return Err(ExceptionInterrupt::Exception(err));
        }
        _ => {}
    };
    mcu.cpu.set_x(parsed.rd, t as u32);
    Ok(1)
}

fn csrrci(mcu: &mut MCU, parsed: IFormat) -> Result<u32, ExceptionInterrupt> {
    let t = match mcu.cpu.get_csr(parsed.imm) {
        Ok(v) => v,
        Err(err) => return Err(ExceptionInterrupt::Exception(err)),
    };
    match mcu
        .cpu
        .set_csr(parsed.imm, t & !(mcu.cpu.get_x(parsed.rs1) & 0b11111))
    {
        Err(err) => return Err(ExceptionInterrupt::Exception(err)),
        _ => {}
    };
    mcu.cpu.set_x(parsed.rd, t as u32);
    Ok(1)
}
