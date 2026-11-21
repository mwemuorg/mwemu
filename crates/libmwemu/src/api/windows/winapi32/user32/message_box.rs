use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::policy::api_try;

pub(super) fn MessageBoxA(emu: &mut emu::Emu) {
    message_box(emu, "MessageBoxA", false);
}

fn message_box(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let titleptr = api_try!(emu, call.arg(emu, 2));
    let msgptr = api_try!(emu, call.arg(emu, 1));
    let msg = strings::read_text(emu, msgptr, wide);
    let title = strings::read_text(emu, titleptr, wide);

    log_red!(emu, "user32!{api} {} {}", title, msg);

    call.set_return(emu, 0);
    call.cleanup_args(emu, 4);
}

pub(super) fn MessageBoxW(emu: &mut emu::Emu) {
    message_box(emu, "MessageBoxW", true);
}
