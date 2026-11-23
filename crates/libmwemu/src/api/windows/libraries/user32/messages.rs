use super::window_text::{copy_text, window_text};
use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu::Emu;
use crate::winapi::common::memory::pointer_size;
use crate::winapi::environment_store::Encoding;
use crate::winapi::policy::{self, api_try};
use crate::winapi::window_table::{PostedMessage, WindowTable};
use crate::windows::constants::controls::{
    PM_REMOVE, QS_TIMER, USER_TIMER_MINIMUM, WM_KEYFIRST, WM_KEYLAST, WM_QUIT, WM_TIMER,
};
use crate::windows::constants::errors::{
    ERROR_INVALID_PARAMETER, ERROR_INVALID_WINDOW_HANDLE, ERROR_NOACCESS, ERROR_NOT_ENOUGH_QUOTA,
};

const MAX_POSTED_MESSAGES: usize = 10_000;

pub(crate) fn registered_message(emu: &mut Emu, name: &[u16]) -> u64 {
    match emu.atom_tables.register_user_name(name) {
        Ok(id) => id as u64,
        Err(error) => {
            emu.set_last_error(error);
            0
        }
    }
}

fn register_message(emu: &mut Emu, wide: bool) {
    let call = WinApi(Stdcall);
    let address = api_try!(emu, call.arg(emu, 0));
    let text = if address == 0 {
        Err(ERROR_INVALID_PARAMETER)
    } else {
        Encoding::from_wide(wide).read(emu, address)
    };
    let result = match text {
        Ok(name) => {
            let id = registered_message(emu, &name);
            log_red!(
                emu,
                "user32!RegisterWindowMessage{} name={:?} id={id:#x}",
                if wide { "W" } else { "A" },
                String::from_utf16_lossy(&name)
            );
            id
        }
        Err(error) => {
            emu.set_last_error(error);
            0
        }
    };
    call.set_return(emu, result);
    call.cleanup_args(emu, 1);
}

pub fn RegisterWindowMessageA(emu: &mut Emu) {
    register_message(emu, false);
}

pub fn RegisterWindowMessageW(emu: &mut Emu) {
    register_message(emu, true);
}

fn enqueue(emu: &mut Emu, thread: u64, hwnd: u64, id: u32, wparam: u64, lparam: u64) -> bool {
    let input = &mut emu.guest_windows.user;
    let queue = input.messages.entry(thread).or_default();
    if queue.len() >= MAX_POSTED_MESSAGES {
        return false;
    }
    queue.push(PostedMessage {
        hwnd,
        id,
        wparam,
        lparam,
        time: emu.tick as u32,
        point: input.cursor_position.unwrap_or([960, 540]),
    });
    true
}

fn post(emu: &mut Emu, api: &str) {
    let call = WinApi(Stdcall);
    let [hwnd, id, wparam, lparam] = api_try!(emu, call.args(emu));
    log_red!(
        emu,
        "user32!{api} hwnd={hwnd:#x} message={id:#x} wparam={wparam:#x} lparam={lparam:#x}"
    );
    let thread = if hwnd == 0 {
        Some(emu.current_thread().id)
    } else {
        emu.guest_windows.get(hwnd).map(|w| w.thread)
    };
    let ok = if let Some(thread) = thread {
        if enqueue(emu, thread, hwnd, id as u32, wparam, lparam) {
            true
        } else {
            emu.set_last_error(ERROR_NOT_ENOUGH_QUOTA);
            false
        }
    } else {
        emu.set_last_error(ERROR_INVALID_WINDOW_HANDLE);
        false
    };
    call.set_return(emu, u64::from(ok));
    call.cleanup_args(emu, 4);
}

fn write_message(emu: &mut Emu, at: u64, m: &PostedMessage) -> bool {
    let wide = emu.cfg.arch.is_64bits();
    let mut bytes = Vec::with_capacity(if wide { 48 } else { 32 });
    if wide {
        bytes.extend(m.hwnd.to_le_bytes());
        bytes.extend(m.id.to_le_bytes());
        bytes.extend([0; 4]);
        bytes.extend(m.wparam.to_le_bytes());
        bytes.extend(m.lparam.to_le_bytes());
    } else {
        bytes.extend((m.hwnd as u32).to_le_bytes());
        bytes.extend(m.id.to_le_bytes());
        bytes.extend((m.wparam as u32).to_le_bytes());
        bytes.extend((m.lparam as u32).to_le_bytes());
    }
    bytes.extend(m.time.to_le_bytes());
    for v in m.point {
        bytes.extend(v.to_le_bytes());
    }
    bytes.extend([0; 4]);
    at != 0
        && emu.maps.validate_write_range(at, bytes.len() as u64)
        && emu.maps.write_bytes(at, &bytes)
}

struct MessageFilter {
    window: u64,
    thread_only: bool,
    minimum: u64,
    maximum: u64,
    include_timers: bool,
}

impl MessageFilter {
    fn accepts_window(&self, windows: &WindowTable, hwnd: u64) -> bool {
        self.window == 0
            || (self.thread_only && hwnd == 0)
            || (!self.thread_only && (hwnd == self.window || windows.is_child(self.window, hwnd)))
    }

    fn accepts_id(&self, id: u32) -> bool {
        if id == WM_TIMER && !self.include_timers {
            return false;
        }
        id == WM_QUIT
            || (self.minimum == 0 && self.maximum == 0)
            || (self.minimum..=self.maximum).contains(&u64::from(id))
    }

    fn queued(&self, emu: &Emu, thread: u64) -> Option<(usize, PostedMessage)> {
        let queue = emu.guest_windows.user.messages.get(&thread)?;
        let index = queue.iter().position(|m| {
            self.accepts_id(m.id) && self.accepts_window(&emu.guest_windows, m.hwnd)
        })?;
        Some((index, queue[index].clone()))
    }
}

fn materialize_timer(
    emu: &mut Emu,
    thread: u64,
    filter: &MessageFilter,
    wait: bool,
) -> Option<(usize, PostedMessage)> {
    if !filter.accepts_id(WM_TIMER) {
        return None;
    }
    let windows = &emu.guest_windows;
    let ((hwnd, id), timer) = emu
        .user_timers
        .next(thread, |hwnd| filter.accepts_window(windows, hwnd))?;
    let now = emu.tick as u64;
    if timer.deadline > now {
        let deadline = usize::try_from(timer.deadline).ok()?;
        if !wait
            || (emu.cfg.enable_threading
                && emu
                    .threads
                    .iter()
                    .any(|other| other.id != thread && other.is_runnable(deadline)))
        {
            return None;
        }
        emu.tick = deadline;
    }
    let now = emu.tick as u64;
    let period = u64::from(timer.period.max(USER_TIMER_MINIMUM));
    let elapsed = now.saturating_sub(timer.deadline);
    timer.deadline = now.saturating_add(period - elapsed % period);
    let m = PostedMessage {
        hwnd,
        id: WM_TIMER,
        wparam: id,
        lparam: timer.callback,
        time: emu.tick as u32,
        point: windows.user.cursor_position.unwrap_or([960, 540]),
    };
    let queue = emu.guest_windows.user.messages.entry(thread).or_default();
    let index = queue.len();
    queue.push(m.clone());
    Some((index, m))
}

fn receive(emu: &mut Emu, api: &str, peek: bool) {
    let call = WinApi(Stdcall);
    let [output, window, minimum, maximum] = api_try!(emu, call.args(emu));
    let flags = if peek {
        api_try!(emu, call.arg_u32(emu, 4))
    } else {
        PM_REMOVE
    };
    let thread = emu.current_thread().id;
    let thread_only = window
        == if emu.cfg.arch.is_64bits() {
            u64::MAX
        } else {
            u32::MAX.into()
        };
    log_red!(
        emu,
        "user32!{api} output={output:#x} hwnd={window:#x} minimum={minimum:#x} maximum={maximum:#x} flags={flags:#x}"
    );
    let failure = if peek { 0 } else { u64::from(u32::MAX) };
    let msg_size = if emu.cfg.arch.is_64bits() { 48 } else { 32 };
    let result = if output == 0 || !emu.maps.validate_write_range(output, msg_size) {
        emu.set_last_error(ERROR_NOACCESS);
        failure
    } else if window != 0 && !thread_only && !emu.guest_windows.contains(window) {
        emu.set_last_error(ERROR_INVALID_WINDOW_HANDLE);
        failure
    } else {
        let filter = MessageFilter {
            window,
            thread_only,
            minimum,
            maximum,
            include_timers: flags >> 16 == 0 || ((flags >> 16) & QS_TIMER) != 0,
        };
        let next = filter
            .queued(emu, thread)
            .or_else(|| materialize_timer(emu, thread, &filter, !peek));
        if let Some((index, m)) = next {
            if !write_message(emu, output, &m) {
                emu.set_last_error(ERROR_NOACCESS);
                failure
            } else {
                if flags & PM_REMOVE != 0
                    && let Some(queue) = emu.guest_windows.user.messages.get_mut(&thread)
                {
                    queue.remove(index);
                }
                emu.guest_windows.user.message_times.insert(thread, m.time);
                u64::from(peek || m.id != WM_QUIT)
            }
        } else if peek {
            0
        } else {
            policy::unsupported(emu, "user32", api, emu.pc());
            return;
        }
    };
    call.set_return(emu, result);
    call.cleanup_args(emu, 4 + usize::from(peek));
}

pub fn PostMessageA(emu: &mut Emu) {
    post(emu, "PostMessageA");
}

pub fn PostMessageW(emu: &mut Emu) {
    post(emu, "PostMessageW");
}

pub fn GetMessageA(emu: &mut Emu) {
    receive(emu, "GetMessageA", false);
}

pub fn GetMessageW(emu: &mut Emu) {
    receive(emu, "GetMessageW", false);
}

pub fn PeekMessageA(emu: &mut Emu) {
    receive(emu, "PeekMessageA", true);
}

pub fn PeekMessageW(emu: &mut Emu) {
    receive(emu, "PeekMessageW", true);
}

pub fn GetMessageTime(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let thread = emu.current_thread().id;
    let time = emu
        .guest_windows
        .user
        .message_times
        .get(&thread)
        .copied()
        .unwrap_or(0);
    log_red!(emu, "user32!GetMessageTime time={time}");
    call.set_return(emu, time.into());
}

pub fn TranslateMessage(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let message = api_try!(emu, call.arg(emu, 0));
    let id = message
        .checked_add(pointer_size(emu))
        .and_then(|p| emu.maps.read_dword(p));
    if id.is_some_and(|id| (WM_KEYFIRST..=WM_KEYLAST).contains(&id)) {
        policy::unsupported(emu, "user32", "TranslateMessage", emu.pc());
        return;
    }
    log_red!(emu, "user32!TranslateMessage message={message:#x}");
    call.set_return(emu, 0);
    call.cleanup_args(emu, 1);
}

pub fn PostQuitMessage(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let exit_code = api_try!(emu, call.arg_i32(emu, 0));
    let thread = emu.current_thread().id;
    log_red!(emu, "user32!PostQuitMessage exit_code={exit_code}");
    enqueue(emu, thread, 0, WM_QUIT, u64::from(exit_code as u32), 0);
    call.cleanup_args(emu, 1);
}

const WM_NULL: u32 = 0;
const WM_SETTEXT: u32 = 0x000c;
const WM_GETTEXT: u32 = 0x000d;
const WM_GETTEXTLENGTH: u32 = 0x000e;

fn default_message(emu: &mut Emu, api: &str, wide: bool, default_proc: bool) {
    let call = WinApi(Stdcall);
    let [hwnd, id, wparam, lparam] = api_try!(emu, call.args(emu));
    let id = id as u32;
    log_red!(
        emu,
        "user32!{api} hwnd={hwnd:#x} message={id:#x} wparam={wparam:#x} lparam={lparam:#x}"
    );
    let encoding = Encoding::from_wide(wide);
    let result = if !emu.guest_windows.contains(hwnd) {
        Err(ERROR_INVALID_WINDOW_HANDLE)
    } else if !default_proc
        && emu
            .guest_windows
            .get(hwnd)
            .is_some_and(|w| emu.guest_windows.class_wide(&w.class).is_some())
    {
        policy::unsupported(emu, "user32", api, emu.pc());
        return;
    } else {
        match id {
            WM_NULL => Ok(0),
            WM_GETTEXTLENGTH => {
                window_text(emu, hwnd, encoding, false).map(|s| s.len() / encoding.width())
            }
            WM_GETTEXT => copy_text(emu, hwnd, encoding, false, lparam, wparam as i32),
            WM_SETTEXT => {
                let value = if lparam == 0 {
                    Ok(Vec::new())
                } else {
                    encoding.read(emu, lparam)
                };
                value.and_then(|s| {
                    emu.guest_windows
                        .get_mut(hwnd)
                        .map(|w| {
                            w.title = String::from_utf16_lossy(&s);
                            1
                        })
                        .ok_or(ERROR_INVALID_WINDOW_HANDLE)
                })
            }
            _ => {
                policy::unsupported(emu, "user32", api, emu.pc());
                return;
            }
        }
    };
    let value = result.unwrap_or_else(|e| {
        emu.set_last_error(e);
        0
    });
    call.set_return(emu, value as u64);
    call.cleanup_args(emu, 4);
}
pub fn SendMessageA(emu: &mut Emu) {
    default_message(emu, "SendMessageA", false, false);
}

pub fn SendMessageW(emu: &mut Emu) {
    default_message(emu, "SendMessageW", true, false);
}

pub fn DefWindowProcA(emu: &mut Emu) {
    default_message(emu, "DefWindowProcA", false, true);
}

pub fn DefWindowProcW(emu: &mut Emu) {
    default_message(emu, "DefWindowProcW", true, true);
}

pub fn WaitMessage(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let thread = emu.current_thread().id;
    if emu
        .guest_windows
        .user
        .messages
        .get(&thread)
        .is_none_or(Vec::is_empty)
        && materialize_timer(
            emu,
            thread,
            &MessageFilter {
                window: 0,
                thread_only: false,
                minimum: 0,
                maximum: 0,
                include_timers: true,
            },
            true,
        )
        .is_none()
    {
        policy::unsupported(emu, "user32", "WaitMessage", emu.pc());
        return;
    }
    log_red!(emu, "user32!WaitMessage thread={thread}");
    call.set_return(emu, 1);
}
