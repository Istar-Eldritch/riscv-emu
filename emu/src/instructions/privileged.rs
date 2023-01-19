use super::ExceptionInterrupt;
use super::Instruction;
use crate::cpu::CSRs;
use crate::mcu::MCU;
use riscv_isa_types::format::RFormat;
use riscv_isa_types::privileged::RVPrivileged;

impl Instruction for RVPrivileged {
    fn execute(&self, mcu: &mut MCU) -> Result<u32, ExceptionInterrupt> {
        use RVPrivileged::*;
        match *self {
            MRET(f) => mret(mcu, f),
            WFI(f) => wfi(mcu, f),
        }
    }

    fn update_pc(&self, mcu: &mut MCU) {
        use RVPrivileged::*;
        match self {
            MRET(_) => (),
            WFI(_) => mcu.cpu.pc = mcu.cpu.pc + 4,
        }
    }
}

fn mret(mcu: &mut MCU, _parsed: RFormat) -> Result<u32, ExceptionInterrupt> {
    mcu.cpu.pc = mcu.cpu.get_csr(CSRs::mepc as u32).unwrap();
    // TODO Set MPP privilege mode.
    let mstatus = mcu.cpu.get_csr(CSRs::mstatus as u32).unwrap();
    let mstatus = mstatus | ((mstatus & (1 << 7)) >> 4); // recover mie from mpie
    let mstatus = mstatus | 1 << 7; // set mpie to 1

    mcu.int_ctrl.reset(&mut mcu.cpu);

    mcu.cpu.set_csr(CSRs::mstatus as u32, mstatus).unwrap();
    Ok(1)
}

fn wfi(mcu: &mut MCU, _parsed: RFormat) -> Result<u32, ExceptionInterrupt> {
    mcu.cpu.wfi = true;
    Ok(1)
}
