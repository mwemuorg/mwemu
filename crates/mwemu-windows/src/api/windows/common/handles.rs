use crate::emu::Emu;
use crate::winapi::common::memory::Call;
use crate::winapi::common::sections;
use crate::winapi::helper;
use crate::windows::constants::errors::*;
use crate::windows::constants::files::*;
use crate::windows::constants::process::*;

pub fn current_handle(emu: &mut Emu, api: &str, thread: bool) {
    let handle = if thread {
        NT_CURRENT_THREAD
    } else {
        NT_CURRENT_PROCESS
    };
    log_red!(emu, "{} => 0x{:x}", api, handle);
    Call::Api.finish(emu, handle, 0);
}

pub fn standard_handle(emu: &mut Emu, api: &str) {
    let call = Call::Api;
    let selector = call.arg(emu, 0) as u32;
    let uri = match selector {
        STD_INPUT_HANDLE => Some("stdio://stdin"),
        STD_OUTPUT_HANDLE => Some("stdio://stdout"),
        STD_ERROR_HANDLE => Some("stdio://stderr"),
        _ => None,
    };
    let handle = if let Some(uri) = uri {
        helper::handler_find_by_uri(uri).unwrap_or_else(|| helper::handler_create(uri))
    } else {
        emu.set_last_error(ERROR_INVALID_HANDLE);
        INVALID_HANDLE_VALUE_64
    };
    log_red!(emu, "{} selector:0x{:x} => 0x{:x}", api, selector, handle);
    call.finish(emu, handle, 1);
}

pub fn close_handle(emu: &mut Emu, api: &str) {
    let call = Call::Api;
    let handle = call.arg(emu, 0);
    log_red!(emu, "{} handle:0x{:x}", api, handle);
    if !sections::close(emu, handle) && !emu.handle_management.close_handle(handle) {
        helper::handler_close(handle);
    }
    call.finish(emu, 1, 1);
}

pub fn dispatch(api: &str, library: &str, emu: &mut Emu) -> bool {
    let handler: fn(&mut Emu, &str) = match api {
        "GetCurrentProcess" => |emu, name| current_handle(emu, name, false),
        "GetCurrentThread" => |emu, name| current_handle(emu, name, true),
        "GetStdHandle" => standard_handle,
        "CloseHandle" => close_handle,
        _ => return false,
    };
    handler(emu, &format!("{library}!{api}"));
    true
}
