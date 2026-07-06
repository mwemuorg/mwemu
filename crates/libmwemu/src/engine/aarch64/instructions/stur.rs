use crate::emu::Emu;
use yaxpeax_arm::armv8::a64::Instruction;

use super::super::helpers::*;

pub fn execute(emu: &mut Emu, ins: &Instruction) -> bool {
    let is64 = operand_is_64(&ins.operands[0]);
    let val = read_reg(emu, &ins.operands[0]);
    let (addr, wb) = resolve_mem_addr(emu, &ins.operands[1]);

    if is64 {
        emu.maps.write_qword(addr, val);
    } else {
        emu.maps.write_dword(addr, val as u32);
    }
    do_writeback(emu, wb);
    true
}
