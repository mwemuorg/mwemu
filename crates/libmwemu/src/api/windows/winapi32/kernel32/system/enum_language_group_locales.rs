use crate::emu;
use crate::winapi::helper::WinApiAbi;
use crate::winapi::winapi32::stdcall_ret;
use crate::windows::constants::errors::*;

pub fn EnumLanguageGroupLocalesA(emu: &mut emu::Emu) {
    let _lpLangGroupLocaleEnumProc = WinApiAbi::arg(emu, 0) as u32;
    let _LanguageGroup = WinApiAbi::arg(emu, 1) as u32;
    let _dwFlags = WinApiAbi::arg(emu, 2) as u32;
    let _lParam = WinApiAbi::arg(emu, 3) as u32;

    log_red!(emu, "kernel32!EnumLanguageGroupLocalesA");

    emu.regs_mut().rax = ERROR_SUCCESS;

    for _ in 0..4 {
        emu.stack_pop32(false);
    }
}

pub fn EnumLanguageGroupLocalesW(emu: &mut emu::Emu) {
    let str_ptr = WinApiAbi::arg(emu, 0);
    let arg2 = WinApiAbi::arg(emu, 1);
    let s = emu.maps.read_wide_string(str_ptr);
    log_red!(
        emu,
        "kernel32!EnumLanguageGroupLocalesW '{}'  0x{:x}",
        s,
        arg2
    );
    stdcall_ret(emu, 4);
    emu.regs_mut().rax = 1;
}
