use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub(super) fn LoadCursorA(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let hinstance = api_try!(emu, call.arg(emu, 0));
    let lpcursorname = api_try!(emu, call.arg(emu, 1));
    log_red!(
        emu,
        "user32!LoadCursorA hInst:0x{:x} name:0x{:x}",
        hinstance,
        lpcursorname
    );
    call.set_return(emu, helper::handler_create("user32://cursor"));
}

pub fn LoadCursorW(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let hinstance = api_try!(emu, call.arg(emu, 0));
    let cursor_name = api_try!(emu, call.arg(emu, 1));
    log_red!(
        emu,
        "user32!LoadCursorW hinst:0x{:x} cursor:0x{:x}",
        hinstance,
        cursor_name
    );
    let hndl = helper::handler_create("user32://LoadCursorW");
    call.set_return(emu, hndl);
}
