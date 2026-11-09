
use crate::windows::constants::errors::*;
use crate::emu;

pub fn GetFirmwareEnvironmentVariableA(emu: &mut emu::Emu) {
    let lpName = emu.regs().rcx as u32;
    let _lpGuid = emu.regs().rdx as u32;
    let _pBuffer = emu.regs().r8 as u32;
    let _nSize = emu.regs().r9 as u32;

    let lpName = emu.maps.read_string(lpName as u64);

    log_red!(emu, "kernel32!GetFirmwareEnvironmentVariableA {}", lpName);

    emu.set_last_error(0);
    emu.regs_mut().rax = ERROR_SUCCESS;
}
