use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn SelectObject(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let hdc = api_try!(emu, call.arg(emu, 0));
    let obj = api_try!(emu, call.arg(emu, 1));
    log_red!(emu, "gdi32!SelectObject hdc:0x{:x} obj:0x{:x}", hdc, obj);
    if !helper::handler_exist(hdc) {
        log_red!(emu, "gdi32!SelectObject invalid hdc");
        call.set_return(emu, 0);
        return;
    }
    if !helper::handler_exist(obj) {
        log_red!(emu, "gdi32!SelectObject invalid obj");
        call.set_return(emu, 0);
        return;
    }
    // Return previous object (simulate by returning the obj being selected)
    call.set_return(emu, obj);
}
