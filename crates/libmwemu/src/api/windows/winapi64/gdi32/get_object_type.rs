use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn GetObjectType(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let obj = api_try!(emu, call.arg(emu, 0));
    let result: u64 = if helper::handler_exist(obj) { 6 } else { 0 }; // OBJ_BITMAP=6 as generic
    log_red!(emu, "gdi32!GetObjectType obj:0x{:x} = {}", obj, result);
    emu.set_last_error(0);
    call.set_return(emu, result);
}
