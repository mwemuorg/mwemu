use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;

pub(super) fn GetWindowTextA(emu: &mut emu::Emu) {
    get_window_text(emu, "GetWindowTextA");
}

fn get_window_text(emu: &mut emu::Emu, api: &str) {
    let call = WinApi(Stdcall);
    log_red!(emu, "user32!{api}");
    call.set_return(emu, 0);
    call.cleanup_args(emu, 3);
}

pub(super) fn GetWindowTextW(emu: &mut emu::Emu) {
    get_window_text(emu, "GetWindowTextW");
}
