use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn CreateCompatibleBitmap(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let hdc = api_try!(emu, call.arg_u32(emu, 0));
    let w = api_try!(emu, call.arg_u32(emu, 1));
    let h = api_try!(emu, call.arg_u32(emu, 2));
    log_red!(emu, "gdi32!CreateCompatibleBitmap {}x{}", w, h);
    let hndl = helper::handler_create("gdi32://CreateCompatibleBitmap");
    assert!(hndl < 0x00000001_00000000);
    call.set_return(emu, hndl);
    call.cleanup_args(emu, 3);
}
