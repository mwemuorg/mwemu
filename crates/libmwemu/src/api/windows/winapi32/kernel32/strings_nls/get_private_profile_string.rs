use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper::WinApiAbi;
use crate::windows::constants::errors::*;

pub fn GetPrivateProfileStringA(emu: &mut emu::Emu) {
    get_private_profile_string(emu, "GetPrivateProfileStringA", false);
}

fn get_private_profile_string(emu: &mut emu::Emu, api: &str, wide: bool) {
    let lpAppName = WinApiAbi::arg(emu, 0) as u32;
    let lpKeyName = WinApiAbi::arg(emu, 1) as u32;
    let _lpDefault = WinApiAbi::arg(emu, 2) as u32;
    let lpReturnedString = WinApiAbi::arg(emu, 3) as u32;
    let _nSize = WinApiAbi::arg(emu, 4) as u32;
    let lpFileName = WinApiAbi::arg(emu, 5) as u32;

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
