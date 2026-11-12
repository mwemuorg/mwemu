use crate::emu;
use crate::winapi::common::strings;
use crate::windows::constants::errors::*;

pub fn WriteProfileStringA(emu: &mut emu::Emu) {
    write_profile_string(emu, "WriteProfileStringA", false);
}

fn write_profile_string(emu: &mut emu::Emu, api: &str, wide: bool) {
    let lpAppName = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let lpKeyName = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let lpString = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);

    let lpAppName = strings::read_text(emu, lpAppName as u64, wide);
    let lpKeyName = strings::read_text(emu, lpKeyName as u64, wide);
    let lpString = strings::read_text(emu, lpString as u64, wide);

    log_red!(
        emu,
        "kernel32!{api} {} {} {}",
        lpAppName,
        lpKeyName,
        lpString
    );

    emu.regs_mut().rax = ERROR_SUCCESS;

    for _ in 0..3 {
        emu.stack_pop32(false);
    }
}

pub fn WriteProfileStringW(emu: &mut emu::Emu) {
    write_profile_string(emu, "WriteProfileStringW", true);
}
