use macros::mask;

const OPCODE_MASK: u32 = mask!(7);
const RD_MASK: u32 = mask!(5) << 8;
const FUNCT3_MASK: u32 = mask!(3) << 13;
const RS1_MASK: u32 = mask!(5) << 16;
const RS2_MASK: u32 = mask!(5) << 21;
const FUNCT7_MASK: u32 = mask!(7) << 26;

pub struct RFormat {
    opcode: u32,
    rd: u32,
    funct3: u32,
    rs1: u32,
    rs2: u32,
    funct7: u32,
}

impl From<u32> for RFormat {
    fn from(v: u32) -> RFormat {
        RFormat {
            opcode: v & OPCODE_MASK,
            rd: (v & RD_MASK) >> 7,
            funct3: (v & FUNCT3_MASK) >> 12,
            rs1: (v & RS1_MASK) >> 15,
            rs2: (v & RS2_MASK) >> 20,
            funct7: (v & FUNCT7_MASK) >> 25,
        }
    }
}

pub struct IFormat {
    opcode: u32,
    rd: u32,
    funct3: u32,
    rs1: u32,
    imm: u32,
}

impl From<u32> for IFormat {
    fn from(v: u32) -> IFormat {
        const IIMM_MASK: u32 = mask!(12) << 20;
        IFormat {
            opcode: v & OPCODE_MASK,
            rd: (v & RD_MASK) >> 7,
            funct3: (v & FUNCT3_MASK) >> 12,
            rs1: (v & RS1_MASK) >> 15,
            imm: (v & IIMM_MASK) >> 20,
        }
    }
}

pub struct SFormat {
    opcode: u32,
    imm0: u32,
    funct3: u32,
    rs1: u32,
    rs2: u32,
    imm1: u32,
}

impl From<u32> for SFormat {
    fn from(v: u32) -> SFormat {
        SFormat {
            opcode: v & OPCODE_MASK,
            imm0: (v & RD_MASK) >> 7,
            funct3: (v & FUNCT3_MASK) >> 12,
            rs1: (v & RS1_MASK) >> 15,
            rs2: (v & RS2_MASK) >> 20,
            imm1: (v & FUNCT7_MASK) >> 25,
        }
    }
}

pub struct BFormat {
    opcode: u32,
    imm0: u32,
    imm1: u32,
    funct3: u32,
    rs1: u32,
    rs2: u32,
    imm4: u32,
    imm5: u32,
}

impl From<u32> for BFormat {
    fn from(v: u32) -> BFormat {
        BFormat {
            opcode: v & OPCODE_MASK,
            imm0: (v & (1 << 7)) >> 7,
            imm1: (v & (mask!(4) << 8)) >> 8,
            funct3: (v & FUNCT3_MASK) >> 12,
            rs1: (v & RS1_MASK) >> 15,
            rs2: (v & RS2_MASK) >> 20,
            imm4: (v & (mask!(6) << 25)) >> 25,
            imm5: (v & (1 << 31)) >> 31,
        }
    }
}

pub struct UFormat {
    opcode: u32,
    rd: u32,
    imm: u32,
}

impl From<u32> for UFormat {
    fn from(v: u32) -> UFormat {
        UFormat {
            opcode: v & OPCODE_MASK,
            rd: (v & RD_MASK) >> 7,
            imm: (v & mask!(20) << 12) >> 12,
        }
    }
}

pub struct JFormat {
    opcode: u32,
    rd: u32,
    imm0: u32,
    imm1: u32,
    imm2: u32,
    imm3: u32,
}

impl From<u32> for JFormat {
    fn from(v: u32) -> JFormat {
        JFormat {
            opcode: v & OPCODE_MASK,
            rd: (v & RD_MASK) >> 7,
            imm0: (v & mask!(8) << 12) >> 12,
            imm1: (v & 1 << 20) >> 20,
            imm2: (v & mask!(10) << 21) >> 21,
            imm3: (v & 1 << 31) >> 31,
        }
    }
}
