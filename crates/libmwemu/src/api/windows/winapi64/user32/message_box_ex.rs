use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::policy::api_try;

pub fn MessageBoxExA(emu: &mut emu::Emu) {
    message_box_ex(emu, "MessageBoxExA", false);
}

fn message_box_ex(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let hwnd = api_try!(emu, call.arg(emu, 0));
    let text_ptr = api_try!(emu, call.arg(emu, 1));
    let caption_ptr = api_try!(emu, call.arg(emu, 2));
    let mtype = api_try!(emu, call.arg_u32(emu, 3));
    let _lang_id = api_try!(emu, call.arg(emu, 4));
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
        "user32!{api} '{}' '{}' type:0x{:x}",
        caption,
        text,
        mtype
    );
    emu.set_last_error(0);
    call.set_return(emu, 1); // IDOK
}

pub fn MessageBoxExW(emu: &mut emu::Emu) {
    message_box_ex(emu, "MessageBoxExW", true);
}
