
use crate::windows::constants::errors::*;
use crate::emu;

pub fn GetPrivateProfileSectionNamesA(emu: &mut emu::Emu) {
    let _lpszReturnBuffer = emu.regs().rcx as u32;
    let _nSize = emu.regs().rdx as u32;
    let lpFileName = emu.regs().r8 as u32;

    let lpFileName = emu.maps.read_string(lpFileName as u64);

    log_red!(
        emu,
        "kernel32!GetPrivateProfileSectionNamesA {}",
        lpFileName
    );

    emu.set_last_error(0);
    emu.regs_mut().rax = ERROR_SUCCESS;
}
