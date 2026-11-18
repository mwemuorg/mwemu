use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn SelectObject(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let hdc = api_try!(emu, call.arg_u32(emu, 0));
    let obj = api_try!(emu, call.arg_u32(emu, 1));
    log_red!(emu, "gdi32!SelectObject hdc:0x{:x} obj:0x{:x}", hdc, obj);
    let prev = helper::handler_create("gdi32://prev_object");
    assert!(prev < 0x00000001_00000000);
    call.set_return(emu, prev);
    call.cleanup_args(emu, 2);
}
