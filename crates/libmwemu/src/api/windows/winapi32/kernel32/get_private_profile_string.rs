use crate::emu;
use crate::winapi::common::strings;
use crate::windows::constants::errors::*;

pub fn GetPrivateProfileStringA(emu: &mut emu::Emu) {
    get_private_profile_string(emu, "GetPrivateProfileStringA", false);
}

fn get_private_profile_string(emu: &mut emu::Emu, api: &str, wide: bool) {
    let lpAppName = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let lpKeyName = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let _lpDefault = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
    let lpReturnedString = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
    let _nSize = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0);
    let lpFileName = emu.maps.read_dword(emu.regs().get_esp() + 20).unwrap_or(0);

    let lpAppName = strings::read_text(emu, lpAppName as u64, wide);
    let lpKeyName = strings::read_text(emu, lpKeyName as u64, wide);
    let lpReturnedString = strings::read_text(emu, lpReturnedString as u64, wide);
    let lpFileName = strings::read_text(emu, lpFileName as u64, wide);

    log_red!(
        emu,
        "kernel32!{api} {} {} {} {}",
        lpAppName,
        lpKeyName,
        lpReturnedString,
        lpFileName
    );

    emu.regs_mut().rax = ERROR_SUCCESS;

    for _ in 0..6 {
        emu.stack_pop32(false);
    }
}

pub fn GetPrivateProfileStringW(emu: &mut emu::Emu) {
    get_private_profile_string(emu, "GetPrivateProfileStringW", true);
}
