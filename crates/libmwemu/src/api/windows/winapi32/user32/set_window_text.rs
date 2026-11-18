use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::policy::api_try;

pub(super) fn SetWindowTextA(emu: &mut emu::Emu) {
    set_window_text(emu, "SetWindowTextA", false);
}

fn set_window_text(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let text = strings::read_text(emu, api_try!(emu, call.arg_u32(emu, 1)) as u64, wide);
    log_red!(emu, "user32!{api} '{}'", text);
    call.set_return(emu, 1);
    call.cleanup_args(emu, 2);
}

pub(super) fn SetWindowTextW(emu: &mut emu::Emu) {
    set_window_text(emu, "SetWindowTextW", true);
}
