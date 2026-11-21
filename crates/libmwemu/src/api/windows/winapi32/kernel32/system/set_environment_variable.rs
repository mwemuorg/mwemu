use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::policy::api_try;

pub fn SetEnvironmentVariableA(emu: &mut emu::Emu) {
    set_environment_variable(emu, "SetEnvironmentVariableA", false);
}

fn set_environment_variable(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let name_ptr = api_try!(emu, call.arg_u32(emu, 0)) as u64;
    let value_ptr = api_try!(emu, call.arg_u32(emu, 1)) as u64;
    let name = strings::read_text(emu, name_ptr, wide);
    let value = if value_ptr != 0 {
        strings::read_text(emu, value_ptr, wide)
    } else {
        String::new()
    };
    log_red!(emu, "kernel32!{api} `{}`=`{}` (stub)", name, value);
    call.set_return(emu, 1);
    call.cleanup_args(emu, 2);
}

pub fn SetEnvironmentVariableW(emu: &mut emu::Emu) {
    set_environment_variable(emu, "SetEnvironmentVariableW", true);
}
