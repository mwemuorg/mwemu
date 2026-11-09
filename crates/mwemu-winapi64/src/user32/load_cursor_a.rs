use crate::emu;
use crate::winapi::helper;

pub(super) fn LoadCursorA(emu: &mut emu::Emu) {
    let hinstance = emu.regs().rcx;
    let lpcursorname = emu.regs().rdx;
    log_red!(
        emu,
        "user32!LoadCursorA hInst:0x{:x} name:0x{:x}",
        hinstance,
        lpcursorname
    );
    emu.regs_mut().rax = helper::handler_create("user32://cursor");
}
