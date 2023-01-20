use crate::format::IFormat;
use macros::instruction;

#[derive(Debug, Clone, Copy)]
#[instruction]
#[format(IFormat)]
#[checks(op = 0b0010011, funct3 = 0b000)]
pub struct ADDI {
    pub rd: u32,
    pub rs1: u32,
    pub imm: u32,
}

#[derive(Debug, Clone, Copy)]
#[instruction]
#[format(IFormat)]
#[checks(op = 0b0010011, funct3 = 0b010)]
pub struct SLTI {
    pub rd: u32,
    pub rs1: u32,
    pub imm: u32,
}

#[derive(Debug, Clone, Copy)]
#[instruction]
#[format(IFormat)]
#[checks(op = 0b0010011, funct3 = 0b011)]
pub struct SLTIU {
    pub rd: u32,
    pub rs1: u32,
    pub imm: u32,
}

#[derive(Debug, Clone, Copy)]
#[instruction]
#[format(IFormat)]
#[checks(op = 0b0010011, funct3 = 0b100)]
pub struct XORI {
    pub rd: u32,
    pub rs1: u32,
    pub imm: u32,
}

#[derive(Debug, Clone, Copy)]
#[instruction]
#[format(IFormat)]
#[checks(op = 0b0010011, funct3 = 0b110)]
pub struct ORI {
    pub rd: u32,
    pub rs1: u32,
    pub imm: u32,
}

#[derive(Debug, Clone, Copy)]
#[instruction]
#[format(IFormat)]
#[checks(op = 0b0010011, funct3 = 0b111)]
pub struct ANDI {
    pub rd: u32,
    pub rs1: u32,
    pub imm: u32,
}

#[derive(Debug, Clone, Copy)]
#[instruction]
#[format(IFormat)]
#[checks(op = 0b0010011, funct3 = 0b001)]
pub struct SLLI {
    pub rd: u32,
    pub rs1: u32,
    pub imm: u32,
}

#[derive(Debug, Clone, Copy)]
#[instruction]
#[format(IFormat)]
#[checks(op = 0b0010011, funct3 = 0b101)] // TODO: Extra checks
pub struct SRLI {
    pub rd: u32,
    pub rs1: u32,
    pub imm: u32,
}

#[derive(Debug, Clone, Copy)]
#[instruction]
#[format(IFormat)]
#[checks(op = 0b0010011, funct3 = 0b101)] // TODO: Extra checks
pub struct SRAI {
    pub rd: u32,
    pub rs1: u32,
    pub imm: u32,
}

#[derive(Debug, Clone, Copy)]
#[instruction]
#[format(IFormat)]
#[checks(op = 0b0010011)]
pub enum Immediate {
    #[checks(funct3 = 0b000)]
    ADDI(ADDI),
    #[checks(funct3 = 0b010)]
    SLTI(SLTI),
    #[checks(funct3 = 0b011)]
    SLTIU(SLTIU),
    #[checks(funct3 = 0b100)]
    XORI(XORI),
    #[checks(funct3 = 0b110)]
    ORI(ORI),
    #[checks(funct3 = 0b111)]
    ANDI(ANDI),
    #[checks(funct3 = 0b001)]
    SLLI(SLLI),
    #[checks(funct3 = 0b101, funct7 = 0b000000)]
    SRLI(SRLI),
    #[checks(funct3 = 0b101, funct7 = 0b010000)]
    SRAI(SRAI),
}
