use crate::emu;
use crate::windows::constants;

pub fn GetFirmwareEnvironmentVariableW(emu: &mut emu::Emu) {
    let lpName = emu.regs().rcx as u32;
    let _lpGuid = emu.regs().rdx as u32;
    let _pBuffer = emu.regs().r8 as u32;
    let _nSize = emu.regs().r9 as u32;

    let lpName = emu.maps.read_wide_string(lpName as u64);

    log_red!(emu, "kernel32!GetFirmwareEnvironmentVariableW {}", lpName);

    emu.set_last_error(0);
    emu.regs_mut().rax = constants::ERROR_SUCCESS;
}
