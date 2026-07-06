use crate::emu::Emu;
use yaxpeax_arm::armv8::a64::Instruction;

use super::super::helpers::*;

pub fn execute(emu: &mut Emu, ins: &Instruction) -> bool {
    let is64 = operand_is_64(&ins.operands[0]);
    let sz: u64 = if is64 { 8 } else { 4 };
    let (addr, wb) = resolve_mem_addr(emu, &ins.operands[2]);

    let v1 = if is64 {
        match emu.maps.read_qword(addr) {
            Some(v) => v,
            None => return false,
        }
    } else {
        match emu.maps.read_dword(addr) {
            Some(v) => v as u64,
            None => return false,
        }
    };
    let v2 = if is64 {
        match emu.maps.read_qword(addr + sz) {
            Some(v) => v,
            None => return false,
        }
    } else {
        match emu.maps.read_dword(addr + sz) {
            Some(v) => v as u64,
            None => return false,
        }
    };

    write_reg(emu, &ins.operands[0], v1);
    write_reg(emu, &ins.operands[1], v2);
    do_writeback(emu, wb);
    true
}
