use crate::emu;
use crate::winapi::helper;

pub fn LoadBitmapA(emu: &mut emu::Emu) {
    let hinst = emu.regs().rcx;
    let name = emu.regs().rdx;
    log_red!(
        emu,
        "user32!LoadBitmapA hInst:0x{:x} name:0x{:x}",
        hinst,
        name
    );
    let hndl = helper::handler_create("user32://bitmap");
    emu.regs_mut().rax = hndl;
}

pub fn LoadBitmapW(emu: &mut emu::Emu) {
    let hinst = emu.regs().rcx;
    let name = emu.regs().rdx;
    log_red!(
        emu,
        "user32!LoadBitmapW hInst:0x{:x} name:0x{:x}",
        hinst,
        name
    );
    let hndl = helper::handler_create("user32://bitmap");
    emu.regs_mut().rax = hndl;
}
