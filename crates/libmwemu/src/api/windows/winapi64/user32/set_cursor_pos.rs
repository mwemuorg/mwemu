use crate::emu;

pub(super) fn SetCursorPos(emu: &mut emu::Emu) {
    let x = emu.regs().rcx;
    let y = emu.regs().rdx;
    log_red!(emu, "user32!SetCursorPos x:{} y:{}", x, y);
    emu.regs_mut().rax = 1;
}
