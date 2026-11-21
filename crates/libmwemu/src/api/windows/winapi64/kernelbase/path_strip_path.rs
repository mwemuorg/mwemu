use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::policy::api_try;

pub fn PathStripPathA(emu: &mut emu::Emu) {
    path_strip_path(emu, "PathStripPathA", false);
}

fn path_strip_path(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let p0 = api_try!(emu, call.arg(emu, 0));
    let s = if p0 != 0 && p0 > 0xffff {
        strings::read_text(emu, p0, wide)
    } else {
        format!("0x{:x}", p0)
    };
    log_red!(emu, "kernelbase!{api} '{}'", s);
    emu.set_last_error(0);
    call.set_return(emu, 1);
}

pub fn PathStripPathW(emu: &mut emu::Emu) {
    path_strip_path(emu, "PathStripPathW", true);
}
