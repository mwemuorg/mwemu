use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper::WinApiAbi;
use crate::winapi::winapi32::stdcall_ret;

pub fn ExpungeConsoleCommandHistoryA(emu: &mut emu::Emu) {
    expunge_console_command_history(emu, "ExpungeConsoleCommandHistoryA", false);
}

fn expunge_console_command_history(emu: &mut emu::Emu, api: &str, wide: bool) {
    let str_ptr = WinApiAbi::arg(emu, 0);
    let arg2 = WinApiAbi::arg(emu, 1);
    let s = strings::read_text(emu, str_ptr, wide);
    log_red!(emu, "kernel32!{api} '{}'  0x{:x}", s, arg2);
    stdcall_ret(emu, 1);
    emu.regs_mut().rax = 1;
}

pub fn ExpungeConsoleCommandHistoryW(emu: &mut emu::Emu) {
    expunge_console_command_history(emu, "ExpungeConsoleCommandHistoryW", true);
}
