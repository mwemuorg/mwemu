use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper::WinApiAbi;

pub fn GetConsoleOriginalTitleA(emu: &mut emu::Emu) {
    get_console_original_title(emu, "GetConsoleOriginalTitleA", false);
}

fn get_console_original_title(emu: &mut emu::Emu, api: &str, wide: bool) {
    let str_ptr = WinApiAbi::arg(emu, 0);
    let arg2 = WinApiAbi::arg(emu, 1);
    let s = strings::read_text(emu, str_ptr, wide);
    log_red!(emu, "kernel32!{api} '{}'  0x{:x}", s, arg2);
    for _ in 0..2 {
        emu.stack_pop32(false);
    }
    emu.regs_mut().rax = 1;
}

pub fn GetConsoleOriginalTitleW(emu: &mut emu::Emu) {
    get_console_original_title(emu, "GetConsoleOriginalTitleW", true);
}
