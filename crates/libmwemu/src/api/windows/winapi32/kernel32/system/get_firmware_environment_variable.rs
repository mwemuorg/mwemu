use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper::WinApiAbi;
use crate::windows::constants::errors::*;

pub fn GetFirmwareEnvironmentVariableA(emu: &mut emu::Emu) {
    get_firmware_environment_variable(emu, "GetFirmwareEnvironmentVariableA", false);
}

fn get_firmware_environment_variable(emu: &mut emu::Emu, api: &str, wide: bool) {
    let lpName = WinApiAbi::arg(emu, 0) as u32;
    let _lpGuid = WinApiAbi::arg(emu, 1) as u32;
    let _pBuffer = WinApiAbi::arg(emu, 2) as u32;
    let _nSize = WinApiAbi::arg(emu, 3) as u32;

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
