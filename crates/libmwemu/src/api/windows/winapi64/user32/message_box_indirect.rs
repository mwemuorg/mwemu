use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::policy::api_try;

pub fn MessageBoxIndirectA(emu: &mut emu::Emu) {
    message_box_indirect(emu, "MessageBoxIndirectA", false);
}

fn message_box_indirect(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let hwnd = api_try!(emu, call.arg(emu, 0));
    let text_ptr = api_try!(emu, call.arg(emu, 1));
    let caption_ptr = api_try!(emu, call.arg(emu, 2));
    let mb_type = api_try!(emu, call.arg(emu, 3));
    let text = if text_ptr != 0 {
        strings::read_text(emu, text_ptr, wide)
    } else {
        String::new()
    };
    let caption = if caption_ptr != 0 {
        strings::read_text(emu, caption_ptr, wide)
    } else {
        String::new()
    };
    log_red!(emu, "user32!{api} '{}': '{}'", caption, text);
    emu.set_last_error(0);
    call.set_return(emu, 1);
}

pub fn MessageBoxIndirectW(emu: &mut emu::Emu) {
    message_box_indirect(emu, "MessageBoxIndirectW", true);
}
