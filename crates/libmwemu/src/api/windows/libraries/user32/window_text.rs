use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu::Emu;
use crate::winapi::common::trace::input_string;
use crate::winapi::environment_store::Encoding;
use crate::winapi::policy::api_try;
use crate::windows::constants::errors::{ERROR_INVALID_WINDOW_HANDLE, ERROR_NOACCESS};

pub(super) fn window_text(
    emu: &Emu,
    hwnd: u64,
    encoding: Encoding,
    class_name: bool,
) -> Result<Vec<u8>, u64> {
    let title = emu
        .guest_windows
        .get(hwnd)
        .map(|w| if class_name { &w.class } else { &w.title })
        .ok_or(ERROR_INVALID_WINDOW_HANDLE)?;
    Ok(encoding.encode(&title.encode_utf16().collect::<Vec<_>>()))
}

fn text_length(emu: &mut Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let hwnd = api_try!(emu, call.arg(emu, 0));
    let encoding = Encoding::from_wide(wide);
    let length =
        window_text(emu, hwnd, encoding, false).map(|bytes| bytes.len() / encoding.width());
    log_red!(emu, "user32!{api} hwnd={hwnd:#x}");
    let result = length.unwrap_or_else(|error| {
        emu.set_last_error(error);
        0
    });
    call.set_return(emu, result as u64);
    call.cleanup_args(emu, 1);
}

fn get_text(emu: &mut Emu, api: &str, wide: bool, class_name: bool, dialog: bool) {
    let call = WinApi(Stdcall);
    let hwnd = api_try!(emu, call.arg(emu, 0));
    let hwnd = if dialog {
        let id = api_try!(emu, call.arg_u32(emu, 1));
        emu.guest_windows.control(hwnd, id)
    } else {
        hwnd
    };
    let buffer = api_try!(emu, call.arg(emu, 1 + usize::from(dialog)));
    let capacity = api_try!(emu, call.arg_i32(emu, 2 + usize::from(dialog)));
    log_red!(
        emu,
        "user32!{api} hwnd={hwnd:#x} buffer={buffer:#x} capacity={capacity}"
    );
    let encoding = Encoding::from_wide(wide);
    let result = copy_text(emu, hwnd, encoding, class_name, buffer, capacity);
    let length = result.unwrap_or_else(|error| {
        emu.set_last_error(error);
        0
    });
    call.set_return(emu, length as u64);
    call.cleanup_args(emu, 3 + usize::from(dialog));
}

fn set_text(emu: &mut Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let [hwnd, text] = api_try!(emu, call.args(emu));
    let encoding = Encoding::from_wide(wide);
    log_red!(
        emu,
        "user32!{api} hwnd={hwnd:#x} text={}",
        input_string(emu, text, encoding)
    );
    let title = if text == 0 {
        Ok(Vec::new())
    } else {
        encoding.read(emu, text)
    };
    let result = title.and_then(|units| {
        let window = emu
            .guest_windows
            .get_mut(hwnd)
            .ok_or(ERROR_INVALID_WINDOW_HANDLE)?;
        window.title = String::from_utf16_lossy(&units);
        Ok(())
    });
    if let Err(error) = result {
        emu.set_last_error(error);
    }
    call.set_return(emu, result.is_ok().into());
    call.cleanup_args(emu, 2);
}

pub fn GetWindowTextLengthA(emu: &mut Emu) {
    text_length(emu, "GetWindowTextLengthA", false);
}

pub fn GetWindowTextLengthW(emu: &mut Emu) {
    text_length(emu, "GetWindowTextLengthW", true);
}

pub fn GetWindowTextA(emu: &mut Emu) {
    get_text(emu, "GetWindowTextA", false, false, false);
}

pub fn GetWindowTextW(emu: &mut Emu) {
    get_text(emu, "GetWindowTextW", true, false, false);
}

pub fn SetWindowTextA(emu: &mut Emu) {
    set_text(emu, "SetWindowTextA", false);
}

pub fn SetWindowTextW(emu: &mut Emu) {
    set_text(emu, "SetWindowTextW", true);
}

pub fn GetClassNameA(emu: &mut Emu) {
    get_text(emu, "GetClassNameA", false, true, false);
}

pub fn GetClassNameW(emu: &mut Emu) {
    get_text(emu, "GetClassNameW", true, true, false);
}

pub fn GetDlgItemTextA(emu: &mut Emu) {
    get_text(emu, "GetDlgItemTextA", false, false, true);
}

pub fn GetDlgItemTextW(emu: &mut Emu) {
    get_text(emu, "GetDlgItemTextW", true, false, true);
}

pub(super) fn copy_text(
    emu: &mut Emu,
    hwnd: u64,
    encoding: Encoding,
    class_name: bool,
    buffer: u64,
    capacity: i32,
) -> Result<usize, u64> {
    window_text(emu, hwnd, encoding, class_name).and_then(|mut bytes| {
        if capacity <= 0 {
            return Ok(0);
        }
        bytes.truncate((capacity as usize - 1) * encoding.width());
        let copied = bytes.len() / encoding.width();
        bytes.resize(bytes.len() + encoding.width(), 0);
        if buffer == 0
            || !emu.maps.validate_write_range(buffer, bytes.len() as u64)
            || !emu.maps.write_bytes(buffer, &bytes)
        {
            return Err(ERROR_NOACCESS);
        }
        Ok(copied)
    })
}
