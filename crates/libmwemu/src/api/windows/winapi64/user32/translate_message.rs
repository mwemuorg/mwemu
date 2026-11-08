use crate::emu;

pub(super) fn TranslateMessage(emu: &mut emu::Emu) {
    let arg1 = emu.regs().rcx;
    log_red!(emu, "user32!TranslateMessage arg1:0x{:x}", arg1);
    emu.regs_mut().rax = 0;
}
