use crate::emu;

pub(super) fn GetKeyboardLayout(emu: &mut emu::Emu) {
    let arg1 = emu.regs().rcx;
    log_red!(emu, "user32!GetKeyboardLayout arg1:0x{:x}", arg1);
    emu.regs_mut().rax = 0x0409_0409;
}
