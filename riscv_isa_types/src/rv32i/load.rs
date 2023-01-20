use crate::format::IFormat;
use macros::instruction;

#[derive(Debug, Clone, Copy)]
#[instruction]
#[format(IFormat)]
#[checks(op = 0b0000011, funct3 = 0b000)]
pub struct LB {
    pub rd: u32,
    pub rs1: u32,
    pub imm: u32,
}

#[derive(Debug, Clone, Copy)]
#[instruction]
#[format(IFormat)]
#[checks(op = 0b0000011, funct3 = 0b100)]
pub struct LBU {
    pub rd: u32,
    pub rs1: u32,
    pub imm: u32,
}

#[derive(Debug, Clone, Copy)]
#[instruction]
#[format(IFormat)]
#[checks(op = 0b0000011, funct3 = 0b001)]
pub struct LH {
    pub rd: u32,
    pub rs1: u32,
    pub imm: u32,
}

#[derive(Debug, Clone, Copy)]
#[instruction]
#[format(IFormat)]
#[checks(op = 0b0000011, funct3 = 0b101)]
pub struct LHU {
    pub rd: u32,
    pub rs1: u32,
    pub imm: u32,
}

#[derive(Debug, Clone, Copy)]
#[instruction]
#[format(IFormat)]
#[checks(op = 0b0000011, funct3 = 0b010)]
pub struct LW {
    pub rd: u32,
    pub rs1: u32,
    pub imm: u32,
}

#[derive(Debug, Clone, Copy)]
#[instruction]
#[format(IFormat)]
#[checks(op = 0b0000011, funct3 = 0b110)]
pub struct LWU {
    pub rd: u32,
    pub rs1: u32,
    pub imm: u32,
}

#[derive(Debug, Clone, Copy)]
#[instruction]
#[format(IFormat)]
#[checks(op = 0b0000011)]
pub enum Load {
    #[checks(funct3 = 0b000)]
    LB(LB),
    #[checks(funct3 = 0b100)]
    LBU(LBU),
    #[checks(funct3 = 0b001)]
    LH(LH),
    #[checks(funct3 = 0b101)]
    LHU(LHU),
    #[checks(funct3 = 0b010)]
    LW(LWU),
    #[checks(funct3 = 0b110)]
    LWU(LWU),
}

// fn load(word: u32) -> Result<RV32i, ()> {
//     use RV32i::*;
//     let parsed = IFormat::from(word);
//     match parsed.funct3 {
//         0b101 => Ok(LHU(parsed)),
//         0b010 => Ok(LW(parsed)),
//         0b110 => Ok(LWU(parsed)),
//         _ => Err(()),
//     }
// }
