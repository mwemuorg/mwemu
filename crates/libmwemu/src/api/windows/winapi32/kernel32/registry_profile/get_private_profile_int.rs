use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper::WinApiAbi;
use crate::windows::constants::errors::*;

pub fn GetPrivateProfileIntA(emu: &mut emu::Emu) {
    get_private_profile_int(emu, "GetPrivateProfileIntA", false);
}

fn get_private_profile_int(emu: &mut emu::Emu, api: &str, wide: bool) {
    let lpAppName = WinApiAbi::arg(emu, 0) as u32;
    let lpKeyName = WinApiAbi::arg(emu, 1) as u32;
    let _nDefault = WinApiAbi::arg(emu, 2) as u32;
    let lpFileName = WinApiAbi::arg(emu, 3) as u32;

    let lpAppName = strings::read_text(emu, lpAppName as u64, wide);
    let lpKeyName = strings::read_text(emu, lpKeyName as u64, wide);
    let lpFileName = strings::read_text(emu, lpFileName as u64, wide);

    log_red!(
        emu,
        "kernel32!{api} {} {} {}",
        lpAppName,
        lpKeyName,
        lpFileName
    );

    emu.regs_mut().rax = ERROR_SUCCESS;

    for _ in 0..4 {
        emu.stack_pop32(false);
    }
}

pub fn GetPrivateProfileIntW(emu: &mut emu::Emu) {
    get_private_profile_int(emu, "GetPrivateProfileIntW", true);
}
