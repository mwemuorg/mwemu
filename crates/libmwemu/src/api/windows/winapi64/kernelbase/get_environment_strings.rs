use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn GetEnvironmentStrings(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    log_red!(emu, "kernelbase!GetEnvironmentStrings");
    let env_str = "COMPUTERNAME=DESKTOP-MWEMU\0SYSTEMROOT=C:\\Windows\0\0";
    let handle = helper::handler_create("env_strings");
    emu.set_last_error(0);
    call.set_return(emu, handle);
}

pub fn GetEnvironmentStringsA(emu: &mut emu::Emu) {
    get_environment_strings(emu, "GetEnvironmentStringsA", false);
}

fn get_environment_strings(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let str_ptr = api_try!(emu, call.arg(emu, 0));
    let arg2 = api_try!(emu, call.arg(emu, 1));
    let name = strings::read_text(emu, str_ptr, wide);
    log_red!(emu, "kernelbase!{api} '{}' 0x{:x}", name, arg2);
    emu.set_last_error(0);
    call.set_return(emu, 1);
}

pub fn GetEnvironmentStringsW(emu: &mut emu::Emu) {
    get_environment_strings(emu, "GetEnvironmentStringsW", true);
}
