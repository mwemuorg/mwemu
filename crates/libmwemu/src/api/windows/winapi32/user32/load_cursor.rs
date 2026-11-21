use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub(super) fn LoadCursorA(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let hinstance = api_try!(emu, call.arg_u32(emu, 0));
    let name = api_try!(emu, call.arg_u32(emu, 1));
    log_red!(
        emu,
        "user32!LoadCursorA hInstance: 0x{:x} name: 0x{:x}",
        hinstance,
        name
    );
    call.set_return(emu, 0);
    call.cleanup_args(emu, 2);
}

pub fn LoadCursorW(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let a0 = api_try!(emu, call.arg_u32(emu, 0)) as u64;
    let a1 = api_try!(emu, call.arg_u32(emu, 1)) as u64;
    log_red!(emu, "user32!LoadCursorW arg0:0x{:x} arg1:0x{:x}", a0, a1);
    call.set_return(emu, helper::handler_create("loadcursorw"));
    call.cleanup_args(emu, 2);
}
