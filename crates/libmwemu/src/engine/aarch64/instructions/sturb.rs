use crate::emu::Emu;
use yaxpeax_arm::armv8::a64::Instruction;

use super::super::helpers::*;

pub fn execute(emu: &mut Emu, ins: &Instruction) -> bool {
    let val = read_reg(emu, &ins.operands[0]);
    let (addr, wb) = resolve_mem_addr(emu, &ins.operands[1]);
    emu.maps.write_byte(addr, val as u8);
    do_writeback(emu, wb);
    true
}
