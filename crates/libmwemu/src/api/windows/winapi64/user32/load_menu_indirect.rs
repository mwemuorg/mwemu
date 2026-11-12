use crate::emu;
use crate::winapi::helper;

pub fn LoadMenuIndirectA(emu: &mut emu::Emu) {
    load_menu_indirect(emu, "LoadMenuIndirectA");
}

fn load_menu_indirect(emu: &mut emu::Emu, api: &str) {
    let template = emu.regs().rcx;
    log_red!(emu, "user32!{api} tmpl:0x{:x}", template);
    let hndl = helper::handler_create("user32://menu");
    emu.regs_mut().rax = hndl;
}

pub fn LoadMenuIndirectW(emu: &mut emu::Emu) {
    load_menu_indirect(emu, "LoadMenuIndirectW");
}
