use super::ExceptionInterrupt;
use super::Instruction;
use crate::cpu::{CSRs, CPU};
use crate::memory::Memory;
use riscv_isa_types::format::RFormat;
use riscv_isa_types::privileged::RVPrivileged;

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
    let mstatus = mstatus | 1 << 7; // set mpie to 1

    // Reset mip bit
    // XXX: Not sure if this is the right place to be doing this
    let mcause = cpu.get_csr(CSRs::mcause as u32).unwrap();
    if mcause & (1 << 31) > 0 {
        let cause = mcause & !(1 << 31);
        let mip = cpu.get_csr(CSRs::mip as u32).unwrap();
        cpu.set_csr(CSRs::mip as u32, mip & !(cause)).unwrap();
    }

    cpu.set_csr(CSRs::mstatus as u32, mstatus).unwrap();
    Ok(1)
}

fn wfi(cpu: &mut CPU, _parsed: RFormat) -> Result<u32, ExceptionInterrupt> {
    cpu.wfi = true;
    Ok(1)
}
