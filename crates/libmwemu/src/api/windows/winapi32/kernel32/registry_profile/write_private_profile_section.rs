use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper::WinApiAbi;
use crate::windows::constants::errors::*;

pub fn WritePrivateProfileSectionA(emu: &mut emu::Emu) {
    write_private_profile_section(emu, "WritePrivateProfileSectionA", false);
}

fn write_private_profile_section(emu: &mut emu::Emu, api: &str, wide: bool) {
    let lpAppName = WinApiAbi::arg(emu, 0) as u32;
    let lpString = WinApiAbi::arg(emu, 1) as u32;
    let lpFileName = WinApiAbi::arg(emu, 2) as u32;

    let lpAppName = strings::read_text(emu, lpAppName as u64, wide);
    let lpString = strings::read_text(emu, lpString as u64, wide);
    let lpFileName = strings::read_text(emu, lpFileName as u64, wide);

    log_red!(
        emu,
        "kernel32!{api} {} {} {}",
        lpAppName,
        lpString,
        lpFileName
    );

    emu.regs_mut().rax = ERROR_SUCCESS;

    for _ in 0..3 {
        emu.stack_pop32(false);
    }
}

pub fn WritePrivateProfileSectionW(emu: &mut emu::Emu) {
    write_private_profile_section(emu, "WritePrivateProfileSectionW", true);
}
