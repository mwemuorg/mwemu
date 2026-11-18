use super::get_window_long_value;
use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub(super) fn SetWindowLongA(emu: &mut emu::Emu) {
    set_window_long(emu, "SetWindowLongA");
}

fn set_window_long(emu: &mut emu::Emu, api: &str) {
    let call = WinApi(Stdcall);
    let idx = api_try!(emu, call.arg_i32(emu, 1));
    let new_val = api_try!(emu, call.arg(emu, 2));
    log_red!(emu, "user32!{api} idx:{} new:0x{:x}", idx, new_val);
    call.set_return(emu, get_window_long_value(idx));
}

pub(super) fn SetWindowLongW(emu: &mut emu::Emu) {
    set_window_long(emu, "SetWindowLongW");
}
