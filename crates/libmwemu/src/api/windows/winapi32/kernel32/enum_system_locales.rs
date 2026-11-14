use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper::WinApiAbi;

pub fn EnumSystemLocalesA(emu: &mut emu::Emu) {
    enum_system_locales(emu, "EnumSystemLocalesA", false);
}

fn enum_system_locales(emu: &mut emu::Emu, api: &str, wide: bool) {
    let p0 = WinApiAbi::arg(emu, 0);
    let p1 = WinApiAbi::arg(emu, 1);
    let s = if p0 != 0 && p0 > 0xffff {
        strings::read_text(emu, p0, wide)
    } else {
        format!("0x{:x}", p0)
    };
    log_red!(emu, "kernel32!{api} '{}' p1:0x{:x}", s, p1);
    for _ in 0..2 {
        emu.stack_pop32(false);
    }
    emu.regs_mut().rax = 1;
}

pub fn EnumSystemLocalesW(emu: &mut emu::Emu) {
    enum_system_locales(emu, "EnumSystemLocalesW", true);
}
