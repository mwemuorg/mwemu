use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::policy::api_try;
use crate::windows::constants::errors::*;

pub fn SetFirmwareEnvironmentVariableA(emu: &mut emu::Emu) {
    set_firmware_environment_variable(emu, "SetFirmwareEnvironmentVariableA", false);
}

fn set_firmware_environment_variable(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let lpName = api_try!(emu, call.arg_u32(emu, 0));
    let _lpGuid = api_try!(emu, call.arg_u32(emu, 1));
    let _pValue = api_try!(emu, call.arg_u32(emu, 2));
    let _nSize = api_try!(emu, call.arg_u32(emu, 3));

    let lpName = strings::read_text(emu, lpName as u64, wide);

    log_red!(emu, "kernel32!{api} {}", lpName);

    call.set_return(emu, ERROR_SUCCESS);
    call.cleanup_args(emu, 4);
}

pub fn SetFirmwareEnvironmentVariableW(emu: &mut emu::Emu) {
    set_firmware_environment_variable(emu, "SetFirmwareEnvironmentVariableW", true);
}
