use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::policy::api_try;

pub fn SetConsoleTitleA(emu: &mut emu::Emu) {
    set_console_title(emu, "SetConsoleTitleA", false);
}

fn set_console_title(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let p0 = api_try!(emu, call.arg(emu, 0));
    let s = if p0 != 0 && p0 > 0xffff {
        strings::read_text(emu, p0, wide)
    } else {
        format!("0x{:x}", p0)
    };
    log_red!(emu, "kernel32!{api} '{}'", s);
    call.cleanup_args(emu, 1);
    call.set_return(emu, 1);
}

pub fn SetConsoleTitleW(emu: &mut emu::Emu) {
    set_console_title(emu, "SetConsoleTitleW", true);
}
