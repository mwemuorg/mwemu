use crate::emu;

pub(super) fn GetKeyboardType(emu: &mut emu::Emu) {
    let ntype = emu.regs().rcx as i32;
    let result: u64 = match ntype {
        0 => 4,
        1 => 0,
        2 => 12,
        _ => 0,
    };
    log_red!(emu, "user32!GetKeyboardType type:{} = {}", ntype, result);
    emu.regs_mut().rax = result;
}
