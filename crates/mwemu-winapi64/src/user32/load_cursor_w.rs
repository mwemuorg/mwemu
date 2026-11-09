use crate::emu;
use crate::winapi::helper;

pub fn LoadCursorW(emu: &mut emu::Emu) {
    let hinstance = emu.regs().rcx;
    let cursor_name = emu.regs().rdx;
    log_red!(
        emu,
        "user32!LoadCursorW hinst:0x{:x} cursor:0x{:x}",
        hinstance,
        cursor_name
    );
    let hndl = helper::handler_create("user32://LoadCursorW");
    emu.regs_mut().rax = hndl;
}
