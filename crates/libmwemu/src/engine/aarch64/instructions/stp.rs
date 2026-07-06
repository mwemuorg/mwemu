use crate::emu::Emu;
use yaxpeax_arm::armv8::a64::Instruction;

use super::super::helpers::*;

pub fn execute(emu: &mut Emu, ins: &Instruction) -> bool {
    let is64 = operand_is_64(&ins.operands[0]);
    let sz: u64 = if is64 { 8 } else { 4 };
    let v1 = read_reg(emu, &ins.operands[0]);
    let v2 = read_reg(emu, &ins.operands[1]);
    let (addr, wb) = resolve_mem_addr(emu, &ins.operands[2]);

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
