use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;
use crate::windows::constants::errors::*;

pub fn WerpMapProtectionLevel(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let _a1 = api_try!(emu, call.arg_u32(emu, 0));

    log_red!(emu, "kernel32!WerpMapProtectionLevel");

    call.set_return(emu, ERROR_SUCCESS);
    call.cleanup_args(emu, 1);
}
