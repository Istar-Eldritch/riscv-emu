use crate::format::RFormat;

#[derive(Debug)]
pub struct MRET;

impl MRET {
    const OP: u32 = 0b1110011;
    const FUNC7: u32 = 0b11000;
    const RS2: u32 = 0b10;
}

impl TryFrom<RFormat> for MRET {
    type Error = ();
    fn try_from(f: RFormat) -> Result<MRET, Self::Error> {
        if f.op == MRET::OP && f.funct7 == MRET::FUNC7 && f.rs2 == MRET::RS2 {
            Ok(MRET)
        } else {
            Err(())
        }
    }
}

impl From<MRET> for RFormat {
    fn from(_: MRET) -> RFormat {
        RFormat {
            op: MRET::OP,
            rd: 0,
            funct3: 0,
            funct7: MRET::FUNC7,
            rs1: 0,
            rs2: MRET::RS2,
        }
    }
}

impl TryFrom<u32> for MRET {
    type Error = ();
    fn try_from(n: u32) -> Result<MRET, Self::Error> {
        MRET::try_from(RFormat::from(n))
    }
}

impl From<MRET> for u32 {
    fn from(i: MRET) -> u32 {
        RFormat::from(i).into()
    }
}

#[derive(Debug)]
pub struct WFI;

impl WFI {
    const OP: u32 = 0b1110011;
    const FUNC7: u32 = 0b1000;
    const RS2: u32 = 0b101;
}

impl TryFrom<RFormat> for WFI {
    type Error = ();
    fn try_from(f: RFormat) -> Result<WFI, Self::Error> {
        if f.op == WFI::OP && f.funct7 == WFI::FUNC7 && f.rs2 == WFI::RS2 {
            Ok(WFI)
        } else {
            Err(())
        }
    }
}

impl From<WFI> for RFormat {
    fn from(_: WFI) -> RFormat {
        RFormat {
            op: WFI::OP,
            rd: 0,
            funct3: 0,
            funct7: WFI::FUNC7,
            rs1: 0,
            rs2: WFI::RS2,
        }
    }
}

impl TryFrom<u32> for WFI {
    type Error = ();
    fn try_from(n: u32) -> Result<WFI, Self::Error> {
        WFI::try_from(RFormat::from(n))
    }
}

impl From<WFI> for u32 {
    fn from(i: WFI) -> u32 {
        RFormat::from(i).into()
    }
}

#[derive(Debug)]
pub enum RVPrivileged {
    MRET(MRET),
    WFI(WFI),
}

impl TryFrom<u32> for RVPrivileged {
    type Error = ();

    fn try_from(word: u32) -> Result<Self, ()> {
        let parsed = RFormat::from(word);
        if let Ok(mret) = MRET::try_from(parsed) {
            Ok(RVPrivileged::MRET(mret))
        } else if let Ok(wfi) = WFI::try_from(parsed) {
            Ok(RVPrivileged::WFI(wfi))
        } else {
            Err(())
        }
    }
}

impl From<RVPrivileged> for u32 {
    fn from(inst: RVPrivileged) -> u32 {
        use RVPrivileged::*;

        match inst {
            MRET(i) => Into::<RFormat>::into(i).into(),
            WFI(i) => Into::<RFormat>::into(i).into(),
        }
    }
}
