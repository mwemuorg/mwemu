use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper::WinApiAbi;

pub fn GetConsoleAliasExesLengthA(emu: &mut emu::Emu) {
    get_console_alias_exes_length(emu, "GetConsoleAliasExesLengthA", false);
}

fn get_console_alias_exes_length(emu: &mut emu::Emu, api: &str, wide: bool) {
    let str_ptr = WinApiAbi::arg(emu, 0);
    let arg2 = WinApiAbi::arg(emu, 1);
    let s = strings::read_text(emu, str_ptr, wide);
    log_red!(emu, "kernel32!{api} '{}'  0x{:x}", s, arg2);

    emu.regs_mut().rax = 1;
}

pub fn GetConsoleAliasExesLengthW(emu: &mut emu::Emu) {
    get_console_alias_exes_length(emu, "GetConsoleAliasExesLengthW", true);
}
