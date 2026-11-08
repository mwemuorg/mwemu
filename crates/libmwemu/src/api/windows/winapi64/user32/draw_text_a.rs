use crate::emu;

pub(super) fn DrawTextA(emu: &mut emu::Emu) {
    let text = emu.maps.read_string(emu.regs().rdx);
    log_red!(emu, "user32!DrawTextA '{}'", text);
    emu.regs_mut().rax = 20;
}
