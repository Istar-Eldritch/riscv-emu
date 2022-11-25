use super::format::*;
use crate::{CSRs, Memory, CPU};

pub fn privileged(cpu: &mut CPU, _mem: &mut dyn Memory, word: u32) -> Option<u32> {
    let op = opcode(word);
    match op {
        0b1110011 if (word & (111 << 11)) == 0 => {
            // privileged instructions
            let parsed = RFormat::from(word);
            match parsed.funct7 {
                // mret
                0b11000 if parsed.rs2 == 0b10 => {
                    cpu.pc = cpu.get_csr(CSRs::mepc as u32).unwrap();
                    // TODO Set MPP privilege mode.
                    let mstatus = cpu.get_csr(CSRs::mstatus as u32).unwrap();
                    let mstatus = (mstatus & 1 << 2) | (mstatus & 1 << 6) >> 4;
                    let mstatus = mstatus | 1 << 6;
                    cpu.set_csr(CSRs::mstatus as u32, mstatus).unwrap();
                    Some(0)
                }
                // wfi
                0b1000 if parsed.rs2 == 0b101 => {
                    cpu.wfi = true;
                    Some(0)
                }
                _ => None,
            }
        }
        _ => None,
    }
}
