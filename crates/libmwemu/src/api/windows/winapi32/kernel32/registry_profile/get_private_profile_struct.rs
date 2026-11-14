use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper::WinApiAbi;
use crate::windows::constants::errors::*;

pub fn GetPrivateProfileStructA(emu: &mut emu::Emu) {
    get_private_profile_struct(emu, "GetPrivateProfileStructA", false);
}

fn get_private_profile_struct(emu: &mut emu::Emu, api: &str, wide: bool) {
    let _lpszSection = WinApiAbi::arg(emu, 0) as u32;
    let _lpszKey = WinApiAbi::arg(emu, 1) as u32;
    let lpStruct = WinApiAbi::arg(emu, 2) as u32;
    let uSizeStruct = WinApiAbi::arg(emu, 3) as u32;
    let _szFile = WinApiAbi::arg(emu, 4) as u32;

    let lpStruct = strings::read_text(emu, lpStruct as u64, wide);
    let uSizeStruct = strings::read_text(emu, uSizeStruct as u64, wide);

    log_red!(emu, "kernel32!{api} {} {}", lpStruct, uSizeStruct);

    emu.regs_mut().rax = ERROR_SUCCESS;

    for _ in 0..5 {
        emu.stack_pop32(false);
    }
}

pub fn GetPrivateProfileStructW(emu: &mut emu::Emu) {
    get_private_profile_struct(emu, "GetPrivateProfileStructW", true);
}
