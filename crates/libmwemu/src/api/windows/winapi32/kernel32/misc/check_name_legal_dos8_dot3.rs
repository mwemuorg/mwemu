use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper::WinApiAbi;
use crate::windows::constants::errors::*;

pub fn CheckNameLegalDOS8Dot3A(emu: &mut emu::Emu) {
    check_name_legal_dos8_dot3(emu, "CheckNameLegalDOS8Dot3A", false);
}

fn check_name_legal_dos8_dot3(emu: &mut emu::Emu, api: &str, wide: bool) {
    let lpName = WinApiAbi::arg(emu, 0) as u32;
    let lpOemName = WinApiAbi::arg(emu, 1) as u32;
    let OemNameSize = WinApiAbi::arg(emu, 2) as u32;
    let pbNameContainsSpaces = WinApiAbi::arg(emu, 3) as u32;
    let pbNameLegal = WinApiAbi::arg(emu, 4) as u32;

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
