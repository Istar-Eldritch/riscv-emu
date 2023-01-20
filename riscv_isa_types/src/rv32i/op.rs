use crate::format::RFormat;
use macros::instruction;

#[derive(Debug, Clone, Copy)]
#[instruction]
#[format(RFormat)]
#[checks(op = 0b0110011, funct3 = 0b111)]
pub struct AND {
    pub rd: u32,
    pub rs1: u32,
    pub rs2: u32,
}

#[derive(Debug, Clone, Copy)]
#[instruction]
#[format(RFormat)]
#[checks(op = 0b0110011, funct3 = 0b110)]
pub struct OR {
    pub rd: u32,
    pub rs1: u32,
    pub rs2: u32,
}

#[derive(Debug, Clone, Copy)]
#[instruction]
#[format(RFormat)]
#[checks(op = 0b0110011, funct3 = 0b101)]
pub struct SRA {
    pub rd: u32,
    pub rs1: u32,
    pub rs2: u32,
}

#[derive(Debug, Clone, Copy)]
#[instruction]
#[format(RFormat)]
#[checks(op = 0b0110011, funct3 = 0b101, funct7 = 0b000)]
pub struct SRL {
    pub rd: u32,
    pub rs1: u32,
    pub rs2: u32,
}

#[derive(Debug, Clone, Copy)]
#[instruction]
#[format(RFormat)]
#[checks(op = 0b0110011, funct3 = 0b100)]
pub struct XOR {
    pub rd: u32,
    pub rs1: u32,
    pub rs2: u32,
}

#[derive(Debug, Clone, Copy)]
#[instruction]
#[format(RFormat)]
#[checks(op = 0b0110011, funct3 = 0b011)]
pub struct SLTU {
    pub rd: u32,
    pub rs1: u32,
    pub rs2: u32,
}

#[derive(Debug, Clone, Copy)]
#[instruction]
#[format(RFormat)]
#[checks(op = 0b0110011, funct3 = 0b010)]
pub struct SLT {
    pub rd: u32,
    pub rs1: u32,
    pub rs2: u32,
}

#[derive(Debug, Clone, Copy)]
#[instruction]
#[format(RFormat)]
#[checks(op = 0b0110011, funct3 = 0b001)]
pub struct SLL {
    pub rd: u32,
    pub rs1: u32,
    pub rs2: u32,
}

#[derive(Debug, Clone, Copy)]
#[instruction]
#[format(RFormat)]
#[checks(op = 0b0110011, funct3 = 0b000, funct7 = 0b000)]
pub struct SUB {
    pub rd: u32,
    pub rs1: u32,
    pub rs2: u32,
}

#[derive(Debug, Clone, Copy)]
#[instruction]
#[format(RFormat)]
#[checks(op = 0b0110011, funct3 = 0b000, funct7 = 0b000)]
pub struct ADD {
    pub rd: u32,
    pub rs1: u32,
    pub rs2: u32,
}

#[derive(Debug, Clone, Copy)]
#[instruction]
#[format(RFormat)]
#[checks(op = 0b0110011)]
pub enum Op {
    #[checks(funct3 = 0b111)]
    AND(AND),
    #[checks(funct3 = 0b110)]
    OR(OR),
    #[checks(funct3 = 0b101, funct7 = 0b0000000)]
    SRL(SRL),
    #[checks(funct3 = 0b101, funct7 = 0b0100000)]
    SRA(SRA),
    #[checks(funct3 = 0b100)]
    XOR(XOR),
    #[checks(funct3 = 0b011)]
    SLTU(SLTU),
    #[checks(funct3 = 0b010)]
    SLT(SLT),
    #[checks(funct3 = 0b001)]
    SLL(SLL),
    #[checks(funct3 = 0b000, funct7 = 0b0000000)]
    ADD(ADD),
    #[checks(funct3 = 0b000, funct7 = 0b0100000)]
    SUB(SUB),
}
