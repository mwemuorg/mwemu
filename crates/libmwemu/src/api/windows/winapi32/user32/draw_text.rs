use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::policy::api_try;

pub(super) fn DrawTextA(emu: &mut emu::Emu) {
    draw_text(emu, "DrawTextA", false);
}

fn draw_text(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let text = strings::read_text(emu, api_try!(emu, call.arg_u32(emu, 1)) as u64, wide);
    log_red!(emu, "user32!{api} '{}'", text);
    call.set_return(emu, 20);
    call.cleanup_args(emu, 5);
}

pub(super) fn DrawTextW(emu: &mut emu::Emu) {
    draw_text(emu, "DrawTextW", true);
}
