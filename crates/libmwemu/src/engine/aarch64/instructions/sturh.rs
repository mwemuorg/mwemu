use crate::emu::Emu;
use yaxpeax_arm::armv8::a64::Instruction;

use super::super::helpers::*;

pub fn execute(emu: &mut Emu, ins: &Instruction) -> bool {
    let val = read_reg(emu, &ins.operands[0]);
    let (addr, wb) = resolve_mem_addr(emu, &ins.operands[1]);
    emu.maps.write_word(addr, val as u16);
    do_writeback(emu, wb);
    true
}
