use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;
use crate::winapi::window_table;
use crate::windows::constants::errors::*;

pub(super) fn DestroyWindow(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let hwnd = api_try!(emu, call.arg(emu, 0));
    log_red!(emu, "user32!DestroyWindow hwnd:0x{:x}", hwnd);
    if helper::handler_exist(hwnd) {
        helper::handler_close(hwnd);
    }
    call.set_return(emu, 1);
    if window_table::destroy_window(hwnd) {
        call.set_return(emu, 1);
    } else {
        emu.set_last_error(ERROR_INVALID_WINDOW_HANDLE);
        call.set_return(emu, 0);
    }
}
