
use crate::windows::constants::errors::*;
use crate::emu;

pub fn CheckNameLegalDOS8Dot3W(emu: &mut emu::Emu) {
    let lpName = emu.regs().rcx as u32;
    let lpOemName = emu.regs().rdx as u32;
    let OemNameSize = emu.regs().r8 as u32;
    let pbNameContainsSpaces = emu.regs().r9 as u32;
    let pbNameLegal = emu.maps.read_qword(emu.regs().rsp + 0x20).unwrap_or(0);

    let lpName = emu.maps.read_wide_string(lpName as u64);
    let lpOemName = emu.maps.read_wide_string(lpOemName as u64);
    let OemNameSize = emu.maps.read_wide_string(OemNameSize as u64);
    let pbNameContainsSpaces = emu.maps.read_wide_string(pbNameContainsSpaces as u64);
    let pbNameLegal = emu.maps.read_wide_string(pbNameLegal);

    log_red!(
        emu,
        "kernel32!CheckNameLegalDOS8Dot3W {} {} {} {} {}",
        lpName,
        lpOemName,
        OemNameSize,
        pbNameContainsSpaces,
        pbNameLegal
    );

    emu.set_last_error(0);
    emu.regs_mut().rax = ERROR_SUCCESS;
}
