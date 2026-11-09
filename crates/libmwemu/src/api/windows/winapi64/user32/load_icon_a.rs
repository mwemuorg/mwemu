use crate::emu;
use crate::winapi::helper;

pub(super) fn LoadIconA(emu: &mut emu::Emu) {
    let hinstance = emu.regs().rcx;
    let lpiconname = emu.regs().rdx;
    log_red!(
        emu,
        "user32!LoadIconA hInst:0x{:x} name:0x{:x}",
        hinstance,
        lpiconname
    );
    emu.regs_mut().rax = helper::handler_create("user32://icon");
}
