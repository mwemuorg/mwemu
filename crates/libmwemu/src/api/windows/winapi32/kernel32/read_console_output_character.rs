use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::policy::api_try;

pub fn ReadConsoleOutputCharacterA(emu: &mut emu::Emu) {
    read_console_output_character(emu, "ReadConsoleOutputCharacterA", false);
}

fn read_console_output_character(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let str_ptr = api_try!(emu, call.arg(emu, 0));
    let arg2 = api_try!(emu, call.arg(emu, 1));
    let s = strings::read_text(emu, str_ptr, wide);
    log_red!(emu, "kernel32!{api} '{}'  0x{:x}", s, arg2);
    call.cleanup_args(emu, 5);
    call.set_return(emu, 1);
}

pub fn ReadConsoleOutputCharacterW(emu: &mut emu::Emu) {
    read_console_output_character(emu, "ReadConsoleOutputCharacterW", true);
}
