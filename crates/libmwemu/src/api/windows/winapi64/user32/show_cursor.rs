use crate::emu;

pub(super) fn ShowCursor(emu: &mut emu::Emu) {
    let bshow = emu.regs().rcx;
    log_red!(emu, "user32!ShowCursor bShow:{}", bshow);
    emu.regs_mut().rax = if bshow != 0 { 1 } else { 0u64.wrapping_sub(1) };
}
