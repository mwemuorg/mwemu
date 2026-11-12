use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper::WinApiAbi;

pub fn TermsrvGetWindowsDirectoryA(emu: &mut emu::Emu) {
    termsrv_get_windows_directory(emu, "TermsrvGetWindowsDirectoryA", false);
}

fn termsrv_get_windows_directory(emu: &mut emu::Emu, api: &str, wide: bool) {
    let str_ptr = WinApiAbi::arg(emu, 0);
    let arg2 = WinApiAbi::arg(emu, 1);
    let s = strings::read_text(emu, str_ptr, wide);
    log_red!(emu, "kernel32!{api} '{}'  0x{:x}", s, arg2);
    for _ in 0..2 {
        emu.stack_pop32(false);
    }
    emu.regs_mut().rax = 1;
}

pub fn TermsrvGetWindowsDirectoryW(emu: &mut emu::Emu) {
    termsrv_get_windows_directory(emu, "TermsrvGetWindowsDirectoryW", true);
}
