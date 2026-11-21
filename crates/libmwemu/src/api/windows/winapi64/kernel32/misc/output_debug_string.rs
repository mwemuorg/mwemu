use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::policy::api_try;

pub fn OutputDebugStringA(emu: &mut emu::Emu) {
    output_debug_string(emu, "OutputDebugStringA", false);
}

fn output_debug_string(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let str_ptr = api_try!(emu, call.arg(emu, 0));

    let msg = strings::read_text(emu, str_ptr, wide);

    log_red!(emu, "kernel32!{api} `{}`", msg);

    emu.set_last_error(0);
    call.set_return(emu, 0);
}

pub fn OutputDebugStringW(emu: &mut emu::Emu) {
    output_debug_string(emu, "OutputDebugStringW", true);
}
