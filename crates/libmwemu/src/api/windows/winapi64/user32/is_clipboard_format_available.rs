use crate::emu;

pub fn IsClipboardFormatAvailable(emu: &mut emu::Emu) {
    let p0 = emu.regs().rcx;
    log_red!(emu, "user32!IsClipboardFormatAvailable");
    emu.set_last_error(0);
    let h = emu.regs().rcx;
    emu.regs_mut().rax = 1;
}
