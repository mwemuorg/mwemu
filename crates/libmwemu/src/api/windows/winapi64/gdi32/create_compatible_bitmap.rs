use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn CreateCompatibleBitmap(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let hdc = api_try!(emu, call.arg(emu, 0));
    let width = api_try!(emu, call.arg(emu, 1));
    let height = api_try!(emu, call.arg(emu, 2));
    log_red!(
        emu,
        "gdi32!CreateCompatibleBitmap hdc:0x{:x} w:{} h:{}",
        hdc,
        width,
        height
    );
    call.set_return(
        emu,
        helper::handler_create("gdi32://CreateCompatibleBitmap"),
    );
}
