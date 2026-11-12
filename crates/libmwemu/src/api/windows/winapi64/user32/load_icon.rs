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

pub fn LoadIconW(emu: &mut emu::Emu) {
    let hinstance = emu.regs().rcx;
    let icon_name = emu.regs().rdx;
    log_red!(
        emu,
        "user32!LoadIconW hinst:0x{:x} icon:0x{:x}",
        hinstance,
        icon_name
    );
    let hndl = helper::handler_create("user32://LoadIconW");
    emu.regs_mut().rax = hndl;
}
