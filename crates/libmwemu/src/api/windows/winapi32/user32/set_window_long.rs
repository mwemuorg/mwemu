use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub(super) fn SetWindowLongA(emu: &mut emu::Emu) {
    set_window_long(emu, "SetWindowLongA");
}

fn set_window_long(emu: &mut emu::Emu, api: &str) {
    let call = WinApi(Stdcall);
    let index = api_try!(emu, call.arg_u32(emu, 1));
    log_red!(emu, "user32!{api} idx:{}", index as i32);
    call.set_return(emu, 0);
    call.cleanup_args(emu, 3);
}

pub(super) fn SetWindowLongW(emu: &mut emu::Emu) {
    set_window_long(emu, "SetWindowLongW");
}
