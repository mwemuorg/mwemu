use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub(super) fn GetWindowLongA(emu: &mut emu::Emu) {
    get_window_long(emu, "GetWindowLongA");
}

fn get_window_long(emu: &mut emu::Emu, api: &str) {
    let call = WinApi(Stdcall);
    let index = api_try!(emu, call.arg_u32(emu, 1));
    log_red!(emu, "user32!{api} idx:{}", index as i32);
    call.set_return(emu, 0);
    call.cleanup_args(emu, 2);
}

pub(super) fn GetWindowLongW(emu: &mut emu::Emu) {
    get_window_long(emu, "GetWindowLongW");
}
