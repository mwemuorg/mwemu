use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;
use crate::windows::constants::errors::*;

pub fn LocalShrink(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let _hMem = api_try!(emu, call.arg_u32(emu, 0));
    let _cbNewSize = api_try!(emu, call.arg_u32(emu, 1));

    log_red!(emu, "kernel32!LocalShrink");

    call.set_return(emu, ERROR_SUCCESS);
    call.cleanup_args(emu, 2);
}
