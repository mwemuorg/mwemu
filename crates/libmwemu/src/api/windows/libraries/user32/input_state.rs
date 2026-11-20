use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu::Emu;
use crate::winapi::common::memory::read_array;
use crate::winapi::policy::api_try;
use crate::windows::constants::errors::ERROR_NOACCESS;

const KEYBOARD_BYTES: usize = 256;
const DESKTOP_CENTER: [i32; 2] = [960, 540];

pub fn GetKeyboardState(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let buffer = api_try!(emu, call.arg(emu, 0));
    let thread = emu.current_thread().id;
    log_red!(emu, "user32!GetKeyboardState key_state={buffer:#x}");
    let ok = buffer != 0
        && emu.maps.validate_write_range(buffer, KEYBOARD_BYTES as u64)
        && match emu.keyboard_state.get(&thread) {
            Some(state) => state.len() == KEYBOARD_BYTES && emu.maps.write_bytes(buffer, state),
            None => emu.maps.write_bytes(buffer, &[0; KEYBOARD_BYTES]),
        };
    if !ok {
        emu.set_last_error(ERROR_NOACCESS);
    }
    call.set_return(emu, ok.into());
    call.cleanup_args(emu, 1);
}

pub fn SetKeyboardState(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let buffer = api_try!(emu, call.arg(emu, 0));
    let thread = emu.current_thread().id;
    log_red!(emu, "user32!SetKeyboardState key_state={buffer:#x}");
    let state = read_array::<KEYBOARD_BYTES>(emu, buffer).filter(|_| buffer != 0);
    let ok = state.is_some();
    if let Some(bytes) = state {
        emu.keyboard_state.insert(thread, bytes.to_vec());
    } else {
        emu.set_last_error(ERROR_NOACCESS);
    }
    call.set_return(emu, ok.into());
    call.cleanup_args(emu, 1);
}

pub fn GetKeyState(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let key = api_try!(emu, call.arg_u32(emu, 0));
    log_red!(emu, "user32!GetKeyState virtual_key={key:#x}");
    let byte = emu
        .keyboard_state
        .get(&emu.current_thread().id)
        .and_then(|state| state.get(key as usize))
        .copied()
        .unwrap_or(0);
    let result = (u16::from(byte & 0x80) << 8) | u16::from(byte & 1);
    call.set_return(emu, result.into());
    call.cleanup_args(emu, 1);
}

fn cursor(emu: &mut Emu, api: &str) {
    let call = WinApi(Stdcall);
    let point = api_try!(emu, call.arg(emu, 0));
    log_red!(emu, "user32!{api} point={point:#x}");
    let [x, y] = emu
        .guest_windows
        .user
        .cursor_position
        .unwrap_or(DESKTOP_CENTER);
    let ok = point != 0
        && emu
            .maps
            .validate_write_range(point, size_of::<[i32; 2]>() as u64)
        && emu.maps.write_dword(point, x as u32)
        && emu
            .maps
            .write_dword(point + size_of::<i32>() as u64, y as u32);
    if !ok {
        emu.set_last_error(ERROR_NOACCESS);
    }
    call.set_return(emu, ok.into());
    call.cleanup_args(emu, 1);
}
pub fn GetCursorPos(emu: &mut Emu) {
    cursor(emu, "GetCursorPos");
}
pub fn GetPhysicalCursorPos(emu: &mut Emu) {
    cursor(emu, "GetPhysicalCursorPos");
}
pub fn IsGUIThread(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let convert = api_try!(emu, call.arg_i32(emu, 0)) != 0;
    let thread = emu.current_thread().id;
    if convert {
        emu.guest_windows.user.gui_threads.insert(thread);
    }
    let result = emu.guest_windows.user.gui_threads.contains(&thread);
    log_red!(
        emu,
        "user32!IsGUIThread thread={thread} convert={convert} is_gui={result}"
    );
    call.set_return(emu, result.into());
    call.cleanup_args(emu, 1);
}

pub fn IsProcessDPIAware(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    log_red!(emu, "user32!IsProcessDPIAware");
    call.set_return(emu, emu.guest_windows.user.dpi_aware.into());
}

pub fn SetProcessDPIAware(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    emu.guest_windows.user.dpi_aware = true;
    log_red!(emu, "user32!SetProcessDPIAware");
    call.set_return(emu, 1);
}
