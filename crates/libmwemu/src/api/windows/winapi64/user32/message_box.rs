use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::policy::api_try;

pub(super) fn MessageBoxA(emu: &mut emu::Emu) {
    message_box(emu, "MessageBoxA", false);
}

pub fn MessageBoxW(emu: &mut emu::Emu) {
    message_box(emu, "MessageBoxW", true);
}

fn message_box(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let hwnd = api_try!(emu, call.arg(emu, 0));
    let text_ptr = api_try!(emu, call.arg(emu, 1));
    let caption_ptr = api_try!(emu, call.arg(emu, 2));
    let utype = api_try!(emu, call.arg(emu, 3));
    if wide {
        let text = strings::read_text(emu, text_ptr, wide);
        let caption = if caption_ptr != 0 {
            strings::read_text(emu, caption_ptr, wide)
        } else {
            String::new()
        };
        log_red!(
            emu,
            "user32!{api} hwnd=0x{:x} caption='{}' text='{}' type=0x{:x}",
            hwnd,
            caption,
            text,
            utype
        );
        call.set_return(emu, 1);
    } else {
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
        log_red!(
            emu,
            "user32!{api} hwnd=0x{:x} caption='{}' text='{}' type=0x{:x}",
            hwnd,
            caption,
            text,
            utype
        );
        call.set_return(emu, 1);
    }
}
