
use crate::windows::constants::errors::*;
use crate::emu;

pub fn GetProfileSectionW(emu: &mut emu::Emu) {
    let lpAppName = emu.regs().rcx as u32;
    let lpReturnedString = emu.regs().rdx as u32;
    let _nSize = emu.regs().r8 as u32;

    let lpAppName = emu.maps.read_wide_string(lpAppName as u64);
    let lpReturnedString = emu.maps.read_wide_string(lpReturnedString as u64);

    log_red!(
        emu,
        "kernel32!GetProfileSectionW {} {}",
        lpAppName,
        lpReturnedString
    );

    emu.set_last_error(0);
    emu.regs_mut().rax = ERROR_SUCCESS;
}
