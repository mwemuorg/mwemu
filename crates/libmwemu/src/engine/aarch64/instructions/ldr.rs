use crate::emu::Emu;
use yaxpeax_arm::armv8::a64::Instruction;

use super::super::helpers::*;

pub fn execute(emu: &mut Emu, ins: &Instruction) -> bool {
    let is64 = operand_is_64(&ins.operands[0]);
    let (addr, wb) = resolve_mem_addr(emu, &ins.operands[1]);
    let val = if is64 {
        match emu.maps.read_qword(addr) {
            Some(v) => v,
            None => {
                log::warn!("LDR: cannot read 8 bytes at 0x{:x}", addr);
                return false;
            }
        }
    } else {
        match emu.maps.read_dword(addr) {
            Some(v) => v as u64,
            None => {
                log::warn!("LDR: cannot read 4 bytes at 0x{:x}", addr);
                return false;
            }
        }
    };
    write_reg(emu, &ins.operands[0], val);
    do_writeback(emu, wb);
    true
}
