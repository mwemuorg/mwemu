use crate::emu;
use crate::winapi::common::strings;
use crate::windows::constants::errors::*;

pub fn GetPrivateProfileSectionNamesA(emu: &mut emu::Emu) {
    get_private_profile_section_names(emu, "GetPrivateProfileSectionNamesA", false);
}

fn get_private_profile_section_names(emu: &mut emu::Emu, api: &str, wide: bool) {
    let _lpszReturnBuffer = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let _nSize = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let lpFileName = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);

    let lpFileName = strings::read_text(emu, lpFileName as u64, wide);

    log_red!(emu, "kernel32!{api} {}", lpFileName);

    emu.regs_mut().rax = ERROR_SUCCESS;

    for _ in 0..3 {
        emu.stack_pop32(false);
    }
}

pub fn GetPrivateProfileSectionNamesW(emu: &mut emu::Emu) {
    get_private_profile_section_names(emu, "GetPrivateProfileSectionNamesW", true);
}
