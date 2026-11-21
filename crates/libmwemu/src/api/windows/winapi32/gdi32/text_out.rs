use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::policy::api_try;

pub fn TextOutA(emu: &mut emu::Emu) {
    text_out(emu, "TextOutA", false);
}

fn text_out(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let hdc = api_try!(emu, call.arg_u32(emu, 0));
    let x = api_try!(emu, call.arg_u32(emu, 1));
    let y = api_try!(emu, call.arg_u32(emu, 2));
    let text_ptr = api_try!(emu, call.arg_u32(emu, 3)) as u64;
    let text = strings::read_text(emu, text_ptr, wide);
    log_red!(emu, "gdi32!{api} '{}'", text);
    call.set_return(emu, 1);
    call.cleanup_args(emu, 5);
}

pub fn TextOutW(emu: &mut emu::Emu) {
    text_out(emu, "TextOutW", true);
}
