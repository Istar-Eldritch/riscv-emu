mod branch;
mod immediate;
mod load;
mod miscmem;
mod op;
mod store;
mod system;

pub use branch::*;
pub use immediate::*;
pub use load::*;
pub use miscmem::*;
pub use op::*;
pub use store::*;
pub use system::*;

use crate::format::{IFormat, JFormat, UFormat};
use macros::instruction;

#[derive(Debug, Clone, Copy)]
#[instruction]
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
    JAL(JAL),
    /// Jump and Link Register
    /// Sets the pc to x[rs1] + sign-extend(offset), masking off the least significant bit of the
    /// computed address, then writes the previous pc+4 to x[rd]. If rd is omitted, x1 is asumed;
    JALR(JALR),
    /// Branch operations
    Branch(Branch),
    /// Load operations
    Load(Load),
    /// Store operations
    Store(Store),
    /// Immediate operations
    Immediate(Immediate),
    /// Register to Register operations
    Op(Op),
    /// Fence operations
    MiscMem(MiscMem),
    /// System operations
    System(System),
}

#[derive(Debug, Clone, Copy)]
#[instruction]
#[format(UFormat)]
#[checks(op = 0b0110111)]
pub struct LUI {
    pub rd: u32,
    pub imm: u32,
}

#[derive(Debug, Clone, Copy)]
#[instruction]
#[format(UFormat)]
#[checks(op = 0b0010111)]
pub struct AUIPC {
    pub rd: u32,
    pub imm: u32,
}

#[derive(Debug, Clone, Copy)]
#[instruction]
#[format(JFormat)]
#[checks(op = 0b1101111)]
pub struct JAL {
    pub rd: u32,
    #[format_mapping(imm0 = 12, imm1 = 11, imm2 = 1, imm3 = 20)]
    pub offset: u32,
}

#[derive(Debug, Clone, Copy)]
#[instruction]
#[format(IFormat)]
#[checks(op = 0b1100111)]
pub struct JALR {
    pub rd: u32,
    pub rs1: u32,
    pub imm: u32,
}
