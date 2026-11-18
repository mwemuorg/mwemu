use super::register_class;
use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub(super) fn RegisterClassExA(emu: &mut emu::Emu) {
    register_class_ex(emu, "RegisterClassExA", false);
}

pub(super) fn RegisterClassExW(emu: &mut emu::Emu) {
    register_class_ex(emu, "RegisterClassExW", true);
}

fn register_class_ex(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let lpwndclass = api_try!(emu, call.arg(emu, 0));
    log_red!(emu, "user32!{api} 0x{:x}", lpwndclass);
    register_class(emu, lpwndclass, wide);
}
