use crate::emu::Emu;
use yaxpeax_arm::armv8::a64::Instruction;

use super::super::helpers::*;

pub fn execute(emu: &mut Emu, ins: &Instruction) -> bool {
    // STXR Ws, Xt, [Xn] — exclusive store, always succeeds in emulation
    let is64 = operand_is_64(&ins.operands[1]);
    let val = read_reg(emu, &ins.operands[1]);
    let (addr, wb) = resolve_mem_addr(emu, &ins.operands[2]);

    if is64 {
        emu.maps.write_qword(addr, val);
    } else {
        emu.maps.write_dword(addr, val as u32);
    }
    // Status register = 0 (success)
    write_reg(emu, &ins.operands[0], 0);
    do_writeback(emu, wb);
    true
}
