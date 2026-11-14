use crate::emu;
use crate::winapi::helper;

pub fn LoadMenuA(emu: &mut emu::Emu) {
    load_menu(emu, "LoadMenuA");
}

fn load_menu(emu: &mut emu::Emu, api: &str) {
    let hinst = emu.regs().rcx;
    log_red!(emu, "user32!{api} hInst:0x{:x}", hinst);
    let hndl = helper::handler_create("user32://menu");
    emu.regs_mut().rax = hndl;
}

pub fn LoadMenuW(emu: &mut emu::Emu) {
    load_menu(emu, "LoadMenuW");
}
