use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;
use crate::windows::constants::errors::*;

pub fn GetConsoleKeyboardLayoutNameA(emu: &mut emu::Emu) {
    get_console_keyboard_layout_name(emu, "GetConsoleKeyboardLayoutNameA");
}

fn get_console_keyboard_layout_name(emu: &mut emu::Emu, api: &str) {
    let call = WinApi(Stdcall);
    let _a1 = api_try!(emu, call.arg_u32(emu, 0));

    log_red!(emu, "kernel32!{api}");

    call.set_return(emu, ERROR_SUCCESS);
    call.cleanup_args(emu, 1);
}

pub fn GetConsoleKeyboardLayoutNameW(emu: &mut emu::Emu) {
    get_console_keyboard_layout_name(emu, "GetConsoleKeyboardLayoutNameW");
}
