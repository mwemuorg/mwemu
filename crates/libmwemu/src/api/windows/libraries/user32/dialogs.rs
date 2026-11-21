use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu::Emu;
use crate::winapi::common::strings;
use crate::winapi::policy::api_try;
use crate::windows::constants::errors::{ERROR_INVALID_PARAMETER, ERROR_NOACCESS};
use crate::windows::constants::system::IDOK;

const MB_TYPEMASK: u32 = 0xf;
const MB_DEFMASK: u32 = 0xf00;
const IDCANCEL: u64 = 2;
const IDABORT: u64 = 3;
const IDRETRY: u64 = 4;
const IDIGNORE: u64 = 5;
const IDYES: u64 = 6;
const IDNO: u64 = 7;
const IDTRYAGAIN: u64 = 10;
const IDCONTINUE: u64 = 11;

fn message_box(emu: &mut Emu, api: &str, wide: bool, extended: bool) {
    let call = WinApi(Stdcall);
    let [hwnd, text, caption, style] = api_try!(emu, call.args(emu));
    let language = if extended {
        api_try!(emu, call.arg_u16(emu, 4))
    } else {
        0
    };
    let message = if text == 0 {
        Some(String::new())
    } else {
        strings::read_text_checked(emu, text, wide)
    };
    let title = if caption == 0 {
        Some("Error".to_string())
    } else {
        strings::read_text_checked(emu, caption, wide)
    };
    let style = style as u32;
    log_red!(
        emu,
        "user32!{api} hwnd={hwnd:#x} text={message:?} caption={title:?} style={style:#x} language={language}"
    );
    let buttons: &[u64] = match style & MB_TYPEMASK {
        0 => &[IDOK],
        1 => &[IDOK, IDCANCEL],
        2 => &[IDABORT, IDRETRY, IDIGNORE],
        3 => &[IDYES, IDNO, IDCANCEL],
        4 => &[IDYES, IDNO],
        5 => &[IDRETRY, IDCANCEL],
        6 => &[IDCANCEL, IDTRYAGAIN, IDCONTINUE],
        _ => &[],
    };
    let button = if title.is_none() || message.is_none() {
        emu.set_last_error(ERROR_NOACCESS);
        0
    } else if buttons.is_empty() {
        emu.set_last_error(ERROR_INVALID_PARAMETER);
        0
    } else {
        buttons
            .get(((style & MB_DEFMASK) >> 8) as usize)
            .copied()
            .unwrap_or(buttons[0])
    };
    call.set_return(emu, button);
    call.cleanup_args(emu, 4 + usize::from(extended));
}
pub fn MessageBoxA(emu: &mut Emu) {
    message_box(emu, "MessageBoxA", false, false);
}

pub fn MessageBoxW(emu: &mut Emu) {
    message_box(emu, "MessageBoxW", true, false);
}

pub fn MessageBoxExA(emu: &mut Emu) {
    message_box(emu, "MessageBoxExA", false, true);
}

pub fn MessageBoxExW(emu: &mut Emu) {
    message_box(emu, "MessageBoxExW", true, true);
}
