use crate::emu;
use crate::winapi::common::strings;
use crate::windows::constants::errors::*;

pub fn GetShortPathNameA(emu: &mut emu::Emu) {
    get_short_path_name(emu, "GetShortPathNameA", false);
}

fn get_short_path_name(emu: &mut emu::Emu, api: &str, wide: bool) {
    let lpszLongPath = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let lpszShortPath = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let _cchBuffer = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);

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
