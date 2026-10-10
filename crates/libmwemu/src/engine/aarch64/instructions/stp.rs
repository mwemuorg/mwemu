use crate::emu::Emu;
use yaxpeax_arm::armv8::a64::{Instruction, Operand, SIMDSizeCode};

use super::super::helpers::*;

pub fn execute(emu: &mut Emu, ins: &Instruction) -> bool {
    // SIMD STP: stp q0, q1, [addr]
    if let Operand::SIMDRegister(sz, r1) = ins.operands[0]
        && let Operand::SIMDRegister(_, r2) = ins.operands[1]
    {
        let (addr, wb) = resolve_mem_addr(emu, &ins.operands[2]);
        let v1 = emu.regs_aarch64().v[r1 as usize];
        let v2 = emu.regs_aarch64().v[r2 as usize];
        let reg_bytes: u64 = match sz {
            SIMDSizeCode::Q => 16,
            SIMDSizeCode::D => 8,
            SIMDSizeCode::S => 4,
            _ => 16,
        };
        guard_mem(emu, addr, (reg_bytes * 2) as u32, true);
        write_simd_to_mem(emu, addr, v1, reg_bytes);
        write_simd_to_mem(emu, addr + reg_bytes, v2, reg_bytes);
        do_writeback(emu, wb);
        return true;
    }

    // GPR STP
    let is64 = operand_is_64(&ins.operands[0]);
    let sz: u64 = if is64 { 8 } else { 4 };
    let v1 = read_reg(emu, &ins.operands[0]);
    let v2 = read_reg(emu, &ins.operands[1]);
    let (addr, wb) = resolve_mem_addr(emu, &ins.operands[2]);
    guard_mem(emu, addr, (sz * 2) as u32, true);

    if is64 {
        emu.maps.write_qword(addr, v1);
        emu.maps.write_qword(addr + sz, v2);
    } else {
        emu.maps.write_dword(addr, v1 as u32);
        emu.maps.write_dword(addr + sz, v2 as u32);
    }
    do_writeback(emu, wb);
    true
}

fn write_simd_to_mem(emu: &mut Emu, addr: u64, val: u128, bytes: u64) {
    let raw = val.to_le_bytes();
    for i in 0..bytes as usize {
        emu.maps.write_byte(addr + i as u64, raw[i]);
    }
}
