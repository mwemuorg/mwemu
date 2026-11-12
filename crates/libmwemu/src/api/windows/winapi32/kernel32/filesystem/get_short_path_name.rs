use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper::WinApiAbi;
use crate::windows::constants::errors::*;

pub fn GetShortPathNameA(emu: &mut emu::Emu) {
    get_short_path_name(emu, "GetShortPathNameA", false);
}

fn get_short_path_name(emu: &mut emu::Emu, api: &str, wide: bool) {
    let lpszLongPath = WinApiAbi::arg(emu, 0) as u32;
    let lpszShortPath = WinApiAbi::arg(emu, 1) as u32;
    let _cchBuffer = WinApiAbi::arg(emu, 2) as u32;

    let lpszLongPath = strings::read_text(emu, lpszLongPath as u64, wide);
    let lpszShortPath = strings::read_text(emu, lpszShortPath as u64, wide);

    log_red!(emu, "kernel32!{api} {} {}", lpszLongPath, lpszShortPath);

    emu.regs_mut().rax = ERROR_SUCCESS;

    for _ in 0..3 {
        emu.stack_pop32(false);
    }
}

pub fn GetShortPathNameW(emu: &mut emu::Emu) {
    get_short_path_name(emu, "GetShortPathNameW", true);
}
