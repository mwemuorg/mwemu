use super::find_window;
use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn FindWindowExA(emu: &mut emu::Emu) {
    find_window_ex(emu, "FindWindowExA", false);
}

pub fn FindWindowExW(emu: &mut emu::Emu) {
    find_window_ex(emu, "FindWindowExW", true);
}

fn find_window_ex(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let (parent, after) = (
        api_try!(emu, call.arg(emu, 0)),
        api_try!(emu, call.arg(emu, 1)),
    );
    let (class_ptr, window_ptr) = (
        api_try!(emu, call.arg(emu, 2)),
        api_try!(emu, call.arg(emu, 3)),
    );
    find_window(emu, api, parent, after, class_ptr, window_ptr, wide);
}
