use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn GetStockObject(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let idx = api_try!(emu, call.arg_u32(emu, 0));
    log_red!(emu, "gdi32!GetStockObject index:{}", idx);
    let hndl = helper::handler_create("gdi32://stock_object");
    assert!(hndl < 0x00000001_00000000);
    call.set_return(emu, hndl);
    call.cleanup_args(emu, 1);
}
