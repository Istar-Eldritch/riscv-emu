use crate::format::RFormat;
use macros::instruction;

#[derive(Debug, Clone, Copy)]
#[instruction]
#[format(RFormat)]
#[checks(op = 0b1110011, funct7 = 0b11000, rs2 = 0b10)]
pub struct MRET {
    rd: u32,
}

#[derive(Debug, Clone, Copy)]
#[instruction]
#[format(RFormat)]
#[checks(op = 0b1110011, funct7 = 0b1000, rs2 = 0b101)]
pub struct WFI {
    rd: u32,
}

#[derive(Debug, Clone, Copy)]
#[instruction]
#[format(RFormat)]
#[checks(op = 0b1110011)]
pub enum RVPrivileged {
    #[checks(funct7 = 0b11000, rs2 = 0b10)]
    MRET(MRET),
    #[checks(funct7 = 0b1000, rs2 = 0b101)]
    WFI(WFI),
}
