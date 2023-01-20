use crate::format::SFormat;
use macros::instruction;

#[derive(Debug, Clone, Copy)]
#[instruction]
#[format(SFormat)]
#[checks(op = 0b0100011, funct3 = 0b000)]
pub struct SB {
    pub rs1: u32,
    pub rs2: u32,
    #[format_mapping(imm0 = 0, imm1 = 5)]
    pub offset: u32,
}

#[derive(Debug, Clone, Copy)]
#[instruction]
#[format(SFormat)]
#[checks(op = 0b0100011, funct3 = 0b001)]
pub struct SH {
    pub rs1: u32,
    pub rs2: u32,
    #[format_mapping(imm0 = 0, imm1 = 5)]
    pub offset: u32,
}

#[derive(Debug, Clone, Copy)]
#[instruction]
#[format(SFormat)]
#[checks(op = 0b0100011, funct3 = 0b010)]
pub struct SW {
    pub rs1: u32,
    pub rs2: u32,
    #[format_mapping(imm0 = 0, imm1 = 5)]
    pub offset: u32,
}

#[derive(Debug, Clone, Copy)]
#[instruction]
#[format(SFormat)]
#[checks(op = 0b0100011)]
pub enum Store {
    #[checks(funct3 = 0b000)]
    SB(SB),
    #[checks(funct3 = 0b001)]
    SH(SH),
    #[checks(funct3 = 0b010)]
    SW(SW),
}
