use crate::emu;
use crate::winapi::common::strings;
use crate::windows::constants::errors::*;

pub fn GetPrivateProfileSectionA(emu: &mut emu::Emu) {
    get_private_profile_section(emu, "GetPrivateProfileSectionA", false);
}

fn get_private_profile_section(emu: &mut emu::Emu, api: &str, wide: bool) {
    let lpAppName = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let lpReturnedString = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let _nSize = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
    let lpFileName = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);

    let lpAppName = strings::read_text(emu, lpAppName as u64, wide);
    let lpReturnedString = strings::read_text(emu, lpReturnedString as u64, wide);
    let lpFileName = strings::read_text(emu, lpFileName as u64, wide);

    log_red!(
        emu,
        "kernel32!{api} {} {} {}",
        lpAppName,
        lpReturnedString,
        lpFileName
    );

    emu.regs_mut().rax = ERROR_SUCCESS;

    for _ in 0..4 {
        emu.stack_pop32(false);
    }
}

pub fn GetPrivateProfileSectionW(emu: &mut emu::Emu) {
    get_private_profile_section(emu, "GetPrivateProfileSectionW", true);
}
