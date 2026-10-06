use crate::emu::Emu;
use yaxpeax_arm::armv8::a64::{Instruction, Opcode, Operand};

use super::super::helpers::*;

pub fn execute(emu: &mut Emu, ins: &Instruction) -> bool {
    // RET {Xn} — default is X30 (LR)
    let raw = if matches!(ins.operands[0], Operand::Nothing) {
        emu.regs_aarch64().x[30]
    } else {
        read_reg(emu, &ins.operands[0])
    };
    let target = match ins.opcode {
        Opcode::RETAA | Opcode::RETAB => pac_strip(raw),
        _ => raw,
    };
    emu.force_reload = true;
    if is_sentinel(emu, target) {
        return emu.set_pc_aarch64(target);
    }
    emu.regs_aarch64_mut().pc = target;
    true
}

/// Returning into a macOS sentinel (end of `main` or of a thread routine)
/// must be intercepted; ordinary returns never land in the library range.
fn is_sentinel(emu: &Emu, target: u64) -> bool {
    target >= crate::windows::constants::LIBS64_MIN
        && emu.maps.get_addr_name(target) == Some(crate::macosapi::SENTINEL_MAP)
}
