use crate::emu;

pub(super) fn SetWindowTextA(emu: &mut emu::Emu) {
    let text = emu.maps.read_string(emu.regs().rdx);
    log_red!(emu, "user32!SetWindowTextA '{}'", text);
    emu.regs_mut().rax = 1;
}
