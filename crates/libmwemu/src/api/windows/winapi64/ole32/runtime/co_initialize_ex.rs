use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;
use crate::windows::constants::errors::*;

pub fn CoInitializeEx(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let co_init = api_try!(emu, call.arg(emu, 1));
    log_red!(emu, "ole32!CoInitializeEx dwCoInit: 0x{:x}", co_init);
    emu.set_last_error(0);
    call.set_return(emu, S_OK);
}
