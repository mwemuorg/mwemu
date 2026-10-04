use crate::emu::Emu;
use yaxpeax_arm::armv8::a64::Instruction;

use super::super::helpers::*;

/// LDPSW Xt1, Xt2, [addr]: load two 32-bit words, sign-extended to 64 bits.
pub fn execute(emu: &mut Emu, ins: &Instruction) -> bool {
    let (addr, wb) = resolve_mem_addr(emu, &ins.operands[2]);
    let (Some(v1), Some(v2)) = (emu.maps.read_dword(addr), emu.maps.read_dword(addr + 4)) else {
        log::warn!("LDPSW: cannot read 0x{:x}", addr);
        return false;
    };
    write_reg(emu, &ins.operands[0], v1 as i32 as i64 as u64);
    write_reg(emu, &ins.operands[1], v2 as i32 as i64 as u64);
    do_writeback(emu, wb);
    true
}
