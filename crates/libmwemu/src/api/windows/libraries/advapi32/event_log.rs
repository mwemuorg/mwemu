use crate::api::abi::windows::{
    CallingConvention::{Cdecl, Stdcall},
    WindowsAbi::WinApi,
};
use crate::emu::Emu;
use crate::winapi::common::trace::input_string;
use crate::winapi::environment_store::Encoding;
use crate::winapi::policy::{self, api_try};
use crate::windows::constants::errors::*;
use crate::windows::constants::process::INVALID_HANDLE_VALUE_64;

const EVENTLOG_FULL_INFO: u32 = 0;
const FULL_INFORMATION_BYTES: u32 = size_of::<u32>() as u32;
const WNODE_HISTORICAL_CONTEXT: u64 = 8;

fn open_log(emu: &mut Emu, api: &str, wide: bool, source: bool) {
    let call = WinApi(Stdcall);
    let [server, name] = api_try!(emu, call.args(emu));
    let enc = Encoding::from_wide(wide);
    log_red!(
        emu,
        "advapi32!{api} server={} source={}",
        input_string(emu, server, enc),
        input_string(emu, name, enc)
    );
    if server != 0 && !matches!(enc.read(emu,server), Ok(value) if value.is_empty()) {
        policy::unsupported(emu, "advapi32", api, emu.pc());
        return;
    }
    let result = enc.read(emu, name).and_then(|value| {
        if value.is_empty() {
            return Err(ERROR_INVALID_PARAMETER);
        }
        Ok(emu.handle_management.open_event_log(source))
    });
    let handle = result.unwrap_or_else(|e| {
        emu.set_last_error(e);
        0
    });
    call.set_return(emu, handle);
    call.cleanup_args(emu, 2);
}

fn close_log(emu: &mut Emu, api: &str, source: bool) {
    let call = WinApi(Stdcall);
    let handle = api_try!(emu, call.arg(emu, 0));
    log_red!(emu, "advapi32!{api} event_log={handle:#x}");
    let ok = emu.handle_management.close_event_log(handle, source);
    if !ok {
        emu.set_last_error(ERROR_INVALID_HANDLE);
    }
    call.set_return(emu, ok.into());
    call.cleanup_args(emu, 1);
}

pub fn GetEventLogInformation(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let [event_log, level, buffer, size, needed] = api_try!(emu, call.args(emu));
    log_red!(
        emu,
        "advapi32!GetEventLogInformation event_log={event_log:#x} info_level={level} buffer={buffer:#x} buffer_size={size} bytes_needed={needed:#x}"
    );
    let error = if level as u32 != EVENTLOG_FULL_INFO {
        Some(ERROR_INVALID_LEVEL)
    } else if needed == 0
        || !emu
            .maps
            .validate_write_range(needed, FULL_INFORMATION_BYTES.into())
        || !emu.maps.write_dword(needed, FULL_INFORMATION_BYTES)
    {
        Some(ERROR_NOACCESS)
    } else if !emu.handle_management.is_event_log(event_log) {
        Some(ERROR_INVALID_HANDLE)
    } else if (size as u32) < FULL_INFORMATION_BYTES {
        Some(ERROR_INSUFFICIENT_BUFFER)
    } else if buffer == 0
        || !emu
            .maps
            .validate_write_range(buffer, FULL_INFORMATION_BYTES.into())
        || !emu.maps.write_dword(buffer, 0)
    {
        Some(ERROR_NOACCESS)
    } else {
        None
    };
    if let Some(e) = error {
        emu.set_last_error(e);
    }
    call.set_return(emu, error.is_none().into());
    call.cleanup_args(emu, 5);
}

pub fn GetTraceLoggerHandle(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let buffer = api_try!(emu, call.arg(emu, 0));
    log_red!(emu, "advapi32!GetTraceLoggerHandle buffer={buffer:#x}");
    let handle = (buffer != 0)
        .then_some(buffer)
        .and_then(|address| address.checked_add(WNODE_HISTORICAL_CONTEXT))
        .and_then(|context| emu.maps.read_qword(context));
    let handle = handle.unwrap_or_else(|| {
        emu.set_last_error(ERROR_INVALID_PARAMETER);
        INVALID_HANDLE_VALUE_64
    });
    call.set_return_u64(emu, handle);
    call.cleanup_args(emu, 1);
}

pub fn TraceMessage(emu: &mut Emu) {
    policy::unsupported_with_abi(emu, "advapi32", "TraceMessage", emu.pc(), WinApi(Cdecl));
}

pub fn OpenEventLogA(emu: &mut Emu) {
    open_log(emu, "OpenEventLogA", false, false);
}

pub fn OpenEventLogW(emu: &mut Emu) {
    open_log(emu, "OpenEventLogW", true, false);
}

pub fn RegisterEventSourceA(emu: &mut Emu) {
    open_log(emu, "RegisterEventSourceA", false, true);
}

pub fn RegisterEventSourceW(emu: &mut Emu) {
    open_log(emu, "RegisterEventSourceW", true, true);
}

pub fn DeregisterEventSource(emu: &mut Emu) {
    close_log(emu, "DeregisterEventSource", true);
}

pub fn CloseEventLog(emu: &mut Emu) {
    close_log(emu, "CloseEventLog", false);
}

pub fn EventProviderEnabled(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let provider = api_try!(emu, call.arg_u64(emu, 0));
    let level_slot = if emu.cfg.arch.is_64bits() { 1 } else { 2 };
    let level = api_try!(emu, call.arg_u8(emu, level_slot));
    let keyword = api_try!(emu, call.arg_u64(emu, level_slot + 1));
    log_red!(
        emu,
        "advapi32!EventProviderEnabled provider={provider:#x} level={level} keyword={keyword:#x}"
    );
    // The guest has no active ETW sessions.
    call.set_return(emu, 0);
    call.cleanup_args(emu, 5);
}

pub fn GetNumberOfEventLogRecords(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let [event_log, count] = api_try!(emu, call.args(emu));
    log_red!(
        emu,
        "advapi32!GetNumberOfEventLogRecords event_log={event_log:#x} count={count:#x}"
    );
    let error = if !emu.handle_management.is_event_log(event_log) {
        Some(ERROR_INVALID_HANDLE)
    } else if count == 0
        || !emu.maps.validate_write_range(count, 4)
        || !emu.maps.write_dword(count, 0)
    {
        Some(ERROR_NOACCESS)
    } else {
        None
    };
    if let Some(error) = error {
        emu.set_last_error(error);
    }
    call.set_return(emu, u64::from(error.is_none()));
    call.cleanup_args(emu, 2);
}
