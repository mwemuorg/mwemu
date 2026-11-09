
use crate::windows::constants::errors::*;
use crate::emu;

pub fn GetPrivateProfileStringA(emu: &mut emu::Emu) {
    let lpAppName = emu.regs().rcx as u32;
    let lpKeyName = emu.regs().rdx as u32;
    let _lpDefault = emu.regs().r8 as u32;
    let lpReturnedString = emu.regs().r9 as u32;
    let _nSize = emu.maps.read_qword(emu.regs().rsp + 0x20).unwrap_or(0);
    let lpFileName = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);

    let lpAppName = emu.maps.read_string(lpAppName as u64);
    let lpKeyName = emu.maps.read_string(lpKeyName as u64);
    let lpReturnedString = emu.maps.read_string(lpReturnedString as u64);
    let lpFileName = emu.maps.read_string(lpFileName);

    log_red!(
        emu,
        "kernel32!GetPrivateProfileStringA {} {} {} {}",
        lpAppName,
        lpKeyName,
        lpReturnedString,
        lpFileName
    );

    emu.set_last_error(0);
    emu.regs_mut().rax = ERROR_SUCCESS;
}
