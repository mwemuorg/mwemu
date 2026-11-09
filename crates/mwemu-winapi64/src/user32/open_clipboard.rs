use crate::emu;
use crate::winapi::helper;

pub fn OpenClipboard(emu: &mut emu::Emu) {
    let hwnd_owner = emu.regs().rcx;
    let hndl = helper::handler_create("clipboard");
    log_red!(
        emu,
        "user32!OpenClipboard owner:0x{:x} => 0x{:x}",
        hwnd_owner,
        hndl
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
