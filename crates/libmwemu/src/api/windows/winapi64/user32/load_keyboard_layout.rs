use crate::emu;
use crate::winapi::common::strings;

pub fn LoadKeyboardLayoutA(emu: &mut emu::Emu) {
    load_keyboard_layout(emu, "LoadKeyboardLayoutA", false);
}

fn load_keyboard_layout(emu: &mut emu::Emu, api: &str, wide: bool) {
    let p0 = emu.regs().rcx;
    let p1 = emu.regs().rdx;
    let s = if p0 != 0 && p0 > 0xffff {
        strings::read_text(emu, p0, wide)
    } else {
        format!("0x{:x}", p0)
    };
    log_red!(emu, "user32!{api} '{}'  p1:0x{:x}", s, p1);
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}

pub fn LoadKeyboardLayoutW(emu: &mut emu::Emu) {
    load_keyboard_layout(emu, "LoadKeyboardLayoutW", true);
}
