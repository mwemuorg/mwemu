use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper::WinApiAbi;

pub fn SetConsoleTitleA(emu: &mut emu::Emu) {
    set_console_title(emu, "SetConsoleTitleA", false);
}

fn set_console_title(emu: &mut emu::Emu, api: &str, wide: bool) {
    let p0 = WinApiAbi::arg(emu, 0);
    let s = if p0 != 0 && p0 > 0xffff {
        strings::read_text(emu, p0, wide)
    } else {
        format!("0x{:x}", p0)
    };
    log_red!(emu, "kernel32!{api} '{}'", s);
    for _ in 0..1 {
        emu.stack_pop32(false);
    }
    emu.regs_mut().rax = 1;
}

pub fn SetConsoleTitleW(emu: &mut emu::Emu) {
    set_console_title(emu, "SetConsoleTitleW", true);
}
