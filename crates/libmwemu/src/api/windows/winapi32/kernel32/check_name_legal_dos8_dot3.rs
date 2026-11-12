use crate::emu;
use crate::winapi::common::strings;
use crate::windows::constants::errors::*;

pub fn CheckNameLegalDOS8Dot3A(emu: &mut emu::Emu) {
    check_name_legal_dos8_dot3(emu, "CheckNameLegalDOS8Dot3A", false);
}

fn check_name_legal_dos8_dot3(emu: &mut emu::Emu, api: &str, wide: bool) {
    let lpName = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let lpOemName = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let OemNameSize = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
    let pbNameContainsSpaces = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
    let pbNameLegal = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0);

    let lpName = strings::read_text(emu, lpName as u64, wide);
    let lpOemName = strings::read_text(emu, lpOemName as u64, wide);
    let OemNameSize = strings::read_text(emu, OemNameSize as u64, wide);
    let pbNameContainsSpaces = strings::read_text(emu, pbNameContainsSpaces as u64, wide);
    let pbNameLegal = strings::read_text(emu, pbNameLegal as u64, wide);

    log_red!(
        emu,
        "kernel32!{api} {} {} {} {} {}",
        lpName,
        lpOemName,
        OemNameSize,
        pbNameContainsSpaces,
        pbNameLegal
    );

    emu.regs_mut().rax = ERROR_SUCCESS;

    for _ in 0..5 {
        emu.stack_pop32(false);
    }
}

pub fn CheckNameLegalDOS8Dot3W(emu: &mut emu::Emu) {
    check_name_legal_dos8_dot3(emu, "CheckNameLegalDOS8Dot3W", true);
}
