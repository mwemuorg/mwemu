use crate::emu::Emu;
use yaxpeax_arm::armv8::a64::{Instruction, Operand, SIMDSizeCode};

use super::super::helpers::*;

pub fn execute(emu: &mut Emu, ins: &Instruction) -> bool {
    // SIMD STR: str q0, [addr] / str d0, [addr] / str s0, [addr]
    if let Operand::SIMDRegister(sz, rs) = ins.operands[0] {
        let (addr, wb) = resolve_mem_addr(emu, &ins.operands[1]);
        let val = emu.regs_aarch64().v[rs as usize];
        let raw = val.to_le_bytes();
        let reg_bytes: usize = match sz {
            SIMDSizeCode::Q => 16,
            SIMDSizeCode::D => 8,
            SIMDSizeCode::S => 4,
            SIMDSizeCode::H => 2,
            SIMDSizeCode::B => 1,
        };
        guard_mem(emu, addr, reg_bytes as u32, true);
        for i in 0..reg_bytes {
            emu.maps.write_byte(addr + i as u64, raw[i]);
        }
        do_writeback(emu, wb);
        return true;
    }

    // GPR STR
    let is64 = operand_is_64(&ins.operands[0]);
    let val = read_reg(emu, &ins.operands[0]);
    let (addr, wb) = resolve_mem_addr(emu, &ins.operands[1]);
    guard_mem(emu, addr, if is64 { 8 } else { 4 }, true);

    if is64 {
        emu.maps.write_qword(addr, val);
    } else {
        emu.maps.write_dword(addr, val as u32);
    }
    do_writeback(emu, wb);
    true
}
