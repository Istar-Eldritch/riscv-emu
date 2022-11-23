use super::{Memory, CPU};
use crate::format::*;
use crate::utils::*;
use macros::mask;

#[derive(Debug)]
pub enum CPUException {
    UnrecognizedInstruction,
    EnvironmentCall,
    Breakpoint,
}

/// Load Upper Immediate
fn lui(cpu: &mut CPU, mem: &mut Memory, word: u32) -> Result<(), CPUException> {
    let parsed = UFormat::try_from(word).unwrap(); // TODO: Handle trap
    cpu.x[parsed.rd as usize] = sext(parsed.imm << 12, 20, 32);
    Ok(())
}

/// Add Upper Immediate to PC
fn auipc(cpu: &mut CPU, mem: &mut Memory, word: u32) -> Result<(), CPUException> {
    let parsed = UFormat::try_from(word).unwrap();
    cpu.x[parsed.rd as usize] = cpu.pc + sext(parsed.imm << 12, 20, 32);
    Ok(())
}

/// Jump and Link
fn jal(cpu: &mut CPU, mem: &mut Memory, word: u32) -> Result<(), CPUException> {
    let parsed = JFormat::try_from(word).unwrap();
    cpu.x[parsed.rd as usize] = cpu.pc + 4;
    cpu.pc += sext(
        parsed.imm0 << 11 | parsed.imm1 << 10 | parsed.imm2 | parsed.imm3 << 19,
        20,
        32,
    );
    Ok(())
}

/// Jump and Link Register
fn jalr(cpu: &mut CPU, mem: &mut Memory, word: u32) -> Result<(), CPUException> {
    let parsed = IFormat::try_from(word).unwrap();
    cpu.pc = (cpu.x[parsed.rs1 as usize] + sext(parsed.imm, 12, 32)) & !1;
    let addr = if parsed.rd == 0 { 1 } else { parsed.rd };
    cpu.x[addr as usize] = cpu.pc + 4;
    Ok(())
}

fn beq(cpu: &mut CPU, mem: &mut Memory, word: BFormat) -> Result<(), CPUException> {
    if cpu.x[word.rs1 as usize] == cpu.x[word.rs2 as usize] {
        cpu.pc += sext(
            word.imm0 << 10 | word.imm1 | word.imm2 << 4 | word.imm3 << 11,
            12,
            32,
        );
    }
    Ok(())
}

fn bge(cpu: &mut CPU, mem: &mut Memory, word: BFormat) -> Result<(), CPUException> {
    if cpu.x[word.rs1 as usize] as i32 >= cpu.x[word.rs2 as usize] as i32 {
        cpu.pc += sext(
            word.imm0 << 10 | word.imm1 | word.imm2 << 4 | word.imm3 << 11,
            12,
            32,
        );
    }
    Ok(())
}

fn bgeu(cpu: &mut CPU, mem: &mut Memory, word: BFormat) -> Result<(), CPUException> {
    if cpu.x[word.rs1 as usize] >= cpu.x[word.rs2 as usize] {
        cpu.pc += sext(
            word.imm0 << 10 | word.imm1 | word.imm2 << 4 | word.imm3 << 11,
            12,
            32,
        );
    }
    Ok(())
}

fn blt(cpu: &mut CPU, mem: &mut Memory, word: BFormat) -> Result<(), CPUException> {
    if (cpu.x[word.rs1 as usize] as i32) < cpu.x[word.rs2 as usize] as i32 {
        cpu.pc += sext(
            word.imm0 << 10 | word.imm1 | word.imm2 << 4 | word.imm3 << 11,
            12,
            32,
        );
    }
    Ok(())
}

fn bltu(cpu: &mut CPU, mem: &mut Memory, word: BFormat) -> Result<(), CPUException> {
    if cpu.x[word.rs1 as usize] < cpu.x[word.rs2 as usize] {
        cpu.pc += sext(
            word.imm0 << 10 | word.imm1 | word.imm2 << 4 | word.imm3 << 11,
            12,
            32,
        );
    }

    Ok(())
}

fn bne(cpu: &mut CPU, mem: &mut Memory, word: BFormat) -> Result<(), CPUException> {
    if cpu.x[word.rs1 as usize] != cpu.x[word.rs2 as usize] {
        cpu.pc += sext(
            word.imm0 << 10 | word.imm1 | word.imm2 << 4 | word.imm3 << 11,
            12,
            32,
        );
    }
    Ok(())
}

/// Branch parsing
fn branch(cpu: &mut CPU, mem: &mut Memory, word: u32) -> Result<(), CPUException> {
    let parsed = BFormat::try_from(word).unwrap();
    let op = match parsed.funct3 {
        0b000 => beq,
        0b101 => bge,                                           // bge
        0b111 => bgeu,                                          // bgeu
        0b100 => blt,                                           // blt
        0b110 => bltu,                                          // bltu
        0b001 => bne,                                           // bne
        _ => return Err(CPUException::UnrecognizedInstruction), // TODO: Trap
    };
    op(cpu, mem, parsed)
}

fn lb(cpu: &mut CPU, mem: &mut Memory, parsed: IFormat) -> Result<(), CPUException> {
    let addr = cpu.x[parsed.rs1 as usize] + sext(parsed.imm, 12, 32);
    cpu.x[parsed.rd as usize] = sext(mem.rb(addr as usize) as u32, 8, 32);
    Ok(())
}

fn lbu(cpu: &mut CPU, mem: &mut Memory, parsed: IFormat) -> Result<(), CPUException> {
    let addr = cpu.x[parsed.rs1 as usize] + sext(parsed.imm, 12, 32);
    cpu.x[parsed.rd as usize] = mem.rb(addr as usize) as u32;
    Ok(())
}

fn lh(cpu: &mut CPU, mem: &mut Memory, parsed: IFormat) -> Result<(), CPUException> {
    let addr = cpu.x[parsed.rs1 as usize] + sext(parsed.imm, 12, 32);
    cpu.x[parsed.rd as usize] = sext(mem.rhw(addr as usize) as u32, 16, 32);
    Ok(())
}

fn lhu(cpu: &mut CPU, mem: &mut Memory, parsed: IFormat) -> Result<(), CPUException> {
    let addr = cpu.x[parsed.rs1 as usize] + sext(parsed.imm, 12, 32);
    cpu.x[parsed.rd as usize] = mem.rhw(addr as usize) as u32;
    Ok(())
}

fn lw(cpu: &mut CPU, mem: &mut Memory, parsed: IFormat) -> Result<(), CPUException> {
    let addr = cpu.x[parsed.rs1 as usize] + sext(parsed.imm, 12, 32);
    cpu.x[parsed.rd as usize] = sext(mem.rw(addr as usize) as u32, 32, 32);
    Ok(())
}

fn lwu(cpu: &mut CPU, mem: &mut Memory, parsed: IFormat) -> Result<(), CPUException> {
    let addr = cpu.x[parsed.rs1 as usize] + sext(parsed.imm, 12, 32);
    cpu.x[parsed.rd as usize] = mem.rw(addr as usize) as u32;
    Ok(())
}

fn load(cpu: &mut CPU, mem: &mut Memory, word: u32) -> Result<(), CPUException> {
    let parsed = IFormat::try_from(word).unwrap();
    let op = match parsed.funct3 {
        0b000 => lb,
        0b100 => lbu,
        0b001 => lh,
        0b101 => lhu,
        0b010 => lw,
        0b110 => lwu,
        _ => return Err(CPUException::UnrecognizedInstruction),
    };
    op(cpu, mem, parsed)
}

fn sb(cpu: &mut CPU, mem: &mut Memory, parsed: SFormat) -> Result<(), CPUException> {
    let addr = cpu.x[parsed.rs1 as usize] + sext(parsed.imm0 | parsed.imm1, 12, 32);
    mem.wb(addr as usize, cpu.x[parsed.rs2 as usize] as u8);
    Ok(())
}

fn sh(cpu: &mut CPU, mem: &mut Memory, parsed: SFormat) -> Result<(), CPUException> {
    let addr = cpu.x[parsed.rs1 as usize] + sext(parsed.imm0 | parsed.imm1, 12, 32);
    mem.whw(addr as usize, cpu.x[parsed.rs2 as usize] as u16);
    Ok(())
}

fn sw(cpu: &mut CPU, mem: &mut Memory, parsed: SFormat) -> Result<(), CPUException> {
    let addr = cpu.x[parsed.rs1 as usize] + sext(parsed.imm0 | parsed.imm1, 12, 32);
    mem.ww(addr as usize, cpu.x[parsed.rs2 as usize]);
    Ok(())
}

fn store(cpu: &mut CPU, mem: &mut Memory, word: u32) -> Result<(), CPUException> {
    let parsed = SFormat::try_from(word).unwrap();
    let op = match parsed.funct3 {
        0b000 => sb,
        0b001 => sh,
        0b010 => sw,
        _ => return Err(CPUException::UnrecognizedInstruction),
    };
    op(cpu, mem, parsed)
}

fn addi(cpu: &mut CPU, _mem: &mut Memory, parsed: IFormat) -> Result<(), CPUException> {
    cpu.x[parsed.rd as usize] = cpu.x[parsed.rs1 as usize].wrapping_add(sext(parsed.imm, 12, 32));
    Ok(())
}

fn slti(cpu: &mut CPU, _mem: &mut Memory, parsed: IFormat) -> Result<(), CPUException> {
    let v = if (cpu.x[parsed.rs1 as usize] as i32) < sext(parsed.imm, 12, 32) as i32 {
        1
    } else {
        0
    };

    cpu.x[parsed.rd as usize] = v;

    Ok(())
}

fn sltiu(cpu: &mut CPU, _mem: &mut Memory, parsed: IFormat) -> Result<(), CPUException> {
    let v = if cpu.x[parsed.rs1 as usize] < sext(parsed.imm, 12, 32) {
        1
    } else {
        0
    };

    cpu.x[parsed.rd as usize] = v;

    Ok(())
}

fn xori(cpu: &mut CPU, _mem: &mut Memory, parsed: IFormat) -> Result<(), CPUException> {
    cpu.x[parsed.rd as usize] =
        cpu.x[parsed.rs1 as usize] ^ sext(cpu.x[parsed.imm as usize], 12, 32);

    Ok(())
}

fn ori(cpu: &mut CPU, _mem: &mut Memory, parsed: IFormat) -> Result<(), CPUException> {
    cpu.x[parsed.rd as usize] =
        cpu.x[parsed.rs1 as usize] | sext(cpu.x[parsed.imm as usize], 12, 32);

    Ok(())
}

fn andi(cpu: &mut CPU, _mem: &mut Memory, parsed: IFormat) -> Result<(), CPUException> {
    cpu.x[parsed.rd as usize] =
        cpu.x[parsed.rs1 as usize] & sext(cpu.x[parsed.imm as usize], 12, 32);

    Ok(())
}

fn slli(cpu: &mut CPU, _mem: &mut Memory, parsed: IFormat) -> Result<(), CPUException> {
    let shamt = parsed.imm & 0b11111;
    // TODO: shamt[5] should be 0, otherwise is an illegal instruction
    cpu.x[parsed.rd as usize] = cpu.x[parsed.rs1 as usize].wrapping_shl(shamt);

    Ok(())
}

fn srli(cpu: &mut CPU, _mem: &mut Memory, parsed: IFormat) -> Result<(), CPUException> {
    let shamt = parsed.imm & 0b11111;
    // TODO: shamt[5] should be 0, otherwise is an illegal instruction
    cpu.x[parsed.rd as usize] = cpu.x[parsed.rs1 as usize].wrapping_shr(shamt);

    Ok(())
}

fn srai(cpu: &mut CPU, _mem: &mut Memory, parsed: IFormat) -> Result<(), CPUException> {
    let shamt = parsed.imm & 0b11111;
    let rs1 = cpu.x[parsed.rs1 as usize];
    // TODO: shamt[5] should be 0, otherwise is an illegal instruction
    cpu.x[parsed.rd as usize] = (rs1 as i32).wrapping_shr(shamt) as u32;

    Ok(())
}

fn immediate(cpu: &mut CPU, mem: &mut Memory, word: u32) -> Result<(), CPUException> {
    let parsed = IFormat::try_from(word).unwrap();
    let op = match parsed.funct3 {
        0b000 => addi,
        0b010 => slti,
        0b011 => sltiu,
        0b100 => xori,
        0b110 => ori,
        0b111 => andi,
        0b001 => slli,
        0b101 if parsed.imm & (0b111111 << 11) == 0 => srli,
        0b101 => srai,
        _ => return Err(CPUException::UnrecognizedInstruction),
    };
    op(cpu, mem, parsed)
}

fn add(cpu: &mut CPU, mem: &mut Memory, parsed: RFormat) -> Result<(), CPUException> {
    cpu.x[parsed.rd as usize] = cpu.x[parsed.rs1 as usize].wrapping_add(cpu.x[parsed.rs2 as usize]);
    Ok(())
}

fn sub(cpu: &mut CPU, mem: &mut Memory, parsed: RFormat) -> Result<(), CPUException> {
    cpu.x[parsed.rd as usize] = cpu.x[parsed.rs1 as usize].wrapping_sub(cpu.x[parsed.rs2 as usize]);
    Ok(())
}

fn sll(cpu: &mut CPU, mem: &mut Memory, parsed: RFormat) -> Result<(), CPUException> {
    cpu.x[parsed.rd as usize] =
        cpu.x[parsed.rs1 as usize].wrapping_shl(cpu.x[parsed.rs2 as usize] & 0b11111);
    Ok(())
}

fn slt(cpu: &mut CPU, mem: &mut Memory, parsed: RFormat) -> Result<(), CPUException> {
    let v = if (cpu.x[parsed.rs1 as usize] as i32) < cpu.x[parsed.rs2 as usize] as i32 {
        1
    } else {
        0
    };

    cpu.x[parsed.rd as usize] = v;
    Ok(())
}

fn sltu(cpu: &mut CPU, mem: &mut Memory, parsed: RFormat) -> Result<(), CPUException> {
    let v = if cpu.x[parsed.rs1 as usize] < cpu.x[parsed.rs2 as usize] {
        1
    } else {
        0
    };

    cpu.x[parsed.rd as usize] = v;
    Ok(())
}

fn xor(cpu: &mut CPU, mem: &mut Memory, parsed: RFormat) -> Result<(), CPUException> {
    cpu.x[parsed.rd as usize] = cpu.x[parsed.rs1 as usize] ^ cpu.x[parsed.rs2 as usize];
    Ok(())
}

fn srl(cpu: &mut CPU, mem: &mut Memory, parsed: RFormat) -> Result<(), CPUException> {
    cpu.x[parsed.rd as usize] =
        cpu.x[parsed.rs1 as usize].wrapping_shr(cpu.x[parsed.rs2 as usize] & 0b11111);
    Ok(())
}

fn sra(cpu: &mut CPU, _mem: &mut Memory, parsed: RFormat) -> Result<(), CPUException> {
    let shamt = cpu.x[parsed.rs2 as usize] & 0b11111;
    let rs1 = cpu.x[parsed.rs1 as usize];
    cpu.x[parsed.rd as usize] = (rs1 as i32).wrapping_shr(shamt) as u32;

    Ok(())
}

fn or(cpu: &mut CPU, mem: &mut Memory, parsed: RFormat) -> Result<(), CPUException> {
    cpu.x[parsed.rd as usize] = cpu.x[parsed.rs1 as usize] | cpu.x[parsed.rs2 as usize];
    Ok(())
}

fn and(cpu: &mut CPU, mem: &mut Memory, parsed: RFormat) -> Result<(), CPUException> {
    cpu.x[parsed.rd as usize] = cpu.x[parsed.rs1 as usize] & cpu.x[parsed.rs2 as usize];
    Ok(())
}

fn arithmetic(cpu: &mut CPU, mem: &mut Memory, word: u32) -> Result<(), CPUException> {
    let parsed = RFormat::from(word);
    let op = match parsed.funct3 {
        0b000 if parsed.funct7 == 0 => add,
        0b000 => sub,
        0b001 => sll,
        0b010 => slt,
        0b011 => sltu,
        0b100 => xor,
        0b101 => srl,
        0b110 => or,
        0b111 => and,
        _ => return Err(CPUException::UnrecognizedInstruction),
    };
    op(cpu, mem, parsed)
}

fn fence(cpu: &mut CPU, mem: &mut Memory, word: u32) -> Result<(), CPUException> {
    // No need to do anything given we do not reorder memroy writes / loads
    Ok(())
}

fn csrrw(cpu: &mut CPU, mem: &mut Memory, parsed: IFormat) -> Result<(), CPUException> {
    let t = cpu.csr[parsed.imm as usize];
    cpu.csr[parsed.imm as usize] = cpu.x[parsed.rs1 as usize] as u16;
    cpu.x[parsed.rd as usize] = t as u32;
    Ok(())
}

fn csrrs(cpu: &mut CPU, mem: &mut Memory, parsed: IFormat) -> Result<(), CPUException> {
    let t = cpu.csr[parsed.imm as usize];
    cpu.csr[parsed.imm as usize] = cpu.x[parsed.rs1 as usize] | t;
    cpu.x[parsed.rd as usize] = t as u32;
    Ok(())
}

fn csrrc(cpu: &mut CPU, mem: &mut Memory, parsed: IFormat) -> Result<(), CPUException> {
    let t = cpu.csr[parsed.imm as usize];
    cpu.csr[parsed.imm as usize] = t & !cpu.x[parsed.rs1 as usize];
    cpu.x[parsed.rd as usize] = t as u32;
    Ok(())
}

fn csrrwi(cpu: &mut CPU, mem: &mut Memory, parsed: IFormat) -> Result<(), CPUException> {
    cpu.x[parsed.rd as usize] = cpu.csr[parsed.imm as usize] as u32;
    cpu.csr[parsed.imm as usize] = cpu.x[parsed.rs1 as usize] & 0b11111;
    Ok(())
}

fn csrrsi(cpu: &mut CPU, mem: &mut Memory, parsed: IFormat) -> Result<(), CPUException> {
    let t = cpu.csr[parsed.imm as usize];
    cpu.csr[parsed.imm as usize] = cpu.x[parsed.rs1 as usize] & 0b11111 | t;
    cpu.x[parsed.rd as usize] = t as u32;
    Ok(())
}

fn csrrci(cpu: &mut CPU, mem: &mut Memory, parsed: IFormat) -> Result<(), CPUException> {
    let t = cpu.csr[parsed.imm as usize];
    cpu.csr[parsed.imm as usize] = t & !(cpu.x[parsed.rs1 as usize] & 0b11111);
    cpu.x[parsed.rd as usize] = t as u32;
    Ok(())
}

fn system(cpu: &mut CPU, mem: &mut Memory, word: u32) -> Result<(), CPUException> {
    let parsed = IFormat::from(word);

    let op = match parsed.funct3 {
        0b000 if parsed.imm == 0 => return Err(CPUException::EnvironmentCall),
        0b000 if parsed.imm == 1 => return Err(CPUException::Breakpoint),
        0b001 => csrrw,
        0b010 => csrrs,
        0b011 => csrrc,
        0b101 => csrrwi,
        0b110 => csrrsi,
        0b111 => csrrci,
        _ => return Err(CPUException::UnrecognizedInstruction),
    };

    op(cpu, mem, parsed)
}

pub fn rv32i(cpu: &mut CPU, mem: &mut Memory, word: u32) -> Result<(), CPUException> {
    let opcode = word & OPCODE_MASK;
    let op = match opcode {
        0b0110111 => lui,
        0b0010111 => auipc,
        0b1101111 => jal,
        0b1100111 => jalr,
        0b1100011 => branch,
        0b0000011 => load,
        0b0100011 => store,
        0b0010011 => immediate,
        0b0110011 => arithmetic,
        0b0001111 => fence,
        0b1110011 => system,
        _ => return Err(CPUException::UnrecognizedInstruction),
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
        assert_eq!(cpu.x[1], 0x22 << 12);
    }
}
