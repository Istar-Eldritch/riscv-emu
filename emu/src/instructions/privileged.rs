use super::Instruction;
use crate::{CSRs, ExceptionInterrupt, Memory, CPU};
use riscv_isa_parser::format::RFormat;
use riscv_isa_parser::privileged::RVPrivileged;

impl Instruction for RVPrivileged {
    fn execute(&self, cpu: &mut CPU, _mem: &mut dyn Memory) -> Result<u32, ExceptionInterrupt> {
        use RVPrivileged::*;
        match *self {
            MRET(f) => mret(cpu, f),
            WFI(f) => wfi(cpu, f),
        }
    }

    fn update_pc(&self, cpu: &mut CPU) {
        use RVPrivileged::*;
        match self {
            MRET(_) => (),
            WFI(_) => cpu.pc = cpu.pc + 4,
        }
    }
}

fn mret(cpu: &mut CPU, _parsed: RFormat) -> Result<u32, ExceptionInterrupt> {
    cpu.pc = cpu.get_csr(CSRs::mepc as u32).unwrap();
    // TODO Set MPP privilege mode.
    let mstatus = cpu.get_csr(CSRs::mstatus as u32).unwrap();
    let mstatus = mstatus | ((mstatus & (1 << 7)) >> 4); // recover mie from mpie
    let mstatus = mstatus | 1 << 7; // set mpie to
    cpu.set_csr(CSRs::mstatus as u32, mstatus).unwrap();
    Ok(1)
}

fn wfi(cpu: &mut CPU, _parsed: RFormat) -> Result<u32, ExceptionInterrupt> {
    cpu.wfi = true;
    Ok(1)
}
