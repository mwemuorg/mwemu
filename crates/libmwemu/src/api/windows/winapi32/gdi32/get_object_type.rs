use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn GetObjectType(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let obj = api_try!(emu, call.arg_u32(emu, 0)) as u64;
    log_red!(emu, "gdi32!GetObjectType obj:0x{:x}", obj);
    if helper::handler_exist(obj) {
        helper::advance_tick(emu, 1);
        emu.set_last_error(0);
        call.set_return(emu, 6); // OBJ_BITMAP
    } else {
        call.set_return(emu, 0);
    }
    call.cleanup_args(emu, 1);
}
