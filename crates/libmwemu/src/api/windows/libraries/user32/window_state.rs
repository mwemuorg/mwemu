use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu::Emu;
use crate::winapi::policy::{self, api_try};
use crate::windows::constants::errors::{
    ERROR_INVALID_PARAMETER, ERROR_INVALID_WINDOW_HANDLE, ERROR_NOACCESS,
};

const WS_VISIBLE: u32 = 0x1000_0000;
const WS_MINIMIZE: u32 = 0x2000_0000;
const SW_HIDE: u32 = 0;
const SW_SHOWMINIMIZED: u32 = 2;
const SW_MINIMIZE: u32 = 6;
const SW_SHOWMINNOACTIVE: u32 = 7;
const SWP_NOSIZE: u32 = 1;
const SWP_NOMOVE: u32 = 2;
const SWP_NOZORDER: u32 = 4;
const SWP_SHOWWINDOW: u32 = 0x40;
const SWP_HIDEWINDOW: u32 = 0x80;
pub(super) const DESKTOP_WINDOW: u64 = 0x0001_0000;
pub(super) const SCREEN_WIDTH: i32 = 1920;
pub(super) const SCREEN_HEIGHT: i32 = 1080;
pub(super) const WORK_HEIGHT: i32 = 1040;
const CW_USEDEFAULT: i32 = i32::MIN;

fn extent(value: i32, default: i32) -> i32 {
    if value == CW_USEDEFAULT {
        default
    } else {
        value
    }
}

fn rectangle(emu: &mut Emu, api: &str, client: bool) {
    let call = WinApi(Stdcall);
    let [window, output] = api_try!(emu, call.args(emu));
    let geometry = if window == DESKTOP_WINDOW {
        Some([0, 0, SCREEN_WIDTH, SCREEN_HEIGHT])
    } else {
        emu.guest_windows.get(window).map(|w| w.bounds)
    };
    let result = if let Some([x, y, width, height]) = geometry {
        let mut origin = if client {
            [0, 0]
        } else {
            [extent(x, 0), extent(y, 0)]
        };
        if !client {
            let mut parent = emu.guest_windows.get(window).map_or(0, |w| w.parent);
            while let Some(w) = emu.guest_windows.get(parent) {
                origin[0] = origin[0].wrapping_add(extent(w.bounds[0], 0));
                origin[1] = origin[1].wrapping_add(extent(w.bounds[1], 0));
                parent = w.parent;
            }
        }
        let [x, y] = origin;
        let rect = [
            x,
            y,
            x.wrapping_add(extent(width, SCREEN_WIDTH).max(0)),
            y.wrapping_add(extent(height, WORK_HEIGHT).max(0)),
        ]
        .map(i32::to_le_bytes);
        if output != 0
            && emu.maps.validate_write_range(output, 16)
            && emu.maps.write_bytes(output, rect.as_flattened())
        {
            true
        } else {
            emu.set_last_error(ERROR_NOACCESS);
            false
        }
    } else {
        emu.set_last_error(ERROR_INVALID_WINDOW_HANDLE);
        false
    };
    log_red!(
        emu,
        "user32!{api} hwnd={window:#x} rect={output:#x} success={result}"
    );
    call.set_return(emu, result.into());
    call.cleanup_args(emu, 2);
}

fn visible(emu: &Emu, mut hwnd: u64) -> bool {
    if hwnd == DESKTOP_WINDOW {
        return true;
    }
    while let Some(w) = emu.guest_windows.get(hwnd) {
        if w.style & WS_VISIBLE == 0 {
            return false;
        }
        hwnd = w.parent;
        if hwnd == 0 {
            return true;
        }
    }
    false
}

fn show(emu: &mut Emu, api: &str, close: bool) {
    let call = WinApi(Stdcall);
    let hwnd = api_try!(emu, call.arg(emu, 0));
    let command = if close {
        SW_MINIMIZE
    } else {
        api_try!(emu, call.arg_u32(emu, 1))
    };
    log_red!(emu, "user32!{api} hwnd={hwnd:#x} command={command}");
    if command > 11 {
        emu.set_last_error(ERROR_INVALID_PARAMETER);
        call.set_return(emu, 0);
    } else if let Some(w) = emu.guest_windows.get_mut(hwnd) {
        let previous = w.style & WS_VISIBLE != 0;
        if command == SW_HIDE {
            w.style &= !WS_VISIBLE;
        } else {
            w.style |= WS_VISIBLE;
            if matches!(
                command,
                SW_MINIMIZE | SW_SHOWMINIMIZED | SW_SHOWMINNOACTIVE | 11
            ) {
                w.style |= WS_MINIMIZE;
            } else {
                w.style &= !WS_MINIMIZE;
            }
        }
        call.set_return(emu, u64::from(close || previous));
    } else {
        emu.set_last_error(ERROR_INVALID_WINDOW_HANDLE);
        call.set_return(emu, 0);
    }
    call.cleanup_args(emu, if close { 1 } else { 2 });
}
pub fn GetClientRect(emu: &mut Emu) {
    rectangle(emu, "GetClientRect", true);
}

pub fn GetWindowRect(emu: &mut Emu) {
    rectangle(emu, "GetWindowRect", false);
}

pub fn CloseWindow(emu: &mut Emu) {
    show(emu, "CloseWindow", true);
}

pub fn ShowWindow(emu: &mut Emu) {
    show(emu, "ShowWindow", false);
}

pub fn GetDesktopWindow(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    log_red!(emu, "user32!GetDesktopWindow");
    call.set_return(emu, DESKTOP_WINDOW);
    call.cleanup_args(emu, 0);
}

pub fn GetTopWindow(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let parent = api_try!(emu, call.arg(emu, 0));
    let result = emu.guest_windows.find(
        if parent == DESKTOP_WINDOW { 0 } else { parent },
        0,
        None,
        None,
    );
    log_red!(emu, "user32!GetTopWindow hwnd={parent:#x} top={result:#x}");
    call.set_return(emu, result);
    call.cleanup_args(emu, 1);
}

pub fn IsWindowVisible(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let hwnd = api_try!(emu, call.arg(emu, 0));
    let value = visible(emu, hwnd);
    log_red!(emu, "user32!IsWindowVisible hwnd={hwnd:#x} visible={value}");
    call.set_return(emu, value.into());
    call.cleanup_args(emu, 1);
}

pub fn GetFocus(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let window = emu
        .guest_windows
        .user
        .focus
        .get(&emu.current_thread().id)
        .copied()
        .unwrap_or(0);
    let result = if emu.guest_windows.contains(window) {
        window
    } else {
        0
    };
    log_red!(emu, "user32!GetFocus hwnd={result:#x}");
    call.set_return(emu, result);
}

pub fn GetForegroundWindow(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let window = emu.guest_windows.user.foreground;
    let result = if emu.guest_windows.contains(window) {
        window
    } else {
        0
    };
    log_red!(emu, "user32!GetForegroundWindow hwnd={result:#x}");
    call.set_return(emu, result);
}

pub fn SetFocus(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let hwnd = api_try!(emu, call.arg(emu, 0));
    let thread = emu.current_thread().id;
    let old = if hwnd != 0
        && emu
            .guest_windows
            .get(hwnd)
            .is_none_or(|w| w.thread != thread || !w.enabled)
    {
        emu.set_last_error(ERROR_INVALID_WINDOW_HANDLE);
        0
    } else {
        let old = emu
            .guest_windows
            .user
            .focus
            .insert(thread, hwnd)
            .unwrap_or(0);
        if emu.guest_windows.contains(old) {
            old
        } else {
            0
        }
    };
    log_red!(emu, "user32!SetFocus hwnd={hwnd:#x} previous={old:#x}");
    call.set_return(emu, old);
    call.cleanup_args(emu, 1);
}

pub fn SetForegroundWindow(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let hwnd = api_try!(emu, call.arg(emu, 0));
    let ok = emu.guest_windows.contains(hwnd);
    if ok {
        emu.guest_windows.user.foreground = hwnd;
    }
    log_red!(
        emu,
        "user32!SetForegroundWindow hwnd={hwnd:#x} success={ok}"
    );
    call.set_return(emu, ok.into());
    call.cleanup_args(emu, 1);
}

pub fn SetWindowPos(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let [hwnd, insert_after, x, y, width, height, flags] = api_try!(emu, call.args(emu));
    let flags = flags as u32;
    log_red!(
        emu,
        "user32!SetWindowPos hwnd={hwnd:#x} insert_after={insert_after:#x} x={x} y={y} width={width} height={height} flags={flags:#x}"
    );
    if !emu.guest_windows.contains(hwnd) {
        emu.set_last_error(ERROR_INVALID_WINDOW_HANDLE);
        call.set_return(emu, 0);
    } else if flags & SWP_NOZORDER == 0 {
        policy::unsupported(emu, "user32", "SetWindowPos", emu.pc());
        return;
    } else if let Some(w) = emu.guest_windows.get_mut(hwnd) {
        if flags & SWP_NOMOVE == 0 {
            w.bounds[0] = x as i32;
            w.bounds[1] = y as i32;
        }
        if flags & SWP_NOSIZE == 0 {
            w.bounds[2] = width as i32;
            w.bounds[3] = height as i32;
        }
        if flags & SWP_SHOWWINDOW != 0 {
            w.style |= WS_VISIBLE;
        }
        if flags & SWP_HIDEWINDOW != 0 {
            w.style &= !WS_VISIBLE;
        }
        call.set_return(emu, 1);
    }
    call.cleanup_args(emu, 7);
}
