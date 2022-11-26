use macros::mask;

pub const OPCODE_MASK: u32 = mask!(7);
const RD_MASK: u32 = mask!(5) << 7;
const FUNCT3_MASK: u32 = mask!(3) << 12;
const RS1_MASK: u32 = mask!(5) << 15;
const RS2_MASK: u32 = mask!(5) << 20;
const FUNCT7_MASK: u32 = mask!(7) << 25;

pub fn opcode(v: u32) -> u32 {
    v & OPCODE_MASK
}

#[derive(Debug, Clone, Copy)]
pub struct RFormat {
    pub op: u32,
    pub rd: u32,
    pub funct3: u32,
    pub rs1: u32,
    pub rs2: u32,
    pub funct7: u32,
}

impl From<u32> for RFormat {
    fn from(v: u32) -> RFormat {
        RFormat {
            op: v & OPCODE_MASK,
            rd: (v & RD_MASK) >> 7,
            funct3: (v & FUNCT3_MASK) >> 12,
            rs1: (v & RS1_MASK) >> 15,
            rs2: (v & RS2_MASK) >> 20,
            funct7: (v & FUNCT7_MASK) >> 25,
        }
    }
}

impl From<RFormat> for u32 {
    fn from(v: RFormat) -> u32 {
        v.op | v.rd << 7 | v.funct3 << 12 | v.rs1 << 15 | v.rs2 << 20 | v.funct7 << 25
    }
}

#[derive(Clone, Copy)]
pub struct IFormat {
    pub op: u32,
    pub rd: u32,
    pub funct3: u32,
    pub rs1: u32,
    pub imm: u32,
}

impl From<u32> for IFormat {
    fn from(v: u32) -> IFormat {
        const IIMM_MASK: u32 = mask!(12) << 20;
        IFormat {
            op: v & OPCODE_MASK,
            rd: (v & RD_MASK) >> 7,
            funct3: (v & FUNCT3_MASK) >> 12,
            rs1: (v & RS1_MASK) >> 15,
            imm: (v & IIMM_MASK) >> 20,
        }
    }
}

impl From<IFormat> for u32 {
    fn from(v: IFormat) -> u32 {
        v.op | v.rd << 7 | v.funct3 << 12 | v.rs1 << 15 | v.imm << 20
    }
}

#[derive(Clone, Copy)]
pub struct SFormat {
    pub op: u32,
    pub imm0: u32,
    pub funct3: u32,
    pub rs1: u32,
    pub rs2: u32,
    pub imm1: u32,
}

impl From<u32> for SFormat {
    fn from(v: u32) -> SFormat {
        SFormat {
            op: v & OPCODE_MASK,
            imm0: (v & RD_MASK) >> 7,
            funct3: (v & FUNCT3_MASK) >> 12,
            rs1: (v & RS1_MASK) >> 15,
            rs2: (v & RS2_MASK) >> 20,
            imm1: (v & FUNCT7_MASK) >> 25,
        }
    }
}

impl From<SFormat> for u32 {
    fn from(v: SFormat) -> u32 {
        v.op | v.imm0 << 7 | v.funct3 << 12 | v.rs1 << 15 | v.rs2 << 20 | v.imm1 << 25
    }
}

#[derive(Clone, Copy)]
pub struct BFormat {
    pub op: u32,
    pub imm0: u32,
    pub imm1: u32,
    pub funct3: u32,
    pub rs1: u32,
    pub rs2: u32,
    pub imm2: u32,
    pub imm3: u32,
}

impl From<u32> for BFormat {
    fn from(v: u32) -> BFormat {
        BFormat {
            op: v & OPCODE_MASK,
            imm0: (v & (1 << 7)) >> 7,
            imm1: (v & (mask!(4) << 8)) >> 8,
            funct3: (v & FUNCT3_MASK) >> 12,
            rs1: (v & RS1_MASK) >> 15,
            rs2: (v & RS2_MASK) >> 20,
            imm2: (v & (mask!(6) << 25)) >> 25,
            imm3: (v & (1 << 31)) >> 31,
        }
    }
}

impl From<BFormat> for u32 {
    fn from(v: BFormat) -> u32 {
        v.op | v.imm0 << 7
            | v.imm1 << 8
            | v.funct3 << 12
            | v.rs1 << 15
            | v.rs2 << 20
            | v.imm2 << 25
            | v.imm3 << 31
    }
}

#[derive(Clone, Copy)]
pub struct UFormat {
    pub rd: u32,
    pub imm: u32,
}

impl From<u32> for UFormat {
    fn from(v: u32) -> UFormat {
        UFormat {
            rd: (v & RD_MASK) >> 7,
            imm: (v & mask!(20) << 12) >> 12,
        }
    }
}
#[derive(Clone, Copy)]
pub struct JFormat {
    pub rd: u32,
    pub imm0: u32,
    pub imm1: u32,
    pub imm2: u32,
    pub imm3: u32,
}

impl From<u32> for JFormat {
    fn from(v: u32) -> JFormat {
        JFormat {
            rd: (v & RD_MASK) >> 7,
            imm0: (v & mask!(8) << 12) >> 12,
            imm1: (v & 1 << 20) >> 20,
            imm2: (v & mask!(10) << 21) >> 21,
            imm3: (v & 1 << 31) >> 31,
        }
    }
}

#[derive(Debug, PartialEq)]
#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum Opcodes {
    add = 0b0110011,
    addi = 0b0010011,
    lui = 0b0110111,
    sb = 0b0100011,
}

impl TryFrom<u32> for Opcodes {
    type Error = ();
    fn try_from(v: u32) -> Result<Self, Self::Error> {
        use Opcodes::*;
        match v {
            v if v == add as u32 => Ok(add),
            v if v == addi as u32 => Ok(addi),
            v if v == lui as u32 => Ok(lui),
            v if v == sb as u32 => Ok(sb),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn parse_r_type() {
        // add x1, x2, x3
        // 0000000_00011_00010_000_00001_0110011
        let inst = 0x0031_00b3;

        let parsed = RFormat::from(inst);

        assert_eq!(opcode(inst), Opcodes::add as u32, "opcode");
        assert_eq!(parsed.rd, 1, "rd");
        assert_eq!(parsed.rs1, 2, "rs1");
        assert_eq!(parsed.rs2, 3, "rs2");
    }

    #[test]
    fn parse_i_type() {
        // add x1, x2, 0x80
        // 000010000000_00010_000_00001_0010011
        let inst = 0x0801_0093;

        let parsed = IFormat::from(inst);

        assert_eq!(opcode(inst), Opcodes::addi as u32, "opcode");
        assert_eq!(parsed.rd, 1, "rd");
        assert_eq!(parsed.rs1, 2, "rs1");
        assert_eq!(parsed.imm, 0x80, "imm");
    }

    #[test]
    fn parse_s_type() {
        // sb x1, 0x7(x2)
        // 000100010000001110100011
        let inst = 0x0011_03a3;

        let parsed = SFormat::from(inst);

        assert_eq!(opcode(inst), Opcodes::sb as u32, "opcode");
        assert_eq!(parsed.rs1, 2, "rs1");
        assert_eq!(parsed.rs2, 1, "rs2");
        assert_eq!(parsed.imm0, 0x7, "offset");
        assert_eq!(parsed.imm1, 0, "imm");
    }

    #[test]
    fn parse_b_type() {
        // beq x5, x7, _start
        // 0111_00101_000_0000_0_1100011
        let inst = 0x0072_8063;

        let parsed = BFormat::from(inst);

        assert_eq!(opcode(inst), 0b1100011, "opcode");
        assert_eq!(parsed.imm0, 0, "imm0");
        assert_eq!(parsed.imm1, 0, "imm1");
        assert_eq!(parsed.imm2, 0, "imm2");
        assert_eq!(parsed.imm3, 0, "imm3");
        assert_eq!(parsed.rs1, 5, "rs1");
        assert_eq!(parsed.rs2, 7, "rs2");
    }

    #[test]
    fn parse_u_type() {
        // lui x1, 0x90
        // 10010000_00001_0110111
        let inst = 0x0009_00b7;

        let parsed = UFormat::from(inst);

        assert_eq!(opcode(inst), Opcodes::lui as u32, "opcode");
        assert_eq!(parsed.rd, 1, "rd");
        assert_eq!(parsed.imm, 0x90, "imm");
    }

    #[test]
    fn parse_j_type() {
        // jal x1, _start
        // 11101111
        let inst = 0x0000_00ef;

        let parsed = JFormat::from(inst);

        assert_eq!(opcode(inst), 0b1101111, "opcode");
        assert_eq!(parsed.rd, 1, "rd");
        assert_eq!(parsed.imm0, 0, "imm0");
        assert_eq!(parsed.imm1, 0, "imm1");
        assert_eq!(parsed.imm2, 0, "imm2");
        assert_eq!(parsed.imm3, 0, "imm3");
    }
}
