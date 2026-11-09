use crate::emu;

pub(super) fn DrawTextW(emu: &mut emu::Emu) {
    let text = emu.maps.read_wide_string(emu.regs().rdx);
    log_red!(emu, "user32!DrawTextW '{}'", text);
    emu.regs_mut().rax = 20;
}
