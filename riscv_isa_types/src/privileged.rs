use crate::format::{RFormat, OPCODE_MASK};

#[derive(Debug)]
pub enum RVPrivileged {
    MRET(RFormat),
    WFI(RFormat),
}

impl TryFrom<u32> for RVPrivileged {
    type Error = ();

    fn try_from(word: u32) -> Result<Self, ()> {
        use RVPrivileged::*;
        let op = word & OPCODE_MASK;
        if op == 0b1110011 {
            let parsed = RFormat::from(word);
            match parsed.funct7 {
                0b11000 if parsed.rs2 == 0b10 => Ok(MRET(parsed)),
                0b1000 if parsed.rs2 == 0b101 => Ok(WFI(parsed)),
                _ => Err(()),
            }
        } else {
            Err(())
        }
    }
}

impl From<RVPrivileged> for u32 {
    fn from(inst: RVPrivileged) -> u32 {
        use RVPrivileged::*;
        match inst {
            MRET(i) => i.into(),
            WFI(i) => i.into(),
        }
    }
}
