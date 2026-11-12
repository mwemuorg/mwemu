use crate::emu;
use crate::winapi::common::strings;
use crate::windows::constants::errors::*;

pub fn SetFileShortNameA(emu: &mut emu::Emu) {
    set_file_short_name(emu, "SetFileShortNameA", false);
}

fn set_file_short_name(emu: &mut emu::Emu, api: &str, wide: bool) {
    let _hFile = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let lpShortName = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);

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
