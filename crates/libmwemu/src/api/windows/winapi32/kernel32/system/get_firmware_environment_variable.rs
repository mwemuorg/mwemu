use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::policy::api_try;
use crate::windows::constants::errors::*;

pub fn GetFirmwareEnvironmentVariableA(emu: &mut emu::Emu) {
    get_firmware_environment_variable(emu, "GetFirmwareEnvironmentVariableA", false);
}

fn get_firmware_environment_variable(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let lpName = api_try!(emu, call.arg_u32(emu, 0));
    let _lpGuid = api_try!(emu, call.arg_u32(emu, 1));
    let _pBuffer = api_try!(emu, call.arg_u32(emu, 2));
    let _nSize = api_try!(emu, call.arg_u32(emu, 3));

    let lpName = strings::read_text(emu, lpName as u64, wide);

    log_red!(emu, "kernel32!{api} {}", lpName);

    call.set_return(emu, ERROR_SUCCESS);
    call.cleanup_args(emu, 4);
}

pub fn GetFirmwareEnvironmentVariableW(emu: &mut emu::Emu) {
    get_firmware_environment_variable(emu, "GetFirmwareEnvironmentVariableW", true);
}
