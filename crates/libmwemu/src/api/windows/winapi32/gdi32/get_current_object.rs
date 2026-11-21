use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn GetCurrentObject(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let hdc = api_try!(emu, call.arg_u32(emu, 0));
    let obj_type = api_try!(emu, call.arg_u32(emu, 1));
    log_red!(
        emu,
        "gdi32!GetCurrentObject hdc:0x{:x} type:{}",
        hdc,
        obj_type
    );
    let hndl = helper::handler_create("gdi32://current_object");
    assert!(hndl < 0x00000001_00000000);
    call.set_return(emu, hndl);
    call.cleanup_args(emu, 2);
}
