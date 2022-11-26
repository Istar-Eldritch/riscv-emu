use super::{format::*, Instruction};
use crate::{CSRs, ExceptionInterrupt, Memory, CPU};

pub enum RVPrivileged {
    MRET(RFormat),
    WFI(RFormat),
}

impl TryFrom<u32> for RVPrivileged {
    type Error = ();

    fn try_from(word: u32) -> Result<Self, ()> {
        use RVPrivileged::*;
        let op = opcode(word);
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

impl Instruction for RVPrivileged {
    fn execute(&self, cpu: &mut CPU, _mem: &mut dyn Memory) -> Result<u32, ExceptionInterrupt> {
        use RVPrivileged::*;
        match *self {
            MRET(f) => mret(cpu, f),
            WFI(f) => wfi(cpu, f),
        }
    }
}

fn mret(cpu: &mut CPU, _parsed: RFormat) -> Result<u32, ExceptionInterrupt> {
    cpu.pc = cpu.get_csr(CSRs::mepc as u32).unwrap();
    // TODO Set MPP privilege mode.
    let mstatus = cpu.get_csr(CSRs::mstatus as u32).unwrap();
    let mstatus = (mstatus & 1 << 2) | (mstatus & 1 << 6) >> 4;
    let mstatus = mstatus | 1 << 6;
    cpu.set_csr(CSRs::mstatus as u32, mstatus).unwrap();
    Ok(1)
}

fn wfi(cpu: &mut CPU, _parsed: RFormat) -> Result<u32, ExceptionInterrupt> {
    cpu.wfi = true;
    Ok(1)
}
