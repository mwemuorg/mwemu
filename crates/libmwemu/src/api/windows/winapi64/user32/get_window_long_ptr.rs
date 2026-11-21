use super::get_window_long_value;
use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub(super) fn GetWindowLongPtrA(emu: &mut emu::Emu) {
    get_window_long_ptr(emu, "GetWindowLongPtrA");
}

fn get_window_long_ptr(emu: &mut emu::Emu, api: &str) {
    let call = WinApi(Stdcall);
    let idx = api_try!(emu, call.arg_i32(emu, 1));
    let result = get_window_long_value(idx);
    log_red!(emu, "user32!{api} idx:{} => 0x{:x}", idx, result);
    call.set_return(emu, result);
}

pub(super) fn GetWindowLongPtrW(emu: &mut emu::Emu) {
    get_window_long_ptr(emu, "GetWindowLongPtrW");
}
