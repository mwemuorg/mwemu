use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper::WinApiAbi;
use crate::windows::constants::errors::*;

pub fn SetFileShortNameA(emu: &mut emu::Emu) {
    set_file_short_name(emu, "SetFileShortNameA", false);
}

fn set_file_short_name(emu: &mut emu::Emu, api: &str, wide: bool) {
    let _hFile = WinApiAbi::arg(emu, 0) as u32;
    let lpShortName = WinApiAbi::arg(emu, 1) as u32;

    let lpShortName = strings::read_text(emu, lpShortName as u64, wide);

    log_red!(emu, "kernel32!{api} {}", lpShortName);

    emu.regs_mut().rax = ERROR_SUCCESS;

    for _ in 0..2 {
        emu.stack_pop32(false);
    }
}

pub fn SetFileShortNameW(emu: &mut emu::Emu) {
    set_file_short_name(emu, "SetFileShortNameW", true);
}
