use crate::emu;
use crate::winapi::helper;

pub fn EnumClipboardFormats(emu: &mut emu::Emu) {
    let format = emu.regs().rcx;
    log_red!(emu, "user32!EnumClipboardFormats format:{}", format);
    helper::advance_tick(emu, 1);
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}
