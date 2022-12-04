use super::format::*;
use super::Instruction;
use crate::utils::*;
use crate::{Exception, ExceptionInterrupt, Memory, CPU};

use Exception::*;
use ExceptionInterrupt::*;

#[derive(Debug, Clone, Copy)]
pub enum RV32i {
    /// Load upper immediate
    /// Writes the sign-extended 20-bit immediate, left shifted by 12bits to x[rd] zeroin the lower
    /// 12 bit.
    LUI(UFormat),
    /// Add upper immediate to PC
    /// Adds the sign-extended 20bit immediate, left-shifted 12bit, to the pc, and writes the
    /// result to x[rd]
    AUIPC(UFormat),
    /// Jump and Link
    /// Writes the address of the next instruction (pc + 4) to x[rd]. then set hte pc to the
    /// current pc plus the sign-extended offset. If rd is omitted, x1 is used.
    JAL(JFormat),
    /// Jump and Link Register
    /// Sets the pc to x[rs1] + sign-extend(offset), masking off the least significant bit of the
    /// computed address, then writes the previous pc+4 to x[rd]. If rd is omitted, x1 is asumed;
    JALR(IFormat),
    /// Branch if Equal
    /// If register x[rs1] equals register x[rs2], set the pc to the current pc plus sign-extended
    /// offset.
    BEQ(BFormat),
    /// Branch if Not Equal
    /// If register x[rs1] does not equal register x[rs2], set the pc to the current pc plus the
    /// sign extended offset.
    BNE(BFormat),
    /// Branch if Less Than
    BLT(BFormat),
    /// Branch if Greater Than or Equal
    BGE(BFormat),
    /// Branch if Less Than unsigned
    BLTU(BFormat),
    /// Branch if Greater or Equal Unsigned
    BGEU(BFormat),
    LB(IFormat),
    LH(IFormat),
    LW(IFormat),
    LBU(IFormat),
    LHU(IFormat),
    LWU(IFormat),
    SB(SFormat),
    SH(SFormat),
    SW(SFormat),
    ADDI(IFormat),
    SLTI(IFormat),
    SLTIU(IFormat),
    XORI(IFormat),
    ORI(IFormat),
    ANDI(IFormat),
    SLLI(IFormat),
    SRLI(IFormat),
    SRAI(IFormat),
    ADD(RFormat),
    SUB(RFormat),
    SLL(RFormat),
    SLT(RFormat),
    SLTU(RFormat),
    XOR(RFormat),
    SRL(RFormat),
    SRA(RFormat),
    OR(RFormat),
    AND(RFormat),
    FENCE(IFormat),
    FENCEI(IFormat),
    ECALL(IFormat),
    EBREAK(IFormat),
    CSRRW(IFormat),
    CSRRS(IFormat),
    CSRRC(IFormat),
    CSRRWI(IFormat),
    CSRRSI(IFormat),
    CSRRCI(IFormat),
}

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
}

impl TryFrom<u32> for RV32i {
    type Error = ();
    fn try_from(word: u32) -> Result<Self, Self::Error> {
        use RV32i::*;
        let opcode = opcode(word);
        match opcode {
            0b0110111 => Ok(LUI(UFormat::from(word))),
            0b0010111 => Ok(AUIPC(UFormat::from(word))),
            0b1101111 => Ok(JAL(JFormat::from(word))),
            0b1100111 => Ok(JALR(IFormat::from(word))),
            0b1100011 => branch(word),
            0b0000011 => load(word),
            0b0100011 => store(word),
            0b0010011 => immediate(word),
            0b0110011 => arithmetic(word),
            0b0001111 => fences(word),
            0b1110011 => system(word),
            _ => Err(()),
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
    cpu.set_x(parsed.rd, cpu.pc + sext(parsed.imm << 12, 32, 32));
    Ok(1)
}

/// Jump and Link
fn jal(cpu: &mut CPU, _mem: &mut dyn Memory, parsed: JFormat) -> Result<u32, ExceptionInterrupt> {
    cpu.set_x(parsed.rd, cpu.pc);
    let offset =
        (parsed.imm0 << 12) | (parsed.imm1 << 11) | (parsed.imm2 << 1) | (parsed.imm3 << 20);
    cpu.pc = (cpu.pc as i32 + (sext(offset, 20, 32) as i32).wrapping_sub(4)) as u32;
    Ok(1)
}

/// Jump and Link Register
fn jalr(cpu: &mut CPU, _mem: &mut dyn Memory, parsed: IFormat) -> Result<u32, ExceptionInterrupt> {
    let rd = if parsed.rd == 0 { 1 } else { parsed.rd };
    let t = cpu.get_x(parsed.rs1);
    cpu.set_x(rd, cpu.pc);
    cpu.pc = (((t as i32).wrapping_add(sext(parsed.imm, 12, 32) as i32)) & !(0b1 as i32)) as u32;
    Ok(1)
}

fn beq(cpu: &mut CPU, _mem: &mut dyn Memory, parsed: BFormat) -> Result<u32, ExceptionInterrupt> {
    if cpu.get_x(parsed.rs1) == cpu.get_x(parsed.rs2) {
        let offset = parsed.imm0 << 11 | parsed.imm1 << 1 | parsed.imm2 << 5 | parsed.imm3 << 12;
        cpu.pc = (cpu.pc as i32)
            .wrapping_add(sext(offset, 12, 32) as i32)
            .wrapping_sub(4) as u32;
    }
    Ok(1)
}

fn bge(cpu: &mut CPU, _mem: &mut dyn Memory, parsed: BFormat) -> Result<u32, ExceptionInterrupt> {
    if cpu.get_x(parsed.rs1) as i32 >= cpu.get_x(parsed.rs2) as i32 {
        let offset = parsed.imm0 << 11 | parsed.imm1 << 1 | parsed.imm2 << 5 | parsed.imm3 << 12;
        cpu.pc = (cpu.pc as i32)
            .wrapping_add(sext(offset, 12, 32) as i32)
            .wrapping_sub(4) as u32;
    }
    Ok(1)
}

fn bgeu(cpu: &mut CPU, _mem: &mut dyn Memory, parsed: BFormat) -> Result<u32, ExceptionInterrupt> {
    if cpu.get_x(parsed.rs1) >= cpu.get_x(parsed.rs2) {
        let offset = parsed.imm0 << 11 | parsed.imm1 << 1 | parsed.imm2 << 5 | parsed.imm3 << 12;
        cpu.pc = (cpu.pc as i32)
            .wrapping_add(sext(offset, 12, 32) as i32)
            .wrapping_sub(4) as u32;
    }
    Ok(1)
}

fn blt(cpu: &mut CPU, _mem: &mut dyn Memory, parsed: BFormat) -> Result<u32, ExceptionInterrupt> {
    if (cpu.get_x(parsed.rs1) as i32) < (cpu.get_x(parsed.rs2) as i32) {
        let offset = parsed.imm0 << 11 | parsed.imm1 << 1 | parsed.imm2 << 5 | parsed.imm3 << 12;
        cpu.pc = (cpu.pc as i32)
            .wrapping_add(sext(offset, 12, 32) as i32)
            .wrapping_sub(4) as u32;
    }
    Ok(1)
}

fn bltu(cpu: &mut CPU, _mem: &mut dyn Memory, parsed: BFormat) -> Result<u32, ExceptionInterrupt> {
    if cpu.get_x(parsed.rs1) < cpu.get_x(parsed.rs2) {
        let offset = parsed.imm0 << 11 | parsed.imm1 << 1 | parsed.imm2 << 5 | parsed.imm3 << 12;
        cpu.pc = (cpu.pc as i32)
            .wrapping_add(sext(offset, 12, 32) as i32)
            .wrapping_sub(4) as u32;
    }
    Ok(1)
}

fn bne(cpu: &mut CPU, _mem: &mut dyn Memory, parsed: BFormat) -> Result<u32, ExceptionInterrupt> {
    if cpu.get_x(parsed.rs1) != cpu.get_x(parsed.rs2) {
        let offset = parsed.imm0 << 11 | parsed.imm1 << 1 | parsed.imm2 << 5 | parsed.imm3 << 12;
        cpu.pc = (cpu.pc as i32)
            .wrapping_add(sext(offset, 12, 32) as i32)
            .wrapping_sub(4) as u32;
    }
    Ok(1)
}

/// Branch parsing
fn branch(word: u32) -> Result<RV32i, ()> {
    use RV32i::*;
    let parsed = BFormat::try_from(word).unwrap();
    match parsed.funct3 {
        0b000 => Ok(BEQ(parsed)),
        0b101 => Ok(BGE(parsed)),
        0b111 => Ok(BGEU(parsed)),
        0b100 => Ok(BLT(parsed)),
        0b110 => Ok(BLTU(parsed)),
        0b001 => Ok(BNE(parsed)),
        _ => Err(()),
    }
}

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
    let addr = (cpu.get_x(parsed.rs1) as i32).wrapping_add(sext(parsed.imm, 12, 32) as i32) as u32;
    cpu.set_x(
        parsed.rd,
        sext(
            mem.rw(addr).map_err(|_| Exception(LoadAccessFault))? as u32,
            32,
            32,
        ),
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

fn load(word: u32) -> Result<RV32i, ()> {
    use RV32i::*;
    let parsed = IFormat::try_from(word).unwrap();
    match parsed.funct3 {
        0b000 => Ok(LB(parsed)),
        0b100 => Ok(LBU(parsed)),
        0b001 => Ok(LH(parsed)),
        0b101 => Ok(LHU(parsed)),
        0b010 => Ok(LW(parsed)),
        0b110 => Ok(LWU(parsed)),
        _ => Err(()),
    }
}

fn sb(cpu: &mut CPU, mem: &mut dyn Memory, parsed: SFormat) -> Result<u32, ExceptionInterrupt> {
    let addr = (cpu.get_x(parsed.rs1) as i32)
        .wrapping_add(sext(parsed.imm0 | parsed.imm1, 12, 32) as i32) as u32;
    mem.wb(addr, cpu.get_x(parsed.rs2) as u8)
        .map_err(|_| Exception(StoreAccessFault))?;
    Ok(1)
}

fn sh(cpu: &mut CPU, mem: &mut dyn Memory, parsed: SFormat) -> Result<u32, ExceptionInterrupt> {
    let addr = (cpu.get_x(parsed.rs1) as i32)
        .wrapping_add(sext(parsed.imm0 | parsed.imm1, 12, 32) as i32) as u32;
    mem.whw(addr, cpu.get_x(parsed.rs2) as u16)
        .map_err(|_| Exception(StoreAccessFault))?;
    Ok(1)
}

fn sw(cpu: &mut CPU, mem: &mut dyn Memory, parsed: SFormat) -> Result<u32, ExceptionInterrupt> {
    let addr = (cpu.get_x(parsed.rs1) as i32)
        .wrapping_add(sext(parsed.imm0 | parsed.imm1, 12, 32) as i32) as u32;
    let value = cpu.get_x(parsed.rs2);
    mem.ww(addr, value)
        .map_err(|_| Exception(StoreAccessFault))?;
    Ok(1)
}

fn store(word: u32) -> Result<RV32i, ()> {
    use RV32i::*;
    let parsed = SFormat::try_from(word).unwrap();
    match parsed.funct3 {
        0b000 => Ok(SB(parsed)),
        0b001 => Ok(SH(parsed)),
        0b010 => Ok(SW(parsed)),
        _ => Err(()),
    }
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
    cpu.set_x(
        parsed.rd,
        cpu.get_x(parsed.rs1) ^ sext(cpu.get_x(parsed.imm), 12, 32),
    );
    Ok(1)
}

fn ori(cpu: &mut CPU, __mem: &mut dyn Memory, parsed: IFormat) -> Result<u32, ExceptionInterrupt> {
    cpu.set_x(
        parsed.rd,
        cpu.get_x(parsed.rs1) | sext(cpu.get_x(parsed.imm), 12, 32),
    );
    Ok(1)
}

fn andi(cpu: &mut CPU, __mem: &mut dyn Memory, parsed: IFormat) -> Result<u32, ExceptionInterrupt> {
    cpu.set_x(
        parsed.rd,
        cpu.get_x(parsed.rs1) & sext(cpu.get_x(parsed.imm), 12, 32),
    );
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

fn immediate(word: u32) -> Result<RV32i, ()> {
    use RV32i::*;
    let parsed = IFormat::try_from(word).unwrap();
    match parsed.funct3 {
        0b000 => Ok(ADDI(parsed)),
        0b010 => Ok(SLTI(parsed)),
        0b011 => Ok(SLTIU(parsed)),
        0b100 => Ok(XORI(parsed)),
        0b110 => Ok(ORI(parsed)),
        0b111 => Ok(ANDI(parsed)),
        0b001 => Ok(SLLI(parsed)),
        0b101 if parsed.imm & (0b111111 << 11) == 0 => Ok(SRLI(parsed)),
        0b101 => Ok(SRAI(parsed)),
        _ => Err(()),
    }
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

fn arithmetic(word: u32) -> Result<RV32i, ()> {
    use RV32i::*;
    let parsed = RFormat::from(word);
    match parsed.funct3 {
        0b000 if parsed.funct7 == 0 => Ok(ADD(parsed)),
        0b000 => Ok(SUB(parsed)),
        0b001 => Ok(SLL(parsed)),
        0b010 => Ok(SLT(parsed)),
        0b011 => Ok(SLTU(parsed)),
        0b100 => Ok(XOR(parsed)),
        0b101 if parsed.funct7 == 0 => Ok(SRL(parsed)),
        0b101 => Ok(SRA(parsed)),
        0b110 => Ok(OR(parsed)),
        0b111 => Ok(AND(parsed)),
        _ => Err(()),
    }
}

fn fences(word: u32) -> Result<RV32i, ()> {
    use RV32i::*;
    let parsed = IFormat::from(word);
    match parsed.funct3 {
        0 => Ok(FENCE(parsed)),
        1 => Ok(FENCEI(parsed)),
        _ => Err(()),
    }
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

fn system(word: u32) -> Result<RV32i, ()> {
    use RV32i::*;
    let parsed = IFormat::from(word);

    match parsed.funct3 {
        0b000 if parsed.imm == 0 => Ok(ECALL(parsed)),
        0b000 if parsed.imm == 1 => Ok(EBREAK(parsed)),
        0b001 => Ok(CSRRW(parsed)),
        0b010 => Ok(CSRRS(parsed)),
        0b011 => Ok(CSRRC(parsed)),
        0b101 => Ok(CSRRWI(parsed)),
        0b110 => Ok(CSRRSI(parsed)),
        0b111 => Ok(CSRRCI(parsed)),
        _ => Err(()),
    }
}

#[cfg(test)]
mod tests {
    use super::RV32i::*;
    use super::*;
    use crate::memory::GenericMemory;

    #[test]
    fn test_lui() {
        let mut cpu = CPU::new();
        let mut mem = GenericMemory::new(1024 * 100);
        // lui x1, 0x23
        let lui = LUI(UFormat { rd: 1, imm: 23 });
        lui.execute(&mut cpu, &mut mem).unwrap();
        assert_eq!(cpu.get_x(1), 23 << 12);
    }

    #[test]
    fn test_auipc() {
        let mut cpu = CPU::new();
        let mut mem = GenericMemory::new(1024 * 100);
        cpu.pc = 10;
        let inst = AUIPC(UFormat { rd: 1, imm: 23 });
        inst.execute(&mut cpu, &mut mem).unwrap();
        assert_eq!(cpu.get_x(1), (23 << 12) + 10);
    }

    #[test]
    fn test_jal() {
        let mut cpu = CPU::new();
        let mut mem = GenericMemory::new(1024 * 100);
        cpu.pc = 10;
        let inst = JAL(JFormat {
            rd: 1,
            imm0: 0,
            imm1: 0,
            imm2: 23,
            imm3: 0,
        });

        inst.execute(&mut cpu, &mut mem).unwrap();
        assert_eq!(cpu.get_x(1), 10 + 4);
        assert_eq!(cpu.pc, 10 + 23);
    }

    #[test]
    fn test_jalr() {
        let mut cpu = CPU::new();
        let mut mem = GenericMemory::new(1024 * 100);
        cpu.pc = 10;
        cpu.set_x(2, 3);
        let inst = JALR(IFormat {
            op: 0b1100111,
            rd: 1,
            funct3: 0,
            rs1: 2,
            imm: 23,
        });

        inst.execute(&mut cpu, &mut mem).unwrap();

        assert_eq!(cpu.get_x(1), 10 + 4);
        assert_eq!(cpu.pc, 3 + 23);
    }

    #[test]
    fn test_beq0() {
        let mut cpu = CPU::new();
        let mut mem = GenericMemory::new(1024 * 100);
        cpu.pc = 10;
        cpu.set_x(1, 1);
        cpu.set_x(2, 1);
        let inst = BEQ(BFormat {
            op: 0b1100011,
            funct3: 0,
            rs1: 1,
            rs2: 2,
            imm0: 0,
            imm1: 15,
            imm2: 0,
            imm3: 0,
        });

        inst.execute(&mut cpu, &mut mem).unwrap();

        assert_eq!(cpu.pc, 10 + 15);
    }

    #[test]
    fn test_beq1() {
        let mut cpu = CPU::new();
        let mut mem = GenericMemory::new(1024 * 100);
        cpu.pc = 10;
        cpu.set_x(1, 1);
        cpu.set_x(2, 2);
        let inst = BEQ(BFormat {
            op: 0b1100011,
            funct3: 0,
            rs1: 1,
            rs2: 2,
            imm0: 0,
            imm1: 15,
            imm2: 0,
            imm3: 0,
        });

        inst.execute(&mut cpu, &mut mem).unwrap();

        assert_eq!(cpu.pc, 10);
    }

    #[test]
    fn test_bne0() {
        let mut cpu = CPU::new();
        let mut mem = GenericMemory::new(1024 * 100);
        cpu.pc = 10;
        cpu.set_x(1, 1);
        cpu.set_x(2, 1);
        let inst = BNE(BFormat {
            op: 0b1100011,
            funct3: 0,
            rs1: 1,
            rs2: 2,
            imm0: 0,
            imm1: 15,
            imm2: 0,
            imm3: 0,
        });

        inst.execute(&mut cpu, &mut mem).unwrap();

        assert_eq!(cpu.pc, 10);
    }

    #[test]
    fn test_bne1() {
        let mut cpu = CPU::new();
        let mut mem = GenericMemory::new(1024 * 100);
        cpu.pc = 10;
        cpu.set_x(1, 1);
        cpu.set_x(2, 2);
        let inst = BNE(BFormat {
            op: 0b1100011,
            funct3: 0,
            rs1: 1,
            rs2: 2,
            imm0: 0,
            imm1: 15,
            imm2: 0,
            imm3: 0,
        });

        inst.execute(&mut cpu, &mut mem).unwrap();

        assert_eq!(cpu.pc, 10 + 15);
    }

    #[test]
    fn test_blt0() {
        let mut cpu = CPU::new();
        let mut mem = GenericMemory::new(1024 * 100);
        cpu.pc = 10;
        cpu.set_x(1, 1);
        cpu.set_x(2, 1);
        let inst = BLT(BFormat {
            op: 0b1100011,
            funct3: 0,
            rs1: 1,
            rs2: 2,
            imm0: 0,
            imm1: 15,
            imm2: 0,
            imm3: 0,
        });

        inst.execute(&mut cpu, &mut mem).unwrap();

        assert_eq!(cpu.pc, 10);
    }

    #[test]
    fn test_blt1() {
        let mut cpu = CPU::new();
        let mut mem = GenericMemory::new(1024 * 100);
        cpu.pc = 10;
        cpu.set_x(1, 1);
        cpu.set_x(2, 2);
        let inst = BLT(BFormat {
            op: 0b1100011,
            funct3: 0,
            rs1: 1,
            rs2: 2,
            imm0: 0,
            imm1: 15,
            imm2: 0,
            imm3: 0,
        });

        inst.execute(&mut cpu, &mut mem).unwrap();

        assert_eq!(cpu.pc, 10 + 15);
    }

    #[test]
    fn test_bge0() {
        let mut cpu = CPU::new();
        let mut mem = GenericMemory::new(1024 * 100);
        cpu.pc = 10;
        cpu.set_x(1, 0);
        cpu.set_x(2, 1);
        let inst = BGE(BFormat {
            op: 0b1100011,
            funct3: 0,
            rs1: 1,
            rs2: 2,
            imm0: 0,
            imm1: 15,
            imm2: 0,
            imm3: 0,
        });

        inst.execute(&mut cpu, &mut mem).unwrap();

        assert_eq!(cpu.pc, 10);
    }

    #[test]
    fn test_bge1() {
        let mut cpu = CPU::new();
        let mut mem = GenericMemory::new(1024 * 100);
        cpu.pc = 10;
        cpu.set_x(1, 1);
        cpu.set_x(2, 1);
        let inst = BGE(BFormat {
            op: 0b1100011,
            funct3: 0,
            rs1: 1,
            rs2: 2,
            imm0: 0,
            imm1: 15,
            imm2: 0,
            imm3: 0,
        });

        inst.execute(&mut cpu, &mut mem).unwrap();

        assert_eq!(cpu.pc, 10 + 15);
    }

    #[test]
    fn test_bge2() {
        let mut cpu = CPU::new();
        let mut mem = GenericMemory::new(1024 * 100);
        cpu.pc = 10;
        cpu.set_x(1, 2);
        cpu.set_x(2, 1);
        let inst = BGE(BFormat {
            op: 0b1100011,
            funct3: 0,
            rs1: 1,
            rs2: 2,
            imm0: 0,
            imm1: 15,
            imm2: 0,
            imm3: 0,
        });

        inst.execute(&mut cpu, &mut mem).unwrap();

        assert_eq!(cpu.pc, 10 + 15);
    }
}
