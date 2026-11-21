use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn FormatMessageA(emu: &mut emu::Emu) {
    format_message(emu, "FormatMessageA");
}

fn format_message(emu: &mut emu::Emu, api: &str) {
    let call = WinApi(Stdcall);
    let flags = api_try!(emu, call.arg_u32(emu, 0));
    log_red!(emu, "kernel32!{api} flags: 0x{:x} (stub)", flags);
    call.set_return(emu, 0);
    call.cleanup_args(emu, 7);
}

pub fn FormatMessageW(emu: &mut emu::Emu) {
    format_message(emu, "FormatMessageW");
}
