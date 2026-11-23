use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu::Emu;
use crate::winapi::common::memory::{pointer_size, read_pointer};
use crate::winapi::policy::{self, api_try};
use crate::winapi::window_table::UserTimer;
use crate::windows::constants::controls::{USER_TIMER_MAXIMUM, USER_TIMER_MINIMUM, WM_TIMER};
use crate::windows::constants::errors::{
    ERROR_INVALID_WINDOW_HANDLE, ERROR_NOACCESS, ERROR_NOT_ENOUGH_MEMORY,
};

fn timer_thread(emu: &Emu, hwnd: u64) -> Option<u64> {
    if hwnd == 0 {
        Some(emu.current_thread().id)
    } else {
        emu.guest_windows.get(hwnd).map(|window| window.thread)
    }
}

pub fn SetTimer(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let [hwnd, requested, millis, callback] = api_try!(emu, call.args(emu));
    let period = (millis as u32).clamp(USER_TIMER_MINIMUM, USER_TIMER_MAXIMUM);
    let result = timer_thread(emu, hwnd)
        .ok_or(ERROR_INVALID_WINDOW_HANDLE)
        .and_then(|thread| {
            let timer = UserTimer {
                thread,
                deadline: (emu.tick as u64).saturating_add(u64::from(period)),
                period,
                callback,
            };
            emu.user_timers
                .set(hwnd, requested, timer)
                .ok_or(ERROR_NOT_ENOUGH_MEMORY)
        });
    let id = result.unwrap_or_else(|error| {
        emu.set_last_error(error);
        0
    });
    log_red!(
        emu,
        "user32!SetTimer hwnd={hwnd:#x} timer_id={requested:#x} interval={period} callback={callback:#x} => {id:#x}"
    );
    call.set_return(emu, id);
    call.cleanup_args(emu, 4);
}

pub fn KillTimer(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let [hwnd, id] = api_try!(emu, call.args(emu));
    let killed = if hwnd == 0 || emu.guest_windows.contains(hwnd) {
        emu.user_timers.kill(hwnd, id)
    } else {
        emu.set_last_error(ERROR_INVALID_WINDOW_HANDLE);
        false
    };
    log_red!(
        emu,
        "user32!KillTimer hwnd={hwnd:#x} timer_id={id:#x} => {killed}"
    );
    call.set_return(emu, u64::from(killed));
    call.cleanup_args(emu, 2);
}

fn dispatch(emu: &mut Emu, api: &str) {
    let call = WinApi(Stdcall);
    let message = api_try!(emu, call.arg(emu, 0));
    let word = pointer_size(emu);
    let fields = (|| {
        message.checked_add(4 * word)?;
        Some((
            read_pointer(emu, message)?,
            emu.maps.read_dword(message + word)?,
            read_pointer(emu, message + 2 * word)?,
            read_pointer(emu, message + 3 * word)?,
        ))
    })();
    let Some((hwnd, id, wparam, lparam)) = fields else {
        emu.set_last_error(ERROR_NOACCESS);
        call.set_return(emu, 0);
        call.cleanup_args(emu, 1);
        return;
    };
    log_red!(
        emu,
        "user32!{api} hwnd={hwnd:#x} message={id:#x} wparam={wparam:#x} lparam={lparam:#x}"
    );
    let mut result = 0;
    if id == WM_TIMER && lparam != 0 {
        // Windows checks the process's live callback addresses, even for an already queued message.
        if emu.user_timers.has_callback(lparam) {
            let args = [
                hwnd,
                u64::from(WM_TIMER),
                wparam,
                u64::from(emu.tick as u32),
            ];
            let Some(value) = policy::invoke_callback(emu, "user32", api, lparam, args) else {
                return;
            };
            result = value;
        }
    } else if hwnd != 0 {
        let Some(window) = emu.guest_windows.get(hwnd) else {
            emu.set_last_error(ERROR_INVALID_WINDOW_HANDLE);
            call.set_return(emu, 0);
            call.cleanup_args(emu, 1);
            return;
        };
        if id != WM_TIMER || emu.guest_windows.class_wide(&window.class).is_some() {
            policy::unsupported(emu, "user32", api, emu.pc());
            return;
        }
    }
    call.set_return(emu, result);
    call.cleanup_args(emu, 1);
}

pub fn DispatchMessageA(emu: &mut Emu) {
    dispatch(emu, "DispatchMessageA");
}

pub fn DispatchMessageW(emu: &mut Emu) {
    dispatch(emu, "DispatchMessageW");
}
