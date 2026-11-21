use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn FreeEnvironmentStringsA(emu: &mut emu::Emu) {
    free_environment_strings(emu, "FreeEnvironmentStringsA");
}

fn free_environment_strings(emu: &mut emu::Emu, api: &str) {
    let call = WinApi(Stdcall);
    let _penv = api_try!(emu, call.arg_u32(emu, 0));

    call.cleanup_args(emu, 1);

    log_red!(emu, "kernel32!{api}");

    call.set_return(emu, 1);
}

pub fn FreeEnvironmentStringsW(emu: &mut emu::Emu) {
    free_environment_strings(emu, "FreeEnvironmentStringsW");
}
