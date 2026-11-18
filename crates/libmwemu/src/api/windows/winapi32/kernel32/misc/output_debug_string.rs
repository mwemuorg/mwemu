use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::policy::api_try;

pub fn OutputDebugStringA(emu: &mut emu::Emu) {
    output_debug_string(emu, "OutputDebugStringA", false);
}

fn output_debug_string(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let msg = strings::read_text(emu, api_try!(emu, call.arg_u32(emu, 0)) as u64, wide);
    log_red!(emu, "kernel32!{api} `{}`", msg);
    call.set_return(emu, 0);
    call.cleanup_args(emu, 1);
}

pub fn OutputDebugStringW(emu: &mut emu::Emu) {
    output_debug_string(emu, "OutputDebugStringW", true);
}
