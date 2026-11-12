use crate::emu;
use crate::winapi::common::strings;
use crate::windows::constants::errors::*;

pub fn GetProfileStringA(emu: &mut emu::Emu) {
    get_profile_string(emu, "GetProfileStringA", false);
}

fn get_profile_string(emu: &mut emu::Emu, api: &str, wide: bool) {
    let lpAppName = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let lpKeyName = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let _lpDefault = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
    let lpReturnedString = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
    let _nSize = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0);

    let lpAppName = strings::read_text(emu, lpAppName as u64, wide);
    let lpKeyName = strings::read_text(emu, lpKeyName as u64, wide);
    let lpReturnedString = strings::read_text(emu, lpReturnedString as u64, wide);

    log_red!(
        emu,
        "kernel32!{api} {} {} {}",
        lpAppName,
        lpKeyName,
        lpReturnedString
    );

    emu.regs_mut().rax = ERROR_SUCCESS;

    for _ in 0..5 {
        emu.stack_pop32(false);
    }
}

pub fn GetProfileStringW(emu: &mut emu::Emu) {
    get_profile_string(emu, "GetProfileStringW", true);
}
