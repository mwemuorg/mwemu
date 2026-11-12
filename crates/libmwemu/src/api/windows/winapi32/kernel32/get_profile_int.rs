use crate::emu;
use crate::winapi::common::strings;
use crate::windows::constants::errors::*;

pub fn GetProfileIntA(emu: &mut emu::Emu) {
    get_profile_int(emu, "GetProfileIntA", false);
}

fn get_profile_int(emu: &mut emu::Emu, api: &str, wide: bool) {
    let lpAppName = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let lpKeyName = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let _nDefault = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);

    let lpAppName = strings::read_text(emu, lpAppName as u64, wide);
    let lpKeyName = strings::read_text(emu, lpKeyName as u64, wide);

    log_red!(emu, "kernel32!{api} {} {}", lpAppName, lpKeyName);

    emu.regs_mut().rax = ERROR_SUCCESS;

    for _ in 0..3 {
        emu.stack_pop32(false);
    }
}

pub fn GetProfileIntW(emu: &mut emu::Emu) {
    get_profile_int(emu, "GetProfileIntW", true);
}
