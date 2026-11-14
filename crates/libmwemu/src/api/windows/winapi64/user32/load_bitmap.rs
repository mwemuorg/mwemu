use crate::emu;
use crate::winapi::helper;

pub fn LoadBitmapA(emu: &mut emu::Emu) {
    load_bitmap(emu, "LoadBitmapA");
}

fn load_bitmap(emu: &mut emu::Emu, api: &str) {
    let hinst = emu.regs().rcx;
    let name = emu.regs().rdx;
    log_red!(emu, "user32!{api} hInst:0x{:x} name:0x{:x}", hinst, name);
    let hndl = helper::handler_create("user32://bitmap");
    emu.regs_mut().rax = hndl;
}

pub fn LoadBitmapW(emu: &mut emu::Emu) {
    load_bitmap(emu, "LoadBitmapW");
}
