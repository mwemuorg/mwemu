use super::window_state::DESKTOP_WINDOW;
use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu::Emu;
use crate::winapi::policy::{self, api_try};
use crate::windows::constants::controls::{
    GWL_EXSTYLE, GWL_STYLE, GWLP_HINSTANCE, GWLP_ID, GWLP_USERDATA, WS_DISABLED,
};
use crate::windows::constants::errors::{
    ERROR_INVALID_PARAMETER, ERROR_INVALID_WINDOW_HANDLE, ERROR_NOACCESS,
};

pub fn GetParent(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let hwnd = api_try!(emu, call.arg(emu, 0));
    log_red!(emu, "user32!GetParent hwnd={hwnd:#x}");
    let parent = emu.guest_windows.get(hwnd).map(|w| w.parent);
    if parent.is_none() {
        emu.set_last_error(ERROR_INVALID_WINDOW_HANDLE);
    }
    call.set_return(emu, parent.unwrap_or(0));
    call.cleanup_args(emu, 1);
}

pub fn SetParent(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let [hwnd, new_parent] = api_try!(emu, call.args(emu));
    log_red!(
        emu,
        "user32!SetParent hwnd={hwnd:#x} new_parent={new_parent:#x}"
    );
    let previous = if !emu.guest_windows.contains(hwnd)
        || new_parent != 0 && !emu.guest_windows.contains(new_parent)
    {
        emu.set_last_error(ERROR_INVALID_WINDOW_HANDLE);
        0
    } else if hwnd == new_parent || emu.guest_windows.is_child(hwnd, new_parent) {
        emu.set_last_error(ERROR_INVALID_PARAMETER);
        0
    } else if let Some(window) = emu.guest_windows.get_mut(hwnd) {
        std::mem::replace(&mut window.parent, new_parent)
    } else {
        0
    };
    call.set_return(emu, previous);
    call.cleanup_args(emu, 2);
}

pub fn IsChild(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let [parent, hwnd] = api_try!(emu, call.args(emu));
    log_red!(emu, "user32!IsChild parent={parent:#x} hwnd={hwnd:#x}");
    let child = emu.guest_windows.is_child(parent, hwnd);
    call.set_return(emu, child.into());
    call.cleanup_args(emu, 2);
}

pub fn IsWindow(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let hwnd = api_try!(emu, call.arg(emu, 0));
    log_red!(emu, "user32!IsWindow hwnd={hwnd:#x}");
    call.set_return(
        emu,
        u64::from(hwnd == DESKTOP_WINDOW || emu.guest_windows.contains(hwnd)),
    );
    call.cleanup_args(emu, 1);
}

pub fn IsWindowUnicode(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let hwnd = api_try!(emu, call.arg(emu, 0));
    log_red!(emu, "user32!IsWindowUnicode hwnd={hwnd:#x}");
    call.set_return(
        emu,
        emu.guest_windows.get(hwnd).is_some_and(|w| w.wide).into(),
    );
    call.cleanup_args(emu, 1);
}

pub fn IsWindowEnabled(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let hwnd = api_try!(emu, call.arg(emu, 0));
    log_red!(emu, "user32!IsWindowEnabled hwnd={hwnd:#x}");
    call.set_return(
        emu,
        emu.guest_windows
            .get(hwnd)
            .is_some_and(|w| w.enabled)
            .into(),
    );
    call.cleanup_args(emu, 1);
}

pub fn EnableWindow(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let [hwnd, enable] = api_try!(emu, call.args(emu));
    log_red!(emu, "user32!EnableWindow hwnd={hwnd:#x} enable={enable}");
    let disabled = emu.guest_windows.get_mut(hwnd).is_some_and(|window| {
        let previous = window.enabled;
        window.enabled = enable as u32 != 0;
        if window.enabled {
            window.style &= !WS_DISABLED;
        } else {
            window.style |= WS_DISABLED;
        }
        !previous
    });
    call.set_return(emu, disabled.into());
    call.cleanup_args(emu, 2);
}

pub fn GetDlgCtrlID(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let hwnd = api_try!(emu, call.arg(emu, 0));
    log_red!(emu, "user32!GetDlgCtrlID hwnd={hwnd:#x}");
    let id = emu.guest_windows.get(hwnd).map(|w| w.control_id as u32);
    if id.is_none() {
        emu.set_last_error(ERROR_INVALID_WINDOW_HANDLE);
    }
    call.set_return(emu, id.unwrap_or(0).into());
    call.cleanup_args(emu, 1);
}

pub fn GetDlgItem(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let parent = api_try!(emu, call.arg(emu, 0));
    let id = api_try!(emu, call.arg_u32(emu, 1));
    log_red!(emu, "user32!GetDlgItem dialog={parent:#x} control_id={id}");
    let hwnd = emu.guest_windows.control(parent, id);
    call.set_return(emu, hwnd);
    call.cleanup_args(emu, 2);
}

pub fn GetWindowThreadProcessId(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let [hwnd, process_id] = api_try!(emu, call.args(emu));
    log_red!(
        emu,
        "user32!GetWindowThreadProcessId hwnd={hwnd:#x} process_id={process_id:#x}"
    );
    let result = emu
        .guest_windows
        .get(hwnd)
        .ok_or(ERROR_INVALID_WINDOW_HANDLE)
        .and_then(|w| {
            if process_id != 0
                && (!emu
                    .maps
                    .validate_write_range(process_id, size_of::<u32>() as u64)
                    || !emu.maps.write_dword(process_id, w.process))
            {
                return Err(ERROR_NOACCESS);
            }
            Ok(w.thread)
        });
    let thread = result.unwrap_or_else(|error| {
        emu.set_last_error(error);
        0
    });
    call.set_return(emu, thread);
    call.cleanup_args(emu, 2);
}

fn window_long(emu: &mut Emu, api: &str, setter: bool, pointer_width: bool) {
    let call = WinApi(Stdcall);
    let hwnd = api_try!(emu, call.arg(emu, 0));
    let index = api_try!(emu, call.arg_i32(emu, 1));
    let value = if setter {
        let raw = api_try!(emu, call.arg(emu, 2));
        Some(if pointer_width {
            raw
        } else {
            u64::from(raw as u32)
        })
    } else {
        None
    };
    log_red!(emu, "user32!{api} hwnd={hwnd:#x} index={index}");
    let previous = if let Some(window) = emu.guest_windows.get_mut(hwnd) {
        match index {
            GWL_STYLE => {
                let prev = window.style;
                if let Some(v) = value {
                    window.style = v as u32;
                    window.enabled = window.style & WS_DISABLED == 0;
                }
                prev.into()
            }
            GWL_EXSTYLE => {
                let prev = window.extended_style;
                if let Some(v) = value {
                    window.extended_style = v as u32;
                }
                prev.into()
            }
            GWLP_ID | GWLP_USERDATA | GWLP_HINSTANCE => {
                let slot = match index {
                    GWLP_ID => &mut window.control_id,
                    GWLP_USERDATA => &mut window.user_data,
                    _ => &mut window.instance,
                };
                match value {
                    Some(v) => std::mem::replace(slot, v),
                    None => *slot,
                }
            }
            _ => {
                policy::unsupported(emu, "user32", api, emu.pc());
                return;
            }
        }
    } else {
        emu.set_last_error(ERROR_INVALID_WINDOW_HANDLE);
        0
    };
    call.set_return(
        emu,
        if pointer_width {
            previous
        } else {
            u64::from(previous as u32)
        },
    );
    call.cleanup_args(emu, 2 + usize::from(setter));
}

pub fn GetWindowLongA(emu: &mut Emu) {
    window_long(emu, "GetWindowLongA", false, false);
}

pub fn GetWindowLongW(emu: &mut Emu) {
    window_long(emu, "GetWindowLongW", false, false);
}

pub fn GetWindowLongPtrA(emu: &mut Emu) {
    window_long(emu, "GetWindowLongPtrA", false, true);
}

pub fn GetWindowLongPtrW(emu: &mut Emu) {
    window_long(emu, "GetWindowLongPtrW", false, true);
}

pub fn SetWindowLongA(emu: &mut Emu) {
    window_long(emu, "SetWindowLongA", true, false);
}

pub fn SetWindowLongW(emu: &mut Emu) {
    window_long(emu, "SetWindowLongW", true, false);
}

pub fn SetWindowLongPtrA(emu: &mut Emu) {
    window_long(emu, "SetWindowLongPtrA", true, true);
}

pub fn SetWindowLongPtrW(emu: &mut Emu) {
    window_long(emu, "SetWindowLongPtrW", true, true);
}
