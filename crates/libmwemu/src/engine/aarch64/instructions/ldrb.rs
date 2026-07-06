use crate::emu::Emu;
use yaxpeax_arm::armv8::a64::Instruction;

use super::super::helpers::*;

pub fn execute(emu: &mut Emu, ins: &Instruction) -> bool {
    let (addr, wb) = resolve_mem_addr(emu, &ins.operands[1]);
    let val = match emu.maps.read_byte(addr) {
        Some(v) => v as u64,
        None => {
            log::warn!("LDRB: cannot read at 0x{:x}", addr);
            return false;
        }
    };
    write_reg(emu, &ins.operands[0], val);
    do_writeback(emu, wb);
    true
}
