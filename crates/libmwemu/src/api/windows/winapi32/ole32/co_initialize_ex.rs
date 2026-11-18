use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;
use crate::windows::constants::errors::*;

pub(super) fn CoInitializeEx(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let co_init = api_try!(emu, call.arg_u32(emu, 1));
    log_red!(emu, "ole32!CoInitializeEx dwCoInit: 0x{:x}", co_init);
    call.set_return(emu, S_OK);
    call.cleanup_args(emu, 2);
}
