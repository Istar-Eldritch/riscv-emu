use crate::format::{BFormat, IFormat, JFormat, RFormat, SFormat, UFormat, OPCODE_MASK};
use macros::instruction;

#[derive(Debug, Clone, Copy)]
pub enum RV32i {
    /// Load upper immediate
    /// Writes the sign-extended 20-bit immediate, left shifted by 12bits to x[rd] zeroin the lower
    /// 12 bit.
    LUI(LUI),
    /// Add upper immediate to PC
    /// Adds the sign-extended 20bit immediate, left-shifted 12bit, to the pc, and writes the
    /// result to x[rd]
    AUIPC(AUIPC),
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

impl From<RV32i> for u32 {
    fn from(inst: RV32i) -> u32 {
        use RV32i::*;
        match inst {
            LUI(f) => f.into(),
            AUIPC(f) => f.into(),
            JAL(f) => f.into(),
            JALR(f) => f.into(),
            BEQ(f) => f.into(),
            BNE(f) => f.into(),
            BLT(f) => f.into(),
            BGE(f) => f.into(),
            BLTU(f) => f.into(),
            BGEU(f) => f.into(),
            LB(f) => f.into(),
            LH(f) => f.into(),
            LW(f) => f.into(),
            LBU(f) => f.into(),
            LHU(f) => f.into(),
            LWU(f) => f.into(),
            SB(f) => f.into(),
            SH(f) => f.into(),
            SW(f) => f.into(),
            ADDI(f) => f.into(),
            SLTI(f) => f.into(),
            SLTIU(f) => f.into(),
            XORI(f) => f.into(),
            ORI(f) => f.into(),
            ANDI(f) => f.into(),
            SLLI(f) => f.into(),
            SRLI(f) => f.into(),
            SRAI(f) => f.into(),
            ADD(f) => f.into(),
            SUB(f) => f.into(),
            SLL(f) => f.into(),
            SLT(f) => f.into(),
            SLTU(f) => f.into(),
            XOR(f) => f.into(),
            SRL(f) => f.into(),
            SRA(f) => f.into(),
            OR(f) => f.into(),
            AND(f) => f.into(),
            FENCE(f) => f.into(),
            FENCEI(f) => f.into(),
            ECALL(f) => f.into(),
            EBREAK(f) => f.into(),
            CSRRW(f) => f.into(),
            CSRRS(f) => f.into(),
            CSRRC(f) => f.into(),
            CSRRWI(f) => f.into(),
            CSRRSI(f) => f.into(),
            CSRRCI(f) => f.into(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
#[instruction(UFormat, op = 0b1110011)]
pub struct LUI {
    pub rd: u32,
    pub imm: u32,
}

#[derive(Debug, Clone, Copy)]
#[instruction(UFormat, op = 0b0010111)]
pub struct AUIPC {
    pub rd: u32,
    pub imm: u32,
}

impl TryFrom<u32> for RV32i {
    type Error = ();
    fn try_from(word: u32) -> Result<Self, Self::Error> {
        if let Ok(lui) = LUI::try_from(word) {
            return Ok(RV32i::LUI(lui));
        } else if let Ok(auipc) = AUIPC::try_from(word) {
            return Ok(RV32i::AUIPC(auipc));
        }
        let opcode = word & OPCODE_MASK;
        match opcode {
            0b1101111 => Ok(RV32i::JAL(JFormat::from(word))),
            0b1100111 => Ok(RV32i::JALR(IFormat::from(word))),
            0b1100011 => branch(word),
            0b0000011 => load(word),
            0b0100011 => store(word),
            0b0010011 => immediate(word),
            0b0110011 => binops(word),
            0b0001111 => fences(word),
            0b1110011 => system(word),
            _ => Err(()),
        }
    }
}

fn branch(word: u32) -> Result<RV32i, ()> {
    use RV32i::*;
    let parsed = BFormat::from(word);
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

fn load(word: u32) -> Result<RV32i, ()> {
    use RV32i::*;
    let parsed = IFormat::from(word);
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

fn store(word: u32) -> Result<RV32i, ()> {
    use RV32i::*;
    let parsed = SFormat::from(word);
    match parsed.funct3 {
        0b000 => Ok(SB(parsed)),
        0b001 => Ok(SH(parsed)),
        0b010 => Ok(SW(parsed)),
        _ => Err(()),
    }
}

fn immediate(word: u32) -> Result<RV32i, ()> {
    use RV32i::*;
    let parsed = IFormat::from(word);
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

fn binops(word: u32) -> Result<RV32i, ()> {
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
