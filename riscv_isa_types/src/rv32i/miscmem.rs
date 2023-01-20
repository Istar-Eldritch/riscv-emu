use crate::format::IFormat;
use macros::instruction;

#[derive(Debug, Clone, Copy)]
#[instruction]
#[format(IFormat)]
#[checks(op = 0b0001111, funct3 = 0b000)]
pub struct FENCE {
    pub rd: u32,
    pub rs1: u32,
    pub imm: u32,
}

#[derive(Debug, Clone, Copy)]
#[instruction]
#[format(IFormat)]
#[checks(op = 0b0001111, funct3 = 0b001)]
pub struct FENCEI {
    pub rd: u32,
    pub rs1: u32,
    pub imm: u32,
}
#[derive(Debug, Clone, Copy)]
#[instruction]
#[format(IFormat)]
#[checks(op = 0b0001111)]
pub enum MiscMem {
    #[checks(funct3 = 0b000)]
    FENCE(FENCE),
    #[checks(funct3 = 0b001)]
    FENCEI(FENCEI),
}
