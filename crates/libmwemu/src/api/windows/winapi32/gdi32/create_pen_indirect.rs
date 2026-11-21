use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn CreatePenIndirect(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let arg1 = api_try!(emu, call.arg_u32(emu, 0));
    log_red!(emu, "gdi32!CreatePenIndirect arg1:0x{:x}", arg1);
    call.cleanup_args(emu, 1);
    let hndl = helper::handler_create("gdi32://CreatePenIndirect");
    assert!(hndl < 0x00000001_00000000);
    call.set_return(emu, hndl);
}
