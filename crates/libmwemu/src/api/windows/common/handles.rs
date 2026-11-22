use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi, WindowsAbi::WinApi};
use crate::emu::Emu;
use crate::winapi::common::memory::{
    read_array, read_guest_bytes, writable_pointer, write_pointer,
};
use crate::winapi::common::{ipc, nt_api, process, sections, security};
use crate::winapi::environment_store::Encoding;
use crate::winapi::policy::{self, api_try};
use crate::windows::constants::errors::*;
use crate::windows::constants::files::*;
use crate::windows::constants::nls::CP_IBM437;
use crate::windows::constants::process::*;
use crate::windows::constants::security::GENERIC_ALL;

const DEFAULT_INPUT_MODE: u32 = 0x7; // Processed, line and echo input.
const DEFAULT_OUTPUT_MODE: u32 = 0x3; // Processed output and wrapping.
const INPUT_MODE_MASK: u32 = 0x3ff;
const OUTPUT_MODE_MASK: u32 = 0x1f;

pub fn GetConsoleMode(emu: &mut Emu) {
    console_mode(emu, false);
}

pub fn SetConsoleMode(emu: &mut Emu) {
    console_mode(emu, true);
}

fn console_mode(emu: &mut Emu, set: bool) {
    let call = WinApi(Stdcall);
    let [handle, argument] = api_try!(emu, call.args(emu));
    let result = (|| {
        let (selector, mode) = emu
            .handle_management
            .console(handle)
            .ok_or(ERROR_INVALID_HANDLE)?;
        if set {
            let new_mode = argument as u32;
            let allowed = if selector == STD_INPUT_HANDLE {
                INPUT_MODE_MASK
            } else {
                OUTPUT_MODE_MASK
            };
            if new_mode & !allowed != 0 {
                return Err(ERROR_INVALID_PARAMETER);
            }
            emu.handle_management.set_console_mode(selector, new_mode);
        } else if !emu.maps.write_bytes(argument, &mode.to_le_bytes()) {
            return Err(ERROR_NOACCESS);
        }
        Ok(())
    })();
    log_red!(
        emu,
        "{} handle={handle:#x}",
        if set {
            "SetConsoleMode"
        } else {
            "GetConsoleMode"
        }
    );
    if let Err(error) = result {
        emu.set_last_error(error);
    }
    call.set_return(emu, u64::from(result.is_ok()));
    call.cleanup_args(emu, 2);
}

pub(crate) fn current_handle(emu: &mut Emu, api: &str, thread: bool) {
    let call = WinApi(Stdcall);
    let handle = if thread {
        NT_CURRENT_THREAD
    } else {
        NT_CURRENT_PROCESS
    };
    log_red!(emu, "{} => 0x{:x}", api, handle);
    call.set_return(emu, handle);
    call.cleanup_args(emu, 0);
}

pub(crate) fn standard_console(emu: &mut Emu, selector: u32) -> Option<u64> {
    let mode = match selector {
        STD_INPUT_HANDLE => DEFAULT_INPUT_MODE,
        STD_OUTPUT_HANDLE | STD_ERROR_HANDLE => DEFAULT_OUTPUT_MODE,
        _ => return None,
    };
    Some(emu.handle_management.console_handle(selector, mode))
}

pub(crate) fn standard_handle(emu: &mut Emu, api: &str) {
    let call = WinApi(Stdcall);
    let selector = api_try!(emu, call.arg_u32(emu, 0));
    let handle = standard_console(emu, selector).unwrap_or_else(|| {
        emu.set_last_error(ERROR_INVALID_HANDLE);
        INVALID_HANDLE_VALUE_64
    });
    log_red!(emu, "{} selector=0x{:x} => 0x{:x}", api, selector, handle);
    call.set_return(emu, handle);
    call.cleanup_args(emu, 1);
}

pub(crate) fn close_handle(emu: &mut Emu, api: &str) {
    let call = WinApi(Stdcall);
    let handle = api_try!(emu, call.arg(emu, 0));
    log_red!(emu, "{} handle=0x{:x}", api, handle);
    let pseudo = [NT_CURRENT_PROCESS, NT_CURRENT_THREAD]
        .into_iter()
        .any(|value| {
            handle
                == if emu.cfg.arch.is_64bits() {
                    value
                } else {
                    value as u32 as u64
                }
        });
    let closed = pseudo || close_object(emu, handle);
    if !closed {
        emu.set_last_error(ERROR_INVALID_HANDLE);
    }
    call.set_return(emu, u64::from(closed));
    call.cleanup_args(emu, 1);
}

pub(crate) fn dispatch(api: &str, library: &str, emu: &mut Emu) -> bool {
    use crate::winapi::libraries::kernel32;
    let shared: Option<fn(&mut Emu)> = match api {
        "GetConsoleMode" => Some(GetConsoleMode),
        "SetConsoleMode" => Some(SetConsoleMode),
        "GetACP" => Some(kernel32::system::GetACP),
        "GetOEMCP" => Some(kernel32::system::GetOEMCP),
        "GetLocalTime" => Some(kernel32::GetLocalTime),
        "GetSystemTime" => Some(kernel32::GetSystemTime),
        "GetSystemTimeAsFileTime" => Some(kernel32::GetSystemTimeAsFileTime),
        "GetSystemTimePreciseAsFileTime" => Some(kernel32::GetSystemTimePreciseAsFileTime),
        _ => None,
    };
    if let Some(handler) = shared {
        handler(emu);
        return true;
    }
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

pub(crate) fn close_object(emu: &mut Emu, handle: u64) -> bool {
    if let Some(name) = emu.section_handles.remove(&handle) {
        emu.last_closed_section_dll = Some(name);
        return true;
    }
    // Bootstrap console files may have both a file name and a console tag.
    let bootstrap_file = emu.file_handles.remove(&handle).is_some();
    let bootstrap_console = emu.console_handles.remove(&handle);
    bootstrap_file
        || bootstrap_console
        || emu.file_section_names.remove(&handle).is_some()
        || emu.known_dll_dir_handles.remove(&handle)
        || emu.symbolic_link_targets.remove(&handle).is_some()
        || ipc::close(emu, handle)
        || sections::close(emu, handle)
        || emu.handle_management.close_registry_key(handle)
        || emu.handle_management.close_handle(handle)
}

pub(crate) fn dispatch_nt(api: &str, emu: &mut Emu, call: WindowsAbi) -> bool {
    match nt_api::normalize(api).as_ref() {
        "NtClose" => native_close(emu, call),
        "NtDuplicateObject" => duplicate(emu, "ntdll", call),
        "NtOpenProcessToken" => security::native_open_process_token(emu, call),
        "NtOpenProcess" => process::native_open_process(emu, call),
        _ => return false,
    }
    true
}

fn native_close(emu: &mut Emu, call: WindowsAbi) {
    let handle = api_try!(emu, call.arg(emu, 0));
    log_red!(emu, "ntdll!NtClose handle={handle:#x}");
    let status = if close_object(emu, handle) {
        STATUS_SUCCESS
    } else {
        STATUS_INVALID_HANDLE
    };
    call.set_return(emu, status);
    call.cleanup_args(emu, 1);
}

#[derive(Clone, Copy, serde::Serialize, serde::Deserialize)]
pub(crate) struct ConsoleCursor {
    size: u32,
    visible: bool,
}

impl Default for ConsoleCursor {
    fn default() -> Self {
        Self {
            size: 25,
            visible: true,
        }
    }
}

pub(crate) fn cursor_info(emu: &mut Emu, set: bool) {
    let call = WinApi(Stdcall);
    let [handle, ptr] = api_try!(emu, call.args(emu));
    let result = (|| {
        if emu
            .handle_management
            .console(handle)
            .is_none_or(|(s, _)| s == STD_INPUT_HANDLE)
        {
            return Err(ERROR_INVALID_HANDLE);
        }
        if ptr == 0 {
            return Err(ERROR_NOACCESS);
        }
        if set {
            let bytes = read_array::<8>(emu, ptr).ok_or(ERROR_NOACCESS)?;
            let size = u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
            if !(1..=100).contains(&size) {
                return Err(ERROR_INVALID_PARAMETER);
            }
            emu.console_cursor = ConsoleCursor {
                size,
                visible: bytes[4..].iter().any(|&b| b != 0),
            };
        } else {
            let mut bytes = [0; 8];
            bytes[..4].copy_from_slice(&emu.console_cursor.size.to_le_bytes());
            bytes[4..].copy_from_slice(&u32::from(emu.console_cursor.visible).to_le_bytes());
            if !emu.maps.validate_write_range(ptr, 8) || !emu.maps.write_bytes(ptr, &bytes) {
                return Err(ERROR_NOACCESS);
            }
        }
        Ok(())
    })();
    let name = if set {
        "SetConsoleCursorInfo"
    } else {
        "GetConsoleCursorInfo"
    };
    log_red!(emu, "{name} handle={handle:#x}");
    if let Err(error) = result {
        emu.set_last_error(error);
    }
    call.set_return(emu, u64::from(result.is_ok()));
    call.cleanup_args(emu, 2);
}

pub(crate) fn input_event_count(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let [h, out] = api_try!(emu, call.args(emu));
    let result = if emu
        .handle_management
        .console(h)
        .is_none_or(|(s, _)| s != STD_INPUT_HANDLE)
    {
        Err(ERROR_INVALID_HANDLE)
    } else if out == 0 || !emu.maps.validate_write_range(out, 4) || !emu.maps.write_dword(out, 0) {
        Err(ERROR_NOACCESS)
    } else {
        Ok(())
    };
    log_red!(emu, "GetNumberOfConsoleInputEvents handle={h:#x}");
    if let Err(e) = result {
        emu.set_last_error(e);
    }
    call.set_return(emu, u64::from(result.is_ok()));
    call.cleanup_args(emu, 2);
}

pub(crate) fn write_console(emu: &mut Emu, library: &str, api: &str) {
    let call = WinApi(Stdcall);
    let handle = api_try!(emu, call.arg(emu, 0));
    let buffer = api_try!(emu, call.arg(emu, 1));
    let count = api_try!(emu, call.arg_u32(emu, 2));
    let written = api_try!(emu, call.arg(emu, 3));
    let reserved = api_try!(emu, call.arg(emu, 4));
    let wide = api.ends_with('W');
    log_red!(
        emu,
        "{library}!{api} console={handle:#x} buffer={buffer:#x} characters={count} written={written:#x} reserved={reserved:#x}"
    );
    let result = (|| {
        if reserved != 0 {
            return Err(ERROR_INVALID_PARAMETER);
        }
        let Some((selector, _)) = emu.handle_management.console(handle) else {
            return Err(ERROR_INVALID_HANDLE);
        };
        if selector == STD_INPUT_HANDLE {
            return Err(ERROR_INVALID_HANDLE);
        }
        if written != 0 && !emu.maps.validate_write_range(written, 4) {
            return Err(ERROR_NOACCESS);
        }
        let size = u64::from(count) * if wide { 2 } else { 1 };
        if size > emu.maps.max_alloc_size {
            return Err(ERROR_NOT_ENOUGH_MEMORY);
        }
        if count != 0 {
            let bytes = read_guest_bytes(emu, buffer, size as usize).ok_or(ERROR_NOACCESS)?;
            let units = if wide {
                bytes
                    .chunks_exact(2)
                    .map(|b| u16::from_le_bytes([b[0], b[1]]))
                    .collect::<Vec<_>>()
            } else {
                bytes
                    .into_iter()
                    .map(|b| Encoding::Oem.decode_byte(b))
                    .collect()
            };
            emu.emulated_stdout
                .extend(String::from_utf16_lossy(&units).bytes());
        }
        if written != 0 {
            emu.maps.write_dword(written, count);
        }
        Ok(())
    })();
    if let Err(error) = result {
        emu.set_last_error(error);
    }
    call.set_return(emu, u64::from(result.is_ok()));
    call.cleanup_args(emu, 5);
}

pub(crate) fn console_code_page(emu: &mut Emu, library: &str, api: &str) {
    let call = WinApi(Stdcall);
    log_red!(emu, "{library}!{api} => {CP_IBM437}");
    call.set_return(emu, u64::from(CP_IBM437));
}

pub(crate) fn duplicate(emu: &mut Emu, library: &str, call: WindowsAbi) {
    let native = library == "ntdll";
    let api = if native {
        "NtDuplicateObject"
    } else {
        "DuplicateHandle"
    };
    let [
        source_process,
        source,
        target_process,
        output,
        desired_access,
        inherit,
        options,
    ] = api_try!(emu, call.args(emu));
    log_red!(
        emu,
        "{library}!{api} source_process={source_process:#x} source_handle={source:#x} target_process={target_process:#x} target_handle={output:#x} desired_access={desired_access:#x} handle_attributes={inherit} options={options:#x}"
    );
    let options = options as u32;
    if options & DUPLICATE_CLOSE_SOURCE != 0 || inherit as u32 != 0 || output == 0 {
        policy::unsupported_with_abi(emu, library, api, emu.pc(), call);
        return;
    }
    let mut error = STATUS_INVALID_HANDLE;
    let valid_output = writable_pointer(emu, output);
    let result = if !valid_output {
        error = STATUS_ACCESS_VIOLATION;
        None
    } else if options & !DUPLICATE_SAME_ACCESS != 0 {
        error = STATUS_INVALID_PARAMETER;
        None
    } else if let Err(status) =
        process::require_memory_access(emu, source_process, PROCESS_DUP_HANDLE)
            .and_then(|()| process::require_memory_access(emu, target_process, PROCESS_DUP_HANDLE))
    {
        error = status;
        None
    } else if let Some(id) = emu.handle_management.waitable_id(source) {
        if options & DUPLICATE_SAME_ACCESS == 0 {
            policy::unsupported_with_abi(emu, library, api, emu.pc(), call);
            return;
        }
        Some(emu.handle_management.open_waitable(id))
    } else if let Some(pid) = emu.handle_management.process_handle_id(source) {
        let rights = emu
            .handle_management
            .process_access(source)
            .unwrap_or_default();
        let desired = desired_access as u32;
        if options & DUPLICATE_SAME_ACCESS == 0 && desired & !rights != 0 {
            policy::unsupported_with_abi(emu, library, api, emu.pc(), call);
            return;
        }
        Some(u64::from(emu.handle_management.insert_process_handle(
            pid,
            if options & DUPLICATE_SAME_ACCESS != 0 {
                rights
            } else {
                desired
            },
        )))
    } else if source
        == if emu.cfg.arch.is_64bits() {
            NT_CURRENT_PROCESS
        } else {
            u64::from(NT_CURRENT_PROCESS as u32)
        }
    {
        let rights = if options & DUPLICATE_SAME_ACCESS != 0 {
            GENERIC_ALL
        } else {
            desired_access as u32
        };
        Some(u64::from(emu.handle_management.insert_process_handle(
            process::current_process_id(emu) as u32,
            rights,
        )))
    } else if source
        == if emu.cfg.arch.is_64bits() {
            NT_CURRENT_THREAD
        } else {
            u64::from(NT_CURRENT_THREAD as u32)
        }
        || emu.handle_management.contains_handle(source)
        || emu.file_handles.contains_key(&source)
        || emu.section_handles.contains_key(&source)
        || emu.file_section_names.contains_key(&source)
        || emu.known_dll_dir_handles.contains(&source)
        || emu.symbolic_link_targets.contains_key(&source)
        || emu.console_handles.contains(&source)
    {
        policy::unsupported_with_abi(emu, library, api, emu.pc(), call);
        return;
    } else {
        None
    };
    if valid_output {
        write_pointer(emu, output, result.unwrap_or_default());
    }
    if native {
        call.set_return(
            emu,
            if result.is_some() {
                STATUS_SUCCESS
            } else {
                error
            },
        );
    } else {
        if result.is_none() {
            emu.set_last_error(match error {
                STATUS_ACCESS_VIOLATION => ERROR_NOACCESS,
                STATUS_INVALID_PARAMETER => ERROR_INVALID_PARAMETER,
                STATUS_ACCESS_DENIED => ERROR_ACCESS_DENIED,
                _ => ERROR_INVALID_HANDLE,
            });
        }
        call.set_return(emu, u64::from(result.is_some()));
    }
    call.cleanup_args(emu, 7);
}

pub(crate) fn set_handle_count(emu: &mut Emu, library: &str) {
    let call = WinApi(Stdcall);
    let requested = api_try!(emu, call.arg_u32(emu, 0));
    log_red!(emu, "{library}!SetHandleCount count={requested}");
    call.set_return(emu, u64::from(requested));
    call.cleanup_args(emu, 1);
}
