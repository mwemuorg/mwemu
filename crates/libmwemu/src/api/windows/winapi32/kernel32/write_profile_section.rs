use crate::emu;
use crate::winapi::common::strings;
use crate::windows::constants::errors::*;

pub fn WriteProfileSectionA(emu: &mut emu::Emu) {
    write_profile_section(emu, "WriteProfileSectionA", false);
}

fn write_profile_section(emu: &mut emu::Emu, api: &str, wide: bool) {
    let lpAppName = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let lpString = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);

    let lpAppName = strings::read_text(emu, lpAppName as u64, wide);
    let lpString = strings::read_text(emu, lpString as u64, wide);

    log_red!(emu, "kernel32!{api} {} {}", lpAppName, lpString);

    emu.regs_mut().rax = ERROR_SUCCESS;

    for _ in 0..2 {
        emu.stack_pop32(false);
    }
}

pub fn WriteProfileSectionW(emu: &mut emu::Emu) {
    write_profile_section(emu, "WriteProfileSectionW", true);
}
