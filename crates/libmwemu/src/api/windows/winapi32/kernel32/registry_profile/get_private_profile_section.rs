use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper::WinApiAbi;
use crate::windows::constants::errors::*;

pub fn GetPrivateProfileSectionA(emu: &mut emu::Emu) {
    get_private_profile_section(emu, "GetPrivateProfileSectionA", false);
}

fn get_private_profile_section(emu: &mut emu::Emu, api: &str, wide: bool) {
    let lpAppName = WinApiAbi::arg(emu, 0) as u32;
    let lpReturnedString = WinApiAbi::arg(emu, 1) as u32;
    let _nSize = WinApiAbi::arg(emu, 2) as u32;
    let lpFileName = WinApiAbi::arg(emu, 3) as u32;

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
