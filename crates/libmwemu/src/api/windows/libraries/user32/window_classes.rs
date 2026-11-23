use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu::Emu;
use crate::winapi::common::memory::read_pointer;
use crate::winapi::common::process::current_process_id;
use crate::winapi::common::trace::input_string;
use crate::winapi::environment_store::Encoding;
use crate::winapi::policy::{self, api_try};
use crate::winapi::window_table::{UnregisterError, Window};
use crate::windows::constants::controls::{WS_CHILD, WS_DISABLED};
use crate::windows::constants::errors::*;

const WNDCLASS32_NAME: u64 = 36;
const WNDCLASSEX32_NAME: u64 = 40;
const WNDCLASS64_NAME: u64 = 64;
const WNDCLASSEX32_SIZE: u32 = 48;
const WNDCLASSEX64_SIZE: u32 = 80;

fn string(emu: &Emu, address: u64, wide: bool) -> Result<String, u64> {
    let units = Encoding::from_wide(wide).read(emu, address)?;
    Ok(String::from_utf16_lossy(&units))
}

fn class(emu: &Emu, address: u64, wide: bool) -> Result<String, u64> {
    if address == 0 {
        return Err(ERROR_INVALID_PARAMETER);
    }
    if let Ok(atom) = u16::try_from(address) {
        return emu
            .guest_windows
            .class_name(atom)
            .map(str::to_owned)
            .ok_or(ERROR_CLASS_DOES_NOT_EXIST);
    }
    string(emu, address, wide)
}

fn register(emu: &mut Emu, api: &str, wide: bool, extended: bool) {
    let call = WinApi(Stdcall);
    let record = api_try!(emu, call.arg(emu, 0));
    let is64 = emu.cfg.arch.is_64bits();
    let offset = if is64 {
        WNDCLASS64_NAME
    } else if extended {
        WNDCLASSEX32_NAME
    } else {
        WNDCLASS32_NAME
    };
    let result = (|| {
        if record == 0
            || (extended
                && emu.maps.read_dword(record)
                    != Some(if is64 {
                        WNDCLASSEX64_SIZE
                    } else {
                        WNDCLASSEX32_SIZE
                    }))
        {
            return Err(ERROR_INVALID_PARAMETER);
        }
        let ptr = record
            .checked_add(offset)
            .and_then(|p| read_pointer(emu, p))
            .ok_or(ERROR_NOACCESS)?;
        let name = string(emu, ptr, wide)?;
        log_red!(emu, "user32!{api} class_name={name:?}");
        if name.is_empty() {
            return Err(ERROR_INVALID_PARAMETER);
        }
        emu.guest_windows
            .register_class(&name, wide)
            .map(u64::from)
            .ok_or(ERROR_CLASS_ALREADY_EXISTS)
    })();
    let atom = result.unwrap_or_else(|e| {
        emu.set_last_error(e);
        0
    });
    call.set_return(emu, atom);
    call.cleanup_args(emu, 1);
}

fn unregister(emu: &mut Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let [name, instance] = api_try!(emu, call.args(emu));
    log_red!(
        emu,
        "user32!{api} class_identifier={name:#x} instance={instance:#x}"
    );
    let name = class(emu, name, wide);
    let result = name.and_then(|n| {
        emu.guest_windows
            .unregister_class(&n)
            .map_err(|error| match error {
                UnregisterError::NotRegistered => ERROR_CLASS_DOES_NOT_EXIST,
                UnregisterError::HasWindows => ERROR_CLASS_HAS_WINDOWS,
            })
    });
    if let Err(error) = result {
        emu.set_last_error(error);
    }
    call.set_return(emu, result.is_ok().into());
    call.cleanup_args(emu, 2);
}

fn create(emu: &mut Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let [
        extended_style,
        class_ptr,
        title_ptr,
        style,
        x,
        y,
        width,
        height,
        parent,
        menu,
        instance,
        _parameter,
    ] = api_try!(emu, call.args(emu));
    if parent != 0 && style as u32 & WS_CHILD == 0 {
        // A top-level owner has separate lifetime and enumeration rules.
        policy::unsupported(emu, "user32", api, emu.pc());
        return;
    }
    let result = (|| {
        let class = class(emu, class_ptr, wide)?;
        let title = if title_ptr == 0 {
            String::new()
        } else {
            string(emu, title_ptr, wide)?
        };
        log_red!(
            emu,
            "user32!{api} class_name={class:?} window_name={title:?} parent={parent:#x} style={:#x}",
            style as u32
        );
        if (parent != 0 || style as u32 & WS_CHILD != 0) && !emu.guest_windows.contains(parent) {
            return Err(ERROR_INVALID_WINDOW_HANDLE);
        }
        let wide = emu.guest_windows.class_wide(&class).unwrap_or(wide);
        let process = current_process_id(emu) as u32;
        let thread = emu.current_thread().id;
        let control_id = if style as u32 & WS_CHILD != 0 {
            menu
        } else {
            0
        };
        emu.guest_windows
            .insert(Window {
                style: style as u32,
                extended_style: extended_style as u32,
                instance,
                user_data: 0,
                class,
                title,
                parent,
                control_id,
                thread,
                process,
                wide,
                enabled: style as u32 & WS_DISABLED == 0,
                bounds: [x as i32, y as i32, width as i32, height as i32],
                menu: if style as u32 & WS_CHILD == 0 {
                    menu
                } else {
                    0
                },
            })
            .ok_or(ERROR_NOT_ENOUGH_MEMORY)
    })();
    let handle = result.unwrap_or_else(|e| {
        emu.set_last_error(e);
        0
    });
    call.set_return(emu, handle);
    call.cleanup_args(emu, 12);
}

fn find(emu: &mut Emu, api: &str, wide: bool, extended: bool) {
    let call = WinApi(Stdcall);
    let (parent, after, class_ptr, title_ptr) = if extended {
        let [p, a, c, t] = api_try!(emu, call.args(emu));
        (p, a, c, t)
    } else {
        let [c, t] = api_try!(emu, call.args(emu));
        (0, 0, c, t)
    };
    let class = (class_ptr != 0)
        .then(|| class(emu, class_ptr, wide))
        .transpose();
    let title = (title_ptr != 0)
        .then(|| string(emu, title_ptr, wide))
        .transpose();
    log_red!(
        emu,
        "user32!{api} parent={parent:#x} after={after:#x} class_identifier={class_ptr:#x} window_name={}",
        input_string(emu, title_ptr, Encoding::from_wide(wide))
    );
    let hwnd = match (class, title) {
        (Ok(class), Ok(title)) => {
            emu.guest_windows
                .find(parent, after, class.as_deref(), title.as_deref())
        }
        _ => 0,
    };
    call.set_return(emu, hwnd);
    call.cleanup_args(emu, if extended { 4 } else { 2 });
}

pub fn DestroyWindow(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let hwnd = api_try!(emu, call.arg(emu, 0));
    log_red!(emu, "user32!DestroyWindow hwnd={hwnd:#x}");
    let success = emu.guest_windows.destroy(hwnd);
    if success {
        emu.user_timers.remove_destroyed_windows(&emu.guest_windows);
    } else {
        emu.set_last_error(ERROR_INVALID_WINDOW_HANDLE);
    }
    call.set_return(emu, success.into());
    call.cleanup_args(emu, 1);
}

pub fn RegisterClassA(emu: &mut Emu) {
    register(emu, "RegisterClassA", false, false);
}

pub fn RegisterClassW(emu: &mut Emu) {
    register(emu, "RegisterClassW", true, false);
}

pub fn RegisterClassExA(emu: &mut Emu) {
    register(emu, "RegisterClassExA", false, true);
}

pub fn RegisterClassExW(emu: &mut Emu) {
    register(emu, "RegisterClassExW", true, true);
}

pub fn UnregisterClassA(emu: &mut Emu) {
    unregister(emu, "UnregisterClassA", false);
}

pub fn UnregisterClassW(emu: &mut Emu) {
    unregister(emu, "UnregisterClassW", true);
}

pub fn CreateWindowExA(emu: &mut Emu) {
    create(emu, "CreateWindowExA", false);
}

pub fn CreateWindowExW(emu: &mut Emu) {
    create(emu, "CreateWindowExW", true);
}

pub fn FindWindowA(emu: &mut Emu) {
    find(emu, "FindWindowA", false, false);
}

pub fn FindWindowW(emu: &mut Emu) {
    find(emu, "FindWindowW", true, false);
}

pub fn FindWindowExA(emu: &mut Emu) {
    find(emu, "FindWindowExA", false, true);
}

pub fn FindWindowExW(emu: &mut Emu) {
    find(emu, "FindWindowExW", true, true);
}
