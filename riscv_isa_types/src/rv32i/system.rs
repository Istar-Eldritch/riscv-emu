use crate::format::IFormat;
use macros::instruction;

#[derive(Debug, Clone, Copy)]
#[instruction]
#[format(IFormat)]
#[checks(op = 0b1110011)]
pub enum System {
    #[checks(funct3 = 0b000, imm = 0)]
    ECALL(ECALL),
    #[checks(funct3 = 0b000, imm = 1)]
    EBREAK(EBREAK),
    #[checks(funct3 = 0b001)]
    CSRRW(CSRRW),
    #[checks(funct3 = 0b010)]
    CSRRS(CSRRS),
    #[checks(funct3 = 0b011)]
    CSRRC(CSRRC),
    #[checks(funct3 = 0b101)]
    CSRRWI(CSRRWI),
    #[checks(funct3 = 0b110)]
    CSRRSI(CSRRSI),
    #[checks(funct3 = 0b111)]
    CSRRCI(CSRRCI),
}

#[derive(Debug, Clone, Copy)]
#[instruction]
#[format(IFormat)]
#[checks(op = 0b1110011, funct3 = 0b111)]
pub struct CSRRCI {
    pub rd: u32,
    pub rs1: u32,
    pub imm: u32,
}

#[derive(Debug, Clone, Copy)]
#[instruction]
#[format(IFormat)]
#[checks(op = 0b1110011, funct3 = 0b110)]
pub struct CSRRSI {
    pub rd: u32,
    pub rs1: u32,
    pub imm: u32,
}

#[derive(Debug, Clone, Copy)]
#[instruction]
#[format(IFormat)]
#[checks(op = 0b1110011, funct3 = 0b101)]
pub struct CSRRWI {
    pub rd: u32,
    pub rs1: u32,
    pub imm: u32,
}

#[derive(Debug, Clone, Copy)]
#[instruction]
#[format(IFormat)]
#[checks(op = 0b1110011, funct3 = 0b011)]
pub struct CSRRC {
    pub rd: u32,
    pub rs1: u32,
    pub imm: u32,
}

#[derive(Debug, Clone, Copy)]
#[instruction]
#[format(IFormat)]
#[checks(op = 0b1110011, funct3 = 0b010)]
pub struct CSRRS {
    pub rd: u32,
    pub rs1: u32,
    pub imm: u32,
}

#[derive(Debug, Clone, Copy)]
#[instruction]
#[format(IFormat)]
#[checks(op = 0b1110011, funct3 = 0b001)]
pub struct CSRRW {
    pub rd: u32,
    pub rs1: u32,
    pub imm: u32,
}

#[derive(Debug, Clone, Copy)]
#[instruction]
#[format(IFormat)]
#[checks(op = 0b1110011, funct3 = 0b000, imm = 0)]
pub struct ECALL {
    pub rd: u32,
}

#[derive(Debug, Clone, Copy)]
#[instruction]
#[format(IFormat)]
#[checks(op = 0b1110011, funct3 = 0b000, imm = 1)]
pub struct EBREAK {
    pub rd: u32,
}
