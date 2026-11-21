use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::policy::api_try;

pub fn GetWindowTextLengthA(emu: &mut emu::Emu) {
    get_window_text_length(emu, "GetWindowTextLengthA", false);
}

fn get_window_text_length(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let hwnd = api_try!(emu, call.arg(emu, 0));
    let buf_ptr = api_try!(emu, call.arg(emu, 1));
    let max_count = api_try!(emu, call.arg(emu, 2));
    let title = "mwemu_window";
    if buf_ptr != 0 && max_count > 0 {
        strings::write_string(emu, buf_ptr, title, wide);
    }
    log_red!(emu, "user32!{api} hwnd:0x{:x} => '{}'", hwnd, title);

    emu.set_last_error(0);
    call.set_return(emu, title.len() as u64);
}

pub fn GetWindowTextLengthW(emu: &mut emu::Emu) {
    get_window_text_length(emu, "GetWindowTextLengthW", true);
}
