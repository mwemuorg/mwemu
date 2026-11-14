use crate::emu;
use crate::winapi::common::strings;

pub fn ChangeDisplaySettingsA(emu: &mut emu::Emu) {
    change_display_settings(emu, "ChangeDisplaySettingsA", false);
}

fn change_display_settings(emu: &mut emu::Emu, api: &str, wide: bool) {
    let p0 = emu.regs().rcx;
    let p1 = emu.regs().rdx;
    let s = if p0 != 0 && p0 > 0xffff {
        strings::read_text(emu, p0, wide)
    } else {
        format!("0x{:x}", p0)
    };
    log_red!(emu, "user32!{api} '{}' p1:0x{:x}", s, p1);
}

pub fn ChangeDisplaySettingsW(emu: &mut emu::Emu) {
    change_display_settings(emu, "ChangeDisplaySettingsW", true);
}
