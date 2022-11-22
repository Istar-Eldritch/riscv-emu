use super::{Memory, CPU};
use crate::format::*;
use crate::utils::*;

#[derive(Debug)]
pub enum CPUError {
    UnrecognizedInstruction,
}

/// Load Upper Immediate
fn lui(cpu: &mut CPU, mem: &mut Memory, word: u32) -> Result<(), CPUError> {
    let parsed = UFormat::try_from(word).unwrap(); // TODO: Handle trap
    cpu.x_registers[parsed.rd as usize] = sext(parsed.imm << 12, 20, 32);
    Ok(())
}

/// Add Upper Immediate to PC
fn auipc(cpu: &mut CPU, mem: &mut Memory, word: u32) -> Result<(), CPUError> {
    let parsed = UFormat::try_from(word).unwrap();
    cpu.x_registers[parsed.rd as usize] = cpu.pc + sext(parsed.imm << 12, 20, 32);
    Ok(())
}

/// Jump and Link
fn jal(cpu: &mut CPU, mem: &mut Memory, word: u32) -> Result<(), CPUError> {
    let parsed = JFormat::try_from(word).unwrap();
    cpu.x_registers[parsed.rd as usize] = cpu.pc + 4;
    cpu.pc += sext(
        parsed.imm0 << 11 | parsed.imm1 << 10 | parsed.imm2 | parsed.imm3 << 19,
        20,
        32,
    );
    Ok(())
}

/// Jump and Link Register
fn jalr(cpu: &mut CPU, mem: &mut Memory, word: u32) -> Result<(), CPUError> {
    let parsed = IFormat::try_from(word).unwrap();
    cpu.pc = (cpu.x_registers[parsed.rs1 as usize] + sext(parsed.imm, 12, 32)) & !1;
    let addr = if parsed.rd == 0 { 1 } else { parsed.rd };
    cpu.x_registers[addr as usize] = cpu.pc + 4;
    Ok(())
}

fn beq(cpu: &mut CPU, mem: &mut Memory, word: BFormat) -> Result<(), CPUError> {
    if cpu.x_registers[word.rs1 as usize] == cpu.x_registers[word.rs2 as usize] {
        cpu.pc += sext(
            word.imm0 << 10 | word.imm1 | word.imm2 << 4 | word.imm3 << 11,
            12,
            32,
        );
    }
    Ok(())
}

fn bge(cpu: &mut CPU, mem: &mut Memory, word: BFormat) -> Result<(), CPUError> {
    if cpu.x_registers[word.rs1 as usize] as i32 >= cpu.x_registers[word.rs2 as usize] as i32 {
        cpu.pc += sext(
            word.imm0 << 10 | word.imm1 | word.imm2 << 4 | word.imm3 << 11,
            12,
            32,
        );
    }
    Ok(())
}

fn bgeu(cpu: &mut CPU, mem: &mut Memory, word: BFormat) -> Result<(), CPUError> {
    if cpu.x_registers[word.rs1 as usize] >= cpu.x_registers[word.rs2 as usize] {
        cpu.pc += sext(
            word.imm0 << 10 | word.imm1 | word.imm2 << 4 | word.imm3 << 11,
            12,
            32,
        );
    }
    Ok(())
}

fn blt(cpu: &mut CPU, mem: &mut Memory, word: BFormat) -> Result<(), CPUError> {
    if (cpu.x_registers[word.rs1 as usize] as i32) < cpu.x_registers[word.rs2 as usize] as i32 {
        cpu.pc += sext(
            word.imm0 << 10 | word.imm1 | word.imm2 << 4 | word.imm3 << 11,
            12,
            32,
        );
    }
    Ok(())
}

fn bltu(cpu: &mut CPU, mem: &mut Memory, word: BFormat) -> Result<(), CPUError> {
    if cpu.x_registers[word.rs1 as usize] < cpu.x_registers[word.rs2 as usize] {
        cpu.pc += sext(
            word.imm0 << 10 | word.imm1 | word.imm2 << 4 | word.imm3 << 11,
            12,
            32,
        );
    }

    Ok(())
}

fn bne(cpu: &mut CPU, mem: &mut Memory, word: BFormat) -> Result<(), CPUError> {
    if cpu.x_registers[word.rs1 as usize] != cpu.x_registers[word.rs2 as usize] {
        cpu.pc += sext(
            word.imm0 << 10 | word.imm1 | word.imm2 << 4 | word.imm3 << 11,
            12,
            32,
        );
    }
    Ok(())
}

/// Branch parsing
fn branch(cpu: &mut CPU, mem: &mut Memory, word: u32) -> Result<(), CPUError> {
    let parsed = BFormat::try_from(word).unwrap();
    let op = match parsed.funct3 {
        0b000 => beq,
        0b101 => bge,                                       // bge
        0b111 => bgeu,                                      // bgeu
        0b100 => blt,                                       // blt
        0b110 => bltu,                                      // bltu
        0b001 => bne,                                       // bne
        _ => return Err(CPUError::UnrecognizedInstruction), // TODO: Trap
    };
    op(cpu, mem, parsed)
}

fn lb(cpu: &mut CPU, mem: &mut Memory, parsed: IFormat) -> Result<(), CPUError> {
    let addr = cpu.x_registers[parsed.rs1 as usize] + sext(parsed.imm, 12, 32);
    cpu.x_registers[parsed.rd as usize] = sext(mem.rb(addr as usize) as u32, 8, 32);
    Ok(())
}

fn lbu(cpu: &mut CPU, mem: &mut Memory, parsed: IFormat) -> Result<(), CPUError> {
    let addr = cpu.x_registers[parsed.rs1 as usize] + sext(parsed.imm, 12, 32);
    cpu.x_registers[parsed.rd as usize] = mem.rb(addr as usize) as u32;
    Ok(())
}

fn lh(cpu: &mut CPU, mem: &mut Memory, parsed: IFormat) -> Result<(), CPUError> {
    let addr = cpu.x_registers[parsed.rs1 as usize] + sext(parsed.imm, 12, 32);
    cpu.x_registers[parsed.rd as usize] = sext(mem.rhw(addr as usize) as u32, 16, 32);
    Ok(())
}

fn lhu(cpu: &mut CPU, mem: &mut Memory, parsed: IFormat) -> Result<(), CPUError> {
    let addr = cpu.x_registers[parsed.rs1 as usize] + sext(parsed.imm, 12, 32);
    cpu.x_registers[parsed.rd as usize] = mem.rhw(addr as usize) as u32;
    Ok(())
}

fn lw(cpu: &mut CPU, mem: &mut Memory, parsed: IFormat) -> Result<(), CPUError> {
    let addr = cpu.x_registers[parsed.rs1 as usize] + sext(parsed.imm, 12, 32);
    cpu.x_registers[parsed.rd as usize] = sext(mem.rw(addr as usize) as u32, 32, 32);
    Ok(())
}

fn lwu(cpu: &mut CPU, mem: &mut Memory, parsed: IFormat) -> Result<(), CPUError> {
    let addr = cpu.x_registers[parsed.rs1 as usize] + sext(parsed.imm, 12, 32);
    cpu.x_registers[parsed.rd as usize] = mem.rw(addr as usize) as u32;
    Ok(())
}

fn load(cpu: &mut CPU, mem: &mut Memory, word: u32) -> Result<(), CPUError> {
    let parsed = IFormat::try_from(word).unwrap();
    let op = match parsed.funct3 {
        0b000 => lb,
        0b100 => lbu,
        0b001 => lh,
        0b101 => lhu,
        0b010 => lw,
        0b110 => lwu,
        _ => return Err(CPUError::UnrecognizedInstruction),
    };
    op(cpu, mem, parsed)
}

fn sb(cpu: &mut CPU, mem: &mut Memory, parsed: SFormat) -> Result<(), CPUError> {
    let addr = cpu.x_registers[parsed.rs1 as usize] + sext(parsed.imm0 | parsed.imm1, 12, 32);
    mem.wb(addr as usize, cpu.x_registers[parsed.rs2 as usize] as u8);
    Ok(())
}

fn sh(cpu: &mut CPU, mem: &mut Memory, parsed: SFormat) -> Result<(), CPUError> {
    let addr = cpu.x_registers[parsed.rs1 as usize] + sext(parsed.imm0 | parsed.imm1, 12, 32);
    mem.whw(addr as usize, cpu.x_registers[parsed.rs2 as usize] as u16);
    Ok(())
}

fn sw(cpu: &mut CPU, mem: &mut Memory, parsed: SFormat) -> Result<(), CPUError> {
    let addr = cpu.x_registers[parsed.rs1 as usize] + sext(parsed.imm0 | parsed.imm1, 12, 32);
    mem.ww(addr as usize, cpu.x_registers[parsed.rs2 as usize]);
    Ok(())
}

fn store(cpu: &mut CPU, mem: &mut Memory, word: u32) -> Result<(), CPUError> {
    let parsed = SFormat::try_from(word).unwrap();
    let op = match parsed.funct3 {
        0b000 => sb,
        0b001 => sh,
        0b010 => sw,
        _ => return Err(CPUError::UnrecognizedInstruction),
    };
    op(cpu, mem, parsed)
}

pub fn rv32i(cpu: &mut CPU, mem: &mut Memory, word: u32) -> Result<(), CPUError> {
    let opcode = word & OPCODE_MASK;
    let op = match opcode {
        0b0110111 => lui,
        0b0010111 => auipc,
        0b1101111 => jal,
        0b1100111 => jalr,
        0b1100011 => branch,  // (beq, bne, blt, bge, bltu, bgeu)
        0b0000011 => load,    // TODO fn3 (lb, lh, lw, lbu, lhu)
        0b0100011 => store,   // TODO fn3 (sb, sh, sw)
        0b0010011 => todo!(), //immediate, // TODO fn3 (adi, slti, sltiu, xori, ori, andi, slli, srli, srai)
        0b0110011 => todo!(), // TODO fn3 (add, sub, sll, slt, sltu, xor, srl, sra, or, and)
        0b0001111 => todo!(), // TODO fn3 (fence, fence.i)
        0b1110011 => todo!(), // TODO fn3 (ecall, ebreak, csrrw, csrrs, csrrc, csrrwi, csrrsi, csrrci),
        _ => return Err(CPUError::UnrecognizedInstruction), // TODO: Should not panic. Return Result.
    };

    op(cpu, mem, word)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lui_writes_register() {
        let mut cpu = CPU::new();
        let mut mem = Memory::new(1024 * 100);
        // lui x1, 0x23
        let word = 0x0002_20b7;
        lui(&mut cpu, &mut mem, word);
        assert_eq!(cpu.x_registers[1], 0x22 << 12);
    }
}
