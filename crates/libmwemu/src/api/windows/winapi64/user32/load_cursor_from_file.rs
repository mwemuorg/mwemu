use crate::emu;
use crate::winapi::helper;

pub fn LoadCursorFromFileA(emu: &mut emu::Emu) {
    load_cursor_from_file(emu, "LoadCursorFromFileA");
}

fn load_cursor_from_file(emu: &mut emu::Emu, api: &str) {
    let arg1 = emu.regs().rcx;
    log_red!(emu, "user32!{api} arg1:0x{:x}", arg1);
    let hndl = helper::handler_create(&format!("user32://{api}"));
    emu.regs_mut().rax = hndl;
}

pub fn LoadCursorFromFileW(emu: &mut emu::Emu) {
    load_cursor_from_file(emu, "LoadCursorFromFileW");
}
