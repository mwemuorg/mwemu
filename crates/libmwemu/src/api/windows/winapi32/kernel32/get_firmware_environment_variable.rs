use crate::emu;
use crate::winapi::common::strings;
use crate::windows::constants::errors::*;

pub fn GetFirmwareEnvironmentVariableA(emu: &mut emu::Emu) {
    get_firmware_environment_variable(emu, "GetFirmwareEnvironmentVariableA", false);
}

fn get_firmware_environment_variable(emu: &mut emu::Emu, api: &str, wide: bool) {
    let lpName = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let _lpGuid = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let _pBuffer = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
    let _nSize = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);

    let lpName = strings::read_text(emu, lpName as u64, wide);

    log_red!(emu, "kernel32!{api} {}", lpName);

    emu.regs_mut().rax = ERROR_SUCCESS;

    for _ in 0..4 {
        emu.stack_pop32(false);
    }
}

pub fn GetFirmwareEnvironmentVariableW(emu: &mut emu::Emu) {
    get_firmware_environment_variable(emu, "GetFirmwareEnvironmentVariableW", true);
}
