use crate::emu;
use crate::winapi::common::strings;
use crate::windows::constants::errors::*;

pub fn WritePrivateProfileStructA(emu: &mut emu::Emu) {
    write_private_profile_struct(emu, "WritePrivateProfileStructA", false);
}

fn write_private_profile_struct(emu: &mut emu::Emu, api: &str, wide: bool) {
    let _lpszSection = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let _lpszKey = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let lpStruct = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
    let uSizeStruct = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
    let _szFile = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0);

    let lpStruct = strings::read_text(emu, lpStruct as u64, wide);
    let uSizeStruct = strings::read_text(emu, uSizeStruct as u64, wide);

    log_red!(emu, "kernel32!{api} {} {}", lpStruct, uSizeStruct);

    emu.regs_mut().rax = ERROR_SUCCESS;

    for _ in 0..5 {
        emu.stack_pop32(false);
    }
}

pub fn WritePrivateProfileStructW(emu: &mut emu::Emu) {
    write_private_profile_struct(emu, "WritePrivateProfileStructW", true);
}
