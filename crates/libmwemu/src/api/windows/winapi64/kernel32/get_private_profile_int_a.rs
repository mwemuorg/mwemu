
use crate::windows::constants::errors::*;
use crate::emu;

pub fn GetPrivateProfileIntA(emu: &mut emu::Emu) {
    let lpAppName = emu.regs().rcx as u32;
    let lpKeyName = emu.regs().rdx as u32;
    let _nDefault = emu.regs().r8 as u32;
    let lpFileName = emu.regs().r9 as u32;

    let lpAppName = emu.maps.read_string(lpAppName as u64);
    let lpKeyName = emu.maps.read_string(lpKeyName as u64);
    let lpFileName = emu.maps.read_string(lpFileName as u64);

    log_red!(
        emu,
        "kernel32!GetPrivateProfileIntA {} {} {}",
        lpAppName,
        lpKeyName,
        lpFileName
    );

    emu.set_last_error(0);
    emu.regs_mut().rax = ERROR_SUCCESS;
}
