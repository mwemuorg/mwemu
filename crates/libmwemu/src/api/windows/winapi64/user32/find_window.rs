use super::find_window;
use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn FindWindowA(emu: &mut emu::Emu) {
    find_window_text(emu, "FindWindowA", false);
}

pub fn FindWindowW(emu: &mut emu::Emu) {
    find_window_text(emu, "FindWindowW", true);
}

fn find_window_text(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let class_ptr = api_try!(emu, call.arg(emu, 0));
    let window_ptr = api_try!(emu, call.arg(emu, 1));
    find_window(emu, api, 0, 0, class_ptr, window_ptr, wide);
}
