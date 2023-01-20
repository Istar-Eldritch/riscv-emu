use crate::format::BFormat;
use macros::instruction;

#[derive(Debug, Clone, Copy)]
#[instruction]
#[format(BFormat)]
#[checks(op = 0b1100011, funct3 = 0b000)]
pub struct BEQ {
    pub rs1: u32,
    pub rs2: u32,
    #[format_mapping(imm0 = 11, imm1 = 1, imm2 = 5, imm3 = 12)]
    pub offset: u32,
}

#[derive(Debug, Clone, Copy)]
#[instruction]
#[format(BFormat)]
#[checks(op = 0b1100011, funct3 = 0b101)]
pub struct BGE {
    pub rs1: u32,
    pub rs2: u32,
    #[format_mapping(imm0 = 11, imm1 = 1, imm2 = 5, imm3 = 12)]
    pub offset: u32,
}

#[derive(Debug, Clone, Copy)]
#[instruction]
#[format(BFormat)]
#[checks(op = 0b1100011, funct3 = 0b111)]
pub struct BGEU {
    pub rs1: u32,
    pub rs2: u32,
    #[format_mapping(imm0 = 11, imm1 = 1, imm2 = 5, imm3 = 12)]
    pub offset: u32,
}

#[derive(Debug, Clone, Copy)]
#[instruction]
#[format(BFormat)]
#[checks(op = 0b1100011, funct3 = 0b100)]
pub struct BLT {
    pub rs1: u32,
    pub rs2: u32,
    #[format_mapping(imm0 = 11, imm1 = 1, imm2 = 5, imm3 = 12)]
    pub offset: u32,
}

#[derive(Debug, Clone, Copy)]
#[instruction]
#[format(BFormat)]
#[checks(op = 0b1100011, funct3 = 0b110)]
pub struct BLTU {
    pub rs1: u32,
    pub rs2: u32,
    #[format_mapping(imm0 = 11, imm1 = 1, imm2 = 5, imm3 = 12)]
    pub offset: u32,
}

#[derive(Debug, Clone, Copy)]
#[instruction]
#[format(BFormat)]
#[checks(op = 0b1100011, funct3 = 0b001)]
pub struct BNE {
    pub rs1: u32,
    pub rs2: u32,
    #[format_mapping(imm0 = 11, imm1 = 1, imm2 = 5, imm3 = 12)]
    pub offset: u32,
}

#[derive(Debug, Clone, Copy)]
#[instruction]
#[format(BFormat)]
#[checks(op = 0b1100011)]
pub enum Branch {
    #[checks(funct3 = 0b000)]
    BEQ(BEQ),
    #[checks(funct3 = 0b101)]
    BGE(BGE),
    #[checks(funct3 = 0b111)]
    BGEU(BGEU),
    #[checks(funct3 = 0b100)]
    BLT(BLT),
    #[checks(funct3 = 0b110)]
    BLTU(BLTU),
    #[checks(funct3 = 0b001)]
    BNE(BNE),
}
