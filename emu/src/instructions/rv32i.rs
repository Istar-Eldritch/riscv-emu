use super::{Exception, ExceptionInterrupt, Instruction};
use crate::cpu::CPU;
use crate::memory::Memory;
use crate::utils::*;
use riscv_isa_types::format::*;
use riscv_isa_types::rv32i::RV32i;

use Exception::*;
use ExceptionInterrupt::*;

impl Instruction for RV32i {
    fn execute(&self, cpu: &mut CPU, mem: &mut dyn Memory) -> Result<u32, ExceptionInterrupt> {
        use RV32i::*;
        match *self {
            LUI(f) => lui(cpu, mem, f),
            AUIPC(f) => auipc(cpu, mem, f),
            JAL(f) => jal(cpu, mem, f),
            JALR(f) => jalr(cpu, mem, f),
            BEQ(f) => beq(cpu, mem, f),
            BNE(f) => bne(cpu, mem, f),
            BLT(f) => blt(cpu, mem, f),
            BGE(f) => bge(cpu, mem, f),
            BLTU(f) => bltu(cpu, mem, f),
            BGEU(f) => bgeu(cpu, mem, f),
            LB(f) => lb(cpu, mem, f),
            LH(f) => lh(cpu, mem, f),
            LW(f) => lw(cpu, mem, f),
            LBU(f) => lbu(cpu, mem, f),
            LHU(f) => lhu(cpu, mem, f),
            LWU(f) => lwu(cpu, mem, f),
            SB(f) => sb(cpu, mem, f),
            SH(f) => sh(cpu, mem, f),
            SW(f) => sw(cpu, mem, f),
            ADDI(f) => addi(cpu, mem, f),
            SLTI(f) => slti(cpu, mem, f),
            SLTIU(f) => sltiu(cpu, mem, f),
            XORI(f) => xori(cpu, mem, f),
            ORI(f) => ori(cpu, mem, f),
            ANDI(f) => andi(cpu, mem, f),
            SLLI(f) => slli(cpu, mem, f),
            SRLI(f) => srli(cpu, mem, f),
            SRAI(f) => srai(cpu, mem, f),
            ADD(f) => add(cpu, mem, f),
            SUB(f) => sub(cpu, mem, f),
            SLL(f) => sll(cpu, mem, f),
            SLT(f) => slt(cpu, mem, f),
            SLTU(f) => sltu(cpu, mem, f),
            XOR(f) => xor(cpu, mem, f),
            SRL(f) => srl(cpu, mem, f),
            SRA(f) => sra(cpu, mem, f),
            OR(f) => or(cpu, mem, f),
            AND(f) => and(cpu, mem, f),
            FENCE(f) => fence(cpu, mem, f),
            FENCEI(f) => fence(cpu, mem, f),
            ECALL(f) => ecall(cpu, mem, f),
            EBREAK(f) => ebreak(cpu, mem, f),
            CSRRW(f) => csrrw(cpu, mem, f),
            CSRRS(f) => csrrs(cpu, mem, f),
            CSRRC(f) => csrrc(cpu, mem, f),
            CSRRWI(f) => csrrwi(cpu, mem, f),
            CSRRSI(f) => csrrsi(cpu, mem, f),
            CSRRCI(f) => csrrci(cpu, mem, f),
        }
    }

    fn update_pc(&self, cpu: &mut CPU) {
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
            _ => cpu.pc += 4,
        }
    }
}

fn ecall(
    _cpu: &mut CPU,
    _mem: &mut dyn Memory,
    _parsed: IFormat,
) -> Result<u32, ExceptionInterrupt> {
    // TODO: Environemnt calls on other privilege modes
    Err(ExceptionInterrupt::Exception(Exception::MEnvironmentCall))
}

fn ebreak(
    _cpu: &mut CPU,
    _mem: &mut dyn Memory,
    _parsed: IFormat,
) -> Result<u32, ExceptionInterrupt> {
    Err(ExceptionInterrupt::Exception(Exception::Breakpoint))
}

/// Load Upper Immediate
fn lui(cpu: &mut CPU, _mem: &mut dyn Memory, parsed: UFormat) -> Result<u32, ExceptionInterrupt> {
    cpu.set_x(parsed.rd, sext(parsed.imm << 12, 32, 32));
    Ok(1)
}

/// Add Upper Immediate to PC
fn auipc(cpu: &mut CPU, _mem: &mut dyn Memory, parsed: UFormat) -> Result<u32, ExceptionInterrupt> {
    cpu.set_x(
        parsed.rd,
        ((cpu.pc as i32) + (sext(parsed.imm << 12, 32, 32)) as i32) as u32,
    );
    Ok(1)
}

/// Jump and Link
fn jal(cpu: &mut CPU, _mem: &mut dyn Memory, parsed: JFormat) -> Result<u32, ExceptionInterrupt> {
    cpu.set_x(parsed.rd, cpu.pc + 4);
    let offset =
        (parsed.imm0 << 12) | (parsed.imm1 << 11) | (parsed.imm2 << 1) | (parsed.imm3 << 20);
    cpu.pc = (cpu.pc as i32 + (sext(offset, 20, 32) as i32)) as u32;
    Ok(1)
}

/// Jump and Link Register
fn jalr(cpu: &mut CPU, _mem: &mut dyn Memory, parsed: IFormat) -> Result<u32, ExceptionInterrupt> {
    let t = cpu.get_x(parsed.rs1);
    cpu.set_x(parsed.rd, cpu.pc + 4);
    cpu.pc = (((t as i32) + (sext(parsed.imm, 12, 32) as i32)) & !(0b1 as i32)) as u32;
    Ok(1)
}

fn beq(cpu: &mut CPU, _mem: &mut dyn Memory, parsed: BFormat) -> Result<u32, ExceptionInterrupt> {
    if cpu.get_x(parsed.rs1) == cpu.get_x(parsed.rs2) {
        let offset = parsed.imm0 << 11 | parsed.imm1 << 1 | parsed.imm2 << 5 | parsed.imm3 << 12;
        cpu.pc = (cpu.pc as i32).wrapping_add(sext(offset, 12, 32) as i32) as u32;
    } else {
        cpu.pc += 4;
    }
    Ok(1)
}

fn bge(cpu: &mut CPU, _mem: &mut dyn Memory, parsed: BFormat) -> Result<u32, ExceptionInterrupt> {
    if cpu.get_x(parsed.rs1) as i32 >= cpu.get_x(parsed.rs2) as i32 {
        let offset = parsed.imm0 << 11 | parsed.imm1 << 1 | parsed.imm2 << 5 | parsed.imm3 << 12;
        cpu.pc = (cpu.pc as i32).wrapping_add(sext(offset, 12, 32) as i32) as u32;
    } else {
        cpu.pc += 4;
    }
    Ok(1)
}

fn bgeu(cpu: &mut CPU, _mem: &mut dyn Memory, parsed: BFormat) -> Result<u32, ExceptionInterrupt> {
    if cpu.get_x(parsed.rs1) >= cpu.get_x(parsed.rs2) {
        let offset = parsed.imm0 << 11 | parsed.imm1 << 1 | parsed.imm2 << 5 | parsed.imm3 << 12;
        cpu.pc = (cpu.pc as i32).wrapping_add(sext(offset, 12, 32) as i32) as u32;
    } else {
        cpu.pc += 4;
    }
    Ok(1)
}

fn blt(cpu: &mut CPU, _mem: &mut dyn Memory, parsed: BFormat) -> Result<u32, ExceptionInterrupt> {
    if (cpu.get_x(parsed.rs1) as i32) < (cpu.get_x(parsed.rs2) as i32) {
        let offset = parsed.imm0 << 11 | parsed.imm1 << 1 | parsed.imm2 << 5 | parsed.imm3 << 12;
        cpu.pc = (cpu.pc as i32).wrapping_add(sext(offset, 12, 32) as i32) as u32;
    } else {
        cpu.pc += 4;
    }
    Ok(1)
}

fn bltu(cpu: &mut CPU, _mem: &mut dyn Memory, parsed: BFormat) -> Result<u32, ExceptionInterrupt> {
    if cpu.get_x(parsed.rs1) < cpu.get_x(parsed.rs2) {
        let offset = parsed.imm0 << 11 | parsed.imm1 << 1 | parsed.imm2 << 5 | parsed.imm3 << 12;
        cpu.pc = (cpu.pc as i32).wrapping_add(sext(offset, 12, 32) as i32) as u32;
    } else {
        cpu.pc += 4;
    }
    Ok(1)
}

fn bne(cpu: &mut CPU, _mem: &mut dyn Memory, parsed: BFormat) -> Result<u32, ExceptionInterrupt> {
    if cpu.get_x(parsed.rs1) != cpu.get_x(parsed.rs2) {
        let offset = parsed.imm0 << 11 | parsed.imm1 << 1 | parsed.imm2 << 5 | parsed.imm3 << 12;
        cpu.pc = (cpu.pc as i32).wrapping_add(sext(offset, 12, 32) as i32) as u32;
    } else {
        cpu.pc += 4;
    }
    Ok(1)
}

/// Branch parsing

fn lb(cpu: &mut CPU, mem: &mut dyn Memory, parsed: IFormat) -> Result<u32, ExceptionInterrupt> {
    let addr = (cpu.get_x(parsed.rs1) as i32).wrapping_add(sext(parsed.imm, 12, 32) as i32) as u32;
    let byte = mem.rb(addr).map_err(|_| Exception(LoadAccessFault))?;
    cpu.set_x(parsed.rd, sext(byte as u32, 8, 32));
    Ok(1)
}

fn lbu(cpu: &mut CPU, mem: &mut dyn Memory, parsed: IFormat) -> Result<u32, ExceptionInterrupt> {
    let addr = (cpu.get_x(parsed.rs1) as i32).wrapping_add(sext(parsed.imm, 12, 32) as i32) as u32;
    cpu.set_x(
        parsed.rd,
        mem.rb(addr).map_err(|_| Exception(LoadAccessFault))? as u32,
    );
    Ok(1)
}

fn lh(cpu: &mut CPU, mem: &mut dyn Memory, parsed: IFormat) -> Result<u32, ExceptionInterrupt> {
    let addr = (cpu.get_x(parsed.rs1) as i32).wrapping_add(sext(parsed.imm, 12, 32) as i32) as u32;
    cpu.set_x(
        parsed.rd,
        sext(
            mem.rhw(addr).map_err(|_| Exception(LoadAccessFault))? as u32,
            16,
            32,
        ),
    );
    Ok(1)
}

fn lhu(cpu: &mut CPU, mem: &mut dyn Memory, parsed: IFormat) -> Result<u32, ExceptionInterrupt> {
    let addr = (cpu.get_x(parsed.rs1) as i32).wrapping_add(sext(parsed.imm, 12, 32) as i32) as u32;
    cpu.set_x(
        parsed.rd,
        mem.rhw(addr).map_err(|_| Exception(LoadAccessFault))? as u32,
    );
    Ok(1)
}

fn lw(cpu: &mut CPU, mem: &mut dyn Memory, parsed: IFormat) -> Result<u32, ExceptionInterrupt> {
    let addr = ((cpu.get_x(parsed.rs1) as i32) + (sext(parsed.imm, 12, 32) as i32)) as u32;
    cpu.set_x(
        parsed.rd,
        mem.rw(addr).map_err(|_| Exception(LoadAccessFault))?,
    );
    Ok(1)
}

fn lwu(cpu: &mut CPU, mem: &mut dyn Memory, parsed: IFormat) -> Result<u32, ExceptionInterrupt> {
    let addr = (cpu.get_x(parsed.rs1) as i32).wrapping_add(sext(parsed.imm, 12, 32) as i32) as u32;
    cpu.set_x(
        parsed.rd,
        mem.rw(addr).map_err(|_| Exception(LoadAccessFault))? as u32,
    );
    Ok(1)
}

fn sb(cpu: &mut CPU, mem: &mut dyn Memory, parsed: SFormat) -> Result<u32, ExceptionInterrupt> {
    let offset = parsed.imm0 | (parsed.imm1 << 5);
    let addr = (cpu.get_x(parsed.rs1) as i32).wrapping_add(sext(offset, 12, 32) as i32) as u32;
    mem.wb(addr, cpu.get_x(parsed.rs2) as u8)
        .map_err(|_| Exception(StoreAccessFault))?;
    Ok(1)
}

fn sh(cpu: &mut CPU, mem: &mut dyn Memory, parsed: SFormat) -> Result<u32, ExceptionInterrupt> {
    let offset = parsed.imm0 | (parsed.imm1 << 5);
    let addr = (cpu.get_x(parsed.rs1) as i32).wrapping_add(sext(offset, 12, 32) as i32) as u32;
    mem.whw(addr, cpu.get_x(parsed.rs2) as u16)
        .map_err(|_| Exception(StoreAccessFault))?;
    Ok(1)
}

fn sw(cpu: &mut CPU, mem: &mut dyn Memory, parsed: SFormat) -> Result<u32, ExceptionInterrupt> {
    let offset = parsed.imm0 | (parsed.imm1 << 5);
    let addr = ((cpu.get_x(parsed.rs1) as i32) + (sext(offset, 12, 32) as i32)) as u32;
    let value = cpu.get_x(parsed.rs2);
    mem.ww(addr, value)
        .map_err(|_| Exception(StoreAccessFault))?;
    Ok(1)
}

fn addi(cpu: &mut CPU, __mem: &mut dyn Memory, parsed: IFormat) -> Result<u32, ExceptionInterrupt> {
    cpu.set_x(
        parsed.rd,
        (cpu.get_x(parsed.rs1) as i32).wrapping_add(sext(parsed.imm, 12, 32) as i32) as u32,
    );
    Ok(1)
}

fn slti(cpu: &mut CPU, __mem: &mut dyn Memory, parsed: IFormat) -> Result<u32, ExceptionInterrupt> {
    let v = if (cpu.get_x(parsed.rs1) as i32) < sext(parsed.imm, 12, 32) as i32 {
        1
    } else {
        0
    };

    cpu.set_x(parsed.rd, v);
    Ok(1)
}

fn sltiu(
    cpu: &mut CPU,
    __mem: &mut dyn Memory,
    parsed: IFormat,
) -> Result<u32, ExceptionInterrupt> {
    let v = if cpu.get_x(parsed.rs1) < sext(parsed.imm, 12, 32) {
        1
    } else {
        0
    };

    cpu.set_x(parsed.rd, v);
    Ok(1)
}

fn xori(cpu: &mut CPU, __mem: &mut dyn Memory, parsed: IFormat) -> Result<u32, ExceptionInterrupt> {
    cpu.set_x(parsed.rd, cpu.get_x(parsed.rs1) ^ sext(parsed.imm, 12, 32));
    Ok(1)
}

fn ori(cpu: &mut CPU, __mem: &mut dyn Memory, parsed: IFormat) -> Result<u32, ExceptionInterrupt> {
    cpu.set_x(parsed.rd, cpu.get_x(parsed.rs1) | sext(parsed.imm, 12, 32));
    Ok(1)
}

fn andi(cpu: &mut CPU, __mem: &mut dyn Memory, parsed: IFormat) -> Result<u32, ExceptionInterrupt> {
    cpu.set_x(parsed.rd, cpu.get_x(parsed.rs1) & sext(parsed.imm, 12, 32));
    Ok(1)
}

fn slli(cpu: &mut CPU, __mem: &mut dyn Memory, parsed: IFormat) -> Result<u32, ExceptionInterrupt> {
    let shamt = parsed.imm & 0b11111;
    // TODO: shamt[5] should be 0, otherwise is an illegal instruction
    cpu.set_x(parsed.rd, cpu.get_x(parsed.rs1).wrapping_shl(shamt));
    Ok(1)
}

fn srli(cpu: &mut CPU, __mem: &mut dyn Memory, parsed: IFormat) -> Result<u32, ExceptionInterrupt> {
    let shamt = parsed.imm & 0b11111;
    // TODO: shamt[5] should be 0, otherwise is an illegal instruction
    cpu.set_x(parsed.rd, cpu.get_x(parsed.rs1).wrapping_shr(shamt));
    Ok(1)
}

fn srai(cpu: &mut CPU, __mem: &mut dyn Memory, parsed: IFormat) -> Result<u32, ExceptionInterrupt> {
    let shamt = parsed.imm & 0b11111;
    if shamt & 0b100000 != 0 {
        return Err(ExceptionInterrupt::Exception(Exception::IllegalInstruction));
    }

    let rs1 = cpu.get_x(parsed.rs1);
    cpu.set_x(parsed.rd, (rs1 as i32).wrapping_shr(shamt) as u32);
    Ok(1)
}

fn add(cpu: &mut CPU, _mem: &mut dyn Memory, parsed: RFormat) -> Result<u32, ExceptionInterrupt> {
    cpu.set_x(
        parsed.rd,
        cpu.get_x(parsed.rs1).wrapping_add(cpu.get_x(parsed.rs2)),
    );
    Ok(1)
}

fn sub(cpu: &mut CPU, _mem: &mut dyn Memory, parsed: RFormat) -> Result<u32, ExceptionInterrupt> {
    cpu.set_x(
        parsed.rd,
        cpu.get_x(parsed.rs1).wrapping_sub(cpu.get_x(parsed.rs2)),
    );
    Ok(1)
}

fn sll(cpu: &mut CPU, _mem: &mut dyn Memory, parsed: RFormat) -> Result<u32, ExceptionInterrupt> {
    cpu.set_x(
        parsed.rd,
        cpu.get_x(parsed.rs1)
            .wrapping_shl(cpu.get_x(parsed.rs2) & 0b11111),
    );
    Ok(1)
}

fn slt(cpu: &mut CPU, _mem: &mut dyn Memory, parsed: RFormat) -> Result<u32, ExceptionInterrupt> {
    let v = if (cpu.get_x(parsed.rs1) as i32) < cpu.get_x(parsed.rs2) as i32 {
        1
    } else {
        0
    };

    cpu.set_x(parsed.rd, v);
    Ok(1)
}

fn sltu(cpu: &mut CPU, _mem: &mut dyn Memory, parsed: RFormat) -> Result<u32, ExceptionInterrupt> {
    let v = if cpu.get_x(parsed.rs1) < cpu.get_x(parsed.rs2) {
        1
    } else {
        0
    };

    cpu.set_x(parsed.rd, v);
    Ok(1)
}

fn xor(cpu: &mut CPU, _mem: &mut dyn Memory, parsed: RFormat) -> Result<u32, ExceptionInterrupt> {
    cpu.set_x(parsed.rd, cpu.get_x(parsed.rs1) ^ cpu.get_x(parsed.rs2));
    Ok(1)
}

fn srl(cpu: &mut CPU, _mem: &mut dyn Memory, parsed: RFormat) -> Result<u32, ExceptionInterrupt> {
    cpu.set_x(
        parsed.rd,
        cpu.get_x(parsed.rs1)
            .wrapping_shr(cpu.get_x(parsed.rs2) & 0b11111),
    );
    Ok(1)
}

fn sra(cpu: &mut CPU, _mem: &mut dyn Memory, parsed: RFormat) -> Result<u32, ExceptionInterrupt> {
    let shamt = cpu.get_x(parsed.rs2) & 0b11111;
    let rs1 = cpu.get_x(parsed.rs1);
    cpu.set_x(parsed.rd, (rs1 as i32).wrapping_shr(shamt) as u32);
    Ok(1)
}

fn or(cpu: &mut CPU, _mem: &mut dyn Memory, parsed: RFormat) -> Result<u32, ExceptionInterrupt> {
    cpu.set_x(parsed.rd, cpu.get_x(parsed.rs1) | cpu.get_x(parsed.rs2));
    Ok(1)
}

fn and(cpu: &mut CPU, _mem: &mut dyn Memory, parsed: RFormat) -> Result<u32, ExceptionInterrupt> {
    cpu.set_x(parsed.rd, cpu.get_x(parsed.rs1) & cpu.get_x(parsed.rs2));
    Ok(1)
}

fn fence(
    _cpu: &mut CPU,
    _mem: &mut dyn Memory,
    _parsed: IFormat,
) -> Result<u32, ExceptionInterrupt> {
    Ok(0)
}

fn csrrw(cpu: &mut CPU, _mem: &mut dyn Memory, parsed: IFormat) -> Result<u32, ExceptionInterrupt> {
    let t = match cpu.get_csr(parsed.imm) {
        Ok(v) => v,
        Err(err) => return Err(ExceptionInterrupt::Exception(err)),
    };
    match cpu.set_csr(parsed.imm, cpu.get_x(parsed.rs1)) {
        Err(err) => return Err(ExceptionInterrupt::Exception(err)),
        _ => {}
    };
    cpu.set_x(parsed.rd, t as u32);
    Ok(1)
}

fn csrrs(cpu: &mut CPU, _mem: &mut dyn Memory, parsed: IFormat) -> Result<u32, ExceptionInterrupt> {
    let t = match cpu.get_csr(parsed.imm) {
        Ok(v) => v,
        Err(err) => return Err(ExceptionInterrupt::Exception(err)),
    };
    match cpu.set_csr(parsed.imm, cpu.get_x(parsed.rs1) | t) {
        Err(err) => return Err(ExceptionInterrupt::Exception(err)),
        _ => {}
    };
    cpu.set_x(parsed.rd, t as u32);
    Ok(1)
}

fn csrrc(cpu: &mut CPU, _mem: &mut dyn Memory, parsed: IFormat) -> Result<u32, ExceptionInterrupt> {
    let t = match cpu.get_csr(parsed.imm) {
        Ok(v) => v,
        Err(err) => return Err(ExceptionInterrupt::Exception(err)),
    };
    match cpu.set_csr(parsed.imm, t & !cpu.get_x(parsed.rs1)) {
        Err(err) => return Err(ExceptionInterrupt::Exception(err)),
        _ => {}
    };
    cpu.set_x(parsed.rd, t as u32);
    Ok(1)
}

fn csrrwi(
    cpu: &mut CPU,
    _mem: &mut dyn Memory,
    parsed: IFormat,
) -> Result<u32, ExceptionInterrupt> {
    cpu.set_x(
        parsed.rd,
        match cpu.get_csr(parsed.imm) {
            Ok(v) => v,
            Err(err) => return Err(ExceptionInterrupt::Exception(err)),
        },
    );
    match cpu.set_csr(parsed.imm, cpu.get_x(parsed.rs1) & 0b11111) {
        Err(err) => return Err(ExceptionInterrupt::Exception(err)),
        _ => (),
    };
    Ok(1)
}

fn csrrsi(
    cpu: &mut CPU,
    _mem: &mut dyn Memory,
    parsed: IFormat,
) -> Result<u32, ExceptionInterrupt> {
    let t = match cpu.get_csr(parsed.imm) {
        Ok(v) => v,
        Err(err) => {
            return Err(ExceptionInterrupt::Exception(err));
        }
    };
    let new_value = cpu.get_x(parsed.rs1) | t;
    match cpu.set_csr(parsed.imm, new_value) {
        Err(err) => {
            return Err(ExceptionInterrupt::Exception(err));
        }
        _ => {}
    };
    cpu.set_x(parsed.rd, t as u32);
    Ok(1)
}

fn csrrci(
    cpu: &mut CPU,
    _mem: &mut dyn Memory,
    parsed: IFormat,
) -> Result<u32, ExceptionInterrupt> {
    let t = match cpu.get_csr(parsed.imm) {
        Ok(v) => v,
        Err(err) => return Err(ExceptionInterrupt::Exception(err)),
    };
    match cpu.set_csr(parsed.imm, t & !(cpu.get_x(parsed.rs1) & 0b11111)) {
        Err(err) => return Err(ExceptionInterrupt::Exception(err)),
        _ => {}
    };
    cpu.set_x(parsed.rd, t as u32);
    Ok(1)
}
