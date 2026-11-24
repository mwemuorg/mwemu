use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi, WindowsAbi::WinApi};
use crate::emu::Emu;
use crate::exception::types::{ExceptionRecord, ExceptionType, MemoryAccess};
use crate::exception::{handlers, raise};
use crate::winapi::common::memory::{
    pointer_size, read_guest_bytes, read_pointer, writable_pointer, write_pointer,
};
use crate::winapi::common::{handles, modules, nt_api, threads, trace};
use crate::winapi::environment_store::Encoding;
use crate::winapi::helper;
use crate::winapi::policy::{self, api_try};
use crate::windows::constants::errors::*;
use crate::windows::constants::exceptions::{
    EXCEPTION_CONTINUE_SEARCH, EXCEPTION_EXECUTE_HANDLER, EXCEPTION_MAXIMUM_PARAMETERS,
};
use crate::windows::constants::process::*;
use crate::windows::constants::security::{
    GENERIC_ALL, GENERIC_READ, GENERIC_WRITE, MAXIMUM_ALLOWED,
};
use crate::windows::kuser_shared::PROCESSOR_FEATURES;
use crate::windows::structures::{ACTIVE_PROCESSOR_MASK, PROCESSORS, TEB, TEB64, save_system_info};
use goblin::pe::dll_characteristic::IMAGE_DLLCHARACTERISTICS_NX_COMPAT;
use goblin::pe::header::COFF_MACHINE_UNKNOWN;
use rs_header::pe::{IMAGE_FILE_MACHINE_AMD64, IMAGE_FILE_MACHINE_I386};

const DEFAULT_PROCESS_ID: u64 = 0x123;
const PROCESS_COOKIE: u32 = 0x0123_4567;
const BASE_PRIORITY_NORMAL: u64 = 8;
const BASIC_INFORMATION_SLOTS: usize = 6;
const EXTENDED_INFORMATION_SLOTS: usize = 8;

pub(crate) fn current_process_id(emu: &Emu) -> u64 {
    let offset = if emu.cfg.arch.is_64bits() {
        TEB64::CLIENT_ID_PROCESS_ID_OFFSET
    } else {
        TEB::CLIENT_ID_PROCESS_ID_OFFSET
    };
    emu.windows_teb()
        .and_then(|base| base.checked_add(offset))
        .and_then(|address| read_pointer(emu, address))
        .filter(|pid| *pid != 0)
        .unwrap_or(DEFAULT_PROCESS_ID)
}

pub fn GetCurrentProcessId(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let pid = current_process_id(emu);
    log_red!(emu, "kernel32!GetCurrentProcessId => {}", pid);
    call.set_return(emu, pid);
}

pub(crate) fn query_current_process(emu: &Emu, handle: u64) -> bool {
    (handle == NT_CURRENT_PROCESS
        || !emu.cfg.arch.is_64bits() && handle == NT_CURRENT_PROCESS as u32 as u64)
        || emu
            .handle_management
            .process_handle_id(handle)
            .is_some_and(|id| u64::from(id) == current_process_id(emu))
        || helper::handler_get_uri(handle) == format!("pid://{}", current_process_id(emu))
}

pub(crate) fn require_memory_access(emu: &Emu, handle: u64, required: u32) -> Result<(), u64> {
    if !query_current_process(emu, handle) {
        return Err(STATUS_INVALID_HANDLE);
    }
    if let Some(mut access) = emu.handle_management.process_access(handle) {
        if access & (GENERIC_ALL | MAXIMUM_ALLOWED) != 0 {
            return Ok(());
        }
        if access & GENERIC_READ != 0 {
            access |= PROCESS_VM_READ | PROCESS_QUERY_INFORMATION;
        }
        if access & GENERIC_WRITE != 0 {
            access |= PROCESS_VM_OPERATION | PROCESS_VM_WRITE;
        }
        if access & required != required {
            return Err(STATUS_ACCESS_DENIED);
        }
    }
    Ok(())
}

pub(crate) fn query_information_process(emu: &mut Emu, call: WindowsAbi) {
    trace::invoke(
        emu,
        call,
        "ntdll!NtQueryInformationProcess",
        "ProcessHandle Class Buffer Length ReturnLength",
        true,
        |emu| {
            let handle = api_try!(emu, call.arg(emu, 0));
            let class = api_try!(emu, call.arg_u32(emu, 1)) as u64;
            let buffer = api_try!(emu, call.arg(emu, 2));
            let length = api_try!(emu, call.arg_u32(emu, 3));
            let returned = api_try!(emu, call.arg(emu, 4));
            let status = query_process(emu, handle, class, buffer, length, returned);
            call.cleanup_args(emu, 5);
            call.set_return(emu, status);
        },
    );
}

fn query_process(
    emu: &mut Emu,
    handle: u64,
    class: u64,
    buffer: u64,
    length: u32,
    returned: u64,
) -> u64 {
    let width = pointer_size(emu) as usize;
    if class == PROCESS_RAISE_UM_EXCEPTION_ON_INVALID_HANDLE_CLOSE {
        return invalid_close_information(emu, handle, buffer, length, Some(returned));
    }
    let basic_size = BASIC_INFORMATION_SLOTS * width;
    let extended_size = EXTENDED_INFORMATION_SLOTS * width;
    let size = match class {
        PROCESS_INFORMATION_CLASS_PROCESS_BASIC_INFORMATION if length as usize == extended_size => {
            extended_size
        }
        PROCESS_INFORMATION_CLASS_PROCESS_BASIC_INFORMATION => basic_size,
        PROCESS_INFORMATION_CLASS_PROCESS_DEBUG_PORT
        | PROCESS_INFORMATION_CLASS_PROCESS_WOW64_INFORMATION
        | PROCESS_INFORMATION_CLASS_PROCESS_DEBUG_OBJECT_HANDLE
        | PROCESS_CONSOLE_HOST_PROCESS => width,
        PROCESS_INFORMATION_CLASS_PROCESS_BREAK_ON_TERMINATION
        | PROCESS_INFORMATION_CLASS_PROCESS_DEBUG_FLAGS
        | PROCESS_INFORMATION_CLASS_PROCESS_COOKIE => size_of::<u32>(),
        _ => {
            log::warn!(
                "NtQueryInformationProcess: unsupported information class {}",
                class
            );
            return STATUS_NOT_IMPLEMENTED;
        }
    };
    if length as usize != size {
        return STATUS_INFO_LENGTH_MISMATCH;
    }
    if !query_current_process(emu, handle) {
        return STATUS_INVALID_HANDLE;
    }
    if buffer == 0
        || !emu.maps.validate_write_range(buffer, size as u64)
        || (returned != 0
            && !emu
                .maps
                .validate_write_range(returned, size_of::<u32>() as u64))
    {
        return STATUS_ACCESS_VIOLATION;
    }
    let mut output = vec![0u8; size];
    let mut status = STATUS_SUCCESS;
    match class {
        PROCESS_INFORMATION_CLASS_PROCESS_BASIC_INFORMATION => {
            let peb = emu
                .maps
                .get_map_by_name("peb")
                .map(|map| map.get_base())
                .unwrap_or(0);
            let fields = [
                STATUS_PENDING,
                peb,
                1,
                BASE_PRIORITY_NORMAL,
                current_process_id(emu),
                0,
            ];
            let offset = if size == extended_size {
                output[..width].copy_from_slice(&(size as u64).to_le_bytes()[..width]);
                width
            } else {
                0
            };
            for (slot, value) in output[offset..offset + basic_size]
                .chunks_exact_mut(width)
                .zip(fields)
            {
                slot.copy_from_slice(&value.to_le_bytes()[..width]);
            }
        }
        PROCESS_INFORMATION_CLASS_PROCESS_DEBUG_FLAGS => {
            output.copy_from_slice(&1u32.to_le_bytes())
        }
        PROCESS_INFORMATION_CLASS_PROCESS_COOKIE => {
            output.copy_from_slice(&PROCESS_COOKIE.to_le_bytes())
        }
        PROCESS_CONSOLE_HOST_PROCESS => {
            output.copy_from_slice(&emu.console_host_process.to_le_bytes()[..width])
        }
        PROCESS_INFORMATION_CLASS_PROCESS_DEBUG_OBJECT_HANDLE => status = STATUS_PORT_NOT_SET,
        _ => {}
    }
    if !emu.maps.write_bytes(buffer, &output)
        || (returned != 0 && !emu.maps.write_dword(returned, size as u32))
    {
        return STATUS_ACCESS_VIOLATION;
    }
    status
}

/// A query probes its optional ReturnLength pointer but does not update it for this class.
fn invalid_close_information(
    emu: &mut Emu,
    handle: u64,
    buffer: u64,
    length: u32,
    query_return_length: Option<u64>,
) -> u64 {
    if length != 0 && !buffer.is_multiple_of(align_of::<u32>() as u64) {
        return STATUS_DATATYPE_MISALIGNMENT;
    }
    if query_return_length
        .is_some_and(|out| out != 0 && !emu.maps.validate_write_range(out, size_of::<u32>() as u64))
    {
        return STATUS_ACCESS_VIOLATION;
    }
    if length as usize != size_of::<u32>() {
        return STATUS_INFO_LENGTH_MISMATCH;
    }
    let query = query_return_length.is_some();
    let value = if query {
        if !emu.maps.validate_write_range(buffer, length as u64) {
            return STATUS_ACCESS_VIOLATION;
        }
        0
    } else if let Some(value) = emu.maps.read_dword(buffer) {
        value
    } else {
        return STATUS_ACCESS_VIOLATION;
    };
    if !query_current_process(emu, handle) {
        return if handles::type_name(emu, handle).is_some()
            || emu.handle_management.contains_handle(handle)
        {
            STATUS_OBJECT_TYPE_MISMATCH
        } else {
            STATUS_INVALID_HANDLE
        };
    }
    let access = if query {
        PROCESS_QUERY_INFORMATION | PROCESS_QUERY_LIMITED_INFORMATION | GENERIC_READ
    } else {
        PROCESS_SET_INFORMATION | GENERIC_WRITE
    };
    if emu
        .handle_management
        .process_access(handle)
        .is_some_and(|rights| rights & (access | GENERIC_ALL | MAXIMUM_ALLOWED) == 0)
    {
        return STATUS_ACCESS_DENIED;
    }
    if query {
        emu.maps
            .write_dword(buffer, u32::from(emu.raise_on_invalid_handle_close));
    } else if value <= 1 {
        emu.raise_on_invalid_handle_close = value != 0;
    }
    STATUS_SUCCESS
}

pub(crate) fn set_invalid_close_policy(emu: &mut Emu, call: WindowsAbi) {
    let [handle, _, buffer, length] = api_try!(emu, call.args(emu));
    let status = invalid_close_information(emu, handle, buffer, length as u32, None);
    log_red!(
        emu,
        "ntdll!NtSetInformationProcess ProcessRaiseUMExceptionOnInvalidHandleClose process={handle:#x} => {status:#x}"
    );
    call.set_return(emu, status);
    call.cleanup_args(emu, 4);
}

pub(crate) fn set_console_host_process(emu: &mut Emu, call: WindowsAbi) {
    let [handle, _, buffer, length] = api_try!(emu, call.args(emu));
    let status = if length as u32 != pointer_size(emu) as u32 {
        STATUS_INFO_LENGTH_MISMATCH
    } else if let Err(status) = require_memory_access(emu, handle, PROCESS_SET_INFORMATION) {
        status
    } else if let Some(value) = read_pointer(emu, buffer) {
        emu.console_host_process = value;
        STATUS_SUCCESS
    } else {
        STATUS_ACCESS_VIOLATION
    };
    call.set_return(emu, status);
    call.cleanup_args(emu, 4);
}

pub(crate) fn dispatch_nt(api: &str, emu: &mut Emu, call: WindowsAbi) -> bool {
    match nt_api::normalize(api).as_ref() {
        "NtQueryInformationProcess" => query_information_process(emu, call),
        "NtSetInformationProcess"
            if call.arg_u32(emu, 1) == Ok(PROCESS_CONSOLE_HOST_PROCESS as u32) =>
        {
            set_console_host_process(emu, call)
        }
        "NtSetInformationProcess"
            if call.arg_u32(emu, 1)
                == Ok(PROCESS_RAISE_UM_EXCEPTION_ON_INVALID_HANDLE_CLOSE as u32) =>
        {
            set_invalid_close_policy(emu, call)
        }
        "NtQueryInformationThread" => query_information_thread(emu, call),
        _ => return false,
    }
    true
}

pub fn GetErrorMode(emu: &mut Emu) {
    let mode = emu.process_error_mode;
    WinApi(Stdcall).set_return(emu, u64::from(mode));
}

pub fn SetErrorMode(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let mode = api_try!(emu, call.arg_u32(emu, 0));
    let previous = emu.process_error_mode;
    emu.process_error_mode = mode | (previous & SEM_NOALIGNMENTFAULTEXCEPT);
    call.cleanup_args(emu, 1);
    call.set_return(emu, u64::from(previous));
}

pub fn GetThreadErrorMode(emu: &mut Emu) {
    let mode = emu.current_thread().error_mode;
    WinApi(Stdcall).set_return(emu, u64::from(mode));
}

pub fn SetThreadErrorMode(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let mode = api_try!(emu, call.arg_u32(emu, 0));
    let output = api_try!(emu, call.arg(emu, 1));
    let previous = emu.current_thread().error_mode;
    let success =
        if mode & !(SEM_FAILCRITICALERRORS | SEM_NOGPFAULTERRORBOX | SEM_NOOPENFILEERRORBOX) != 0 {
            emu.set_last_error(ERROR_INVALID_PARAMETER);
            false
        } else if output != 0
            && (!emu.maps.validate_write_range(output, 4)
                || !emu.maps.write_dword(output, previous))
        {
            emu.set_last_error(ERROR_NOACCESS);
            false
        } else {
            emu.current_thread_mut().error_mode = mode;
            true
        };
    call.cleanup_args(emu, 2);
    call.set_return(emu, u64::from(success));
}

pub(crate) fn native_open_process(emu: &mut Emu, call: WindowsAbi) {
    let [output, desired_access, attributes, client] = api_try!(emu, call.args(emu));
    log_red!(
        emu,
        "ntdll!NtOpenProcess process_handle={output:#x} desired_access={desired_access:#x} object_attributes={attributes:#x} client_id={client:#x}"
    );
    // OBJECT_ATTRIBUTES holds a length and five pointer-aligned fields.
    let width = pointer_size(emu);
    let oa_size = 6 * width;
    let result = (|| {
        if !writable_pointer(emu, output) {
            return Some(STATUS_ACCESS_VIOLATION);
        }
        let (Some(attr), Some(cid)) = (
            read_guest_bytes(emu, attributes, oa_size as usize),
            read_guest_bytes(emu, client, (2 * width) as usize),
        ) else {
            return Some(STATUS_ACCESS_VIOLATION);
        };
        let len = u32::from_le_bytes(attr[..4].try_into().ok()?);
        if u64::from(len) != oa_size {
            return Some(STATUS_INVALID_PARAMETER);
        }
        let slot = width as usize;
        let flags = u32::from_le_bytes(
            attr[3 * slot..3 * slot + size_of::<u32>()]
                .try_into()
                .ok()?,
        );
        if flags & !OBJ_INHERIT != 0
            || attr[slot..3 * slot]
                .iter()
                .chain(&attr[4 * slot..])
                .any(|b| *b != 0)
            || cid[slot..].iter().any(|b| *b != 0)
        {
            return None;
        }
        let pid = read_pointer(emu, client)?;
        if pid != current_process_id(emu) {
            return Some(STATUS_INVALID_CID);
        }
        let handle =
            emu.handle_management
                .insert_process_handle(pid as u32, desired_access as u32, flags);
        write_pointer(emu, output, u64::from(handle));
        Some(STATUS_SUCCESS)
    })();
    if let Some(status) = result {
        call.set_return(emu, status);
        call.cleanup_args(emu, 4);
    } else {
        policy::unsupported_with_abi(emu, "ntdll", "NtOpenProcess", emu.pc(), call);
    }
}

pub fn OpenProcess(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let access = api_try!(emu, call.arg_u32(emu, 0));
    let inherit = api_try!(emu, call.arg_u32(emu, 1));
    let pid = api_try!(emu, call.arg_u32(emu, 2));
    let handle = if pid != 0 && u64::from(pid) == current_process_id(emu) {
        emu.handle_management.insert_process_handle(
            pid,
            access,
            if inherit != 0 { OBJ_INHERIT } else { 0 },
        )
    } else {
        emu.set_last_error(ERROR_INVALID_PARAMETER);
        0
    };
    call.cleanup_args(emu, 3);
    call.set_return(emu, u64::from(handle));
}

pub fn GetProcessId(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let handle = api_try!(emu, call.arg(emu, 0));
    let id = if query_current_process(emu, handle) {
        current_process_id(emu)
    } else {
        emu.set_last_error(ERROR_INVALID_HANDLE);
        0
    };
    call.cleanup_args(emu, 1);
    call.set_return(emu, id);
}

pub fn GetProcessIdOfThread(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let handle = api_try!(emu, call.arg(emu, 0));
    let id = if threads::thread_index(emu, handle).is_some() {
        current_process_id(emu)
    } else {
        emu.set_last_error(ERROR_INVALID_HANDLE);
        0
    };
    call.cleanup_args(emu, 1);
    call.set_return(emu, id);
}

fn exit_code(emu: &mut Emu, thread: bool) {
    let call = WinApi(Stdcall);
    let handle = api_try!(emu, call.arg(emu, 0));
    let output = api_try!(emu, call.arg(emu, 1));
    let code = if thread {
        threads::thread_index(emu, handle)
            .map(|index| emu.threads[index].exit_value.unwrap_or(STATUS_PENDING))
    } else {
        query_current_process(emu, handle).then_some(STATUS_PENDING)
    };
    let success = if let Some(code) = code {
        let written = output != 0
            && emu.maps.validate_write_range(output, 4)
            && emu.maps.write_dword(output, code as u32);
        if !written {
            emu.set_last_error(ERROR_NOACCESS);
        }
        written
    } else {
        emu.set_last_error(ERROR_INVALID_HANDLE);
        false
    };
    call.cleanup_args(emu, 2);
    call.set_return(emu, u64::from(success));
}

pub fn GetExitCodeProcess(emu: &mut Emu) {
    exit_code(emu, false);
}

pub fn GetExitCodeThread(emu: &mut Emu) {
    exit_code(emu, true);
}

const DEP_ENABLED_WITHOUT_ATL: u32 = PROCESS_DEP_ENABLE | PROCESS_DEP_DISABLE_ATL_THUNK_EMULATION;

fn process_dep_flags(emu: &Emu) -> u32 {
    if emu.cfg.arch.is_64bits()
        || emu
            .pe32
            .as_ref()
            .is_some_and(|pe| pe.opt.dll_characteristics & IMAGE_DLLCHARACTERISTICS_NX_COMPAT != 0)
    {
        DEP_ENABLED_WITHOUT_ATL
    } else {
        emu.process_dep_flags
    }
}

fn GetSystemDEPPolicy(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    call.set_return(emu, u64::from(DEP_OPT_IN));
}

fn SetProcessDEPPolicy(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let flags = api_try!(emu, call.arg_u32(emu, 0));
    let error = if emu.cfg.arch.is_64bits() {
        Some(ERROR_NOT_SUPPORTED)
    } else if !matches!(flags, 0 | PROCESS_DEP_ENABLE | DEP_ENABLED_WITHOUT_ATL) {
        Some(ERROR_INVALID_PARAMETER)
    } else if process_dep_flags(emu) & PROCESS_DEP_ENABLE != 0 {
        Some(ERROR_ACCESS_DENIED)
    } else {
        emu.process_dep_flags = flags;
        None
    };
    if let Some(error) = error {
        emu.set_last_error(error);
    }
    call.cleanup_args(emu, 1);
    call.set_return(emu, u64::from(error.is_none()));
}

fn GetProcessDEPPolicy(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let [handle, flags_output, permanent_output] = api_try!(emu, call.args(emu));
    if let Err(status) = require_memory_access(emu, handle, PROCESS_QUERY_INFORMATION) {
        let error = if status == STATUS_INVALID_HANDLE {
            ERROR_INVALID_HANDLE
        } else {
            ERROR_ACCESS_DENIED
        };
        emu.set_last_error(error);
        call.cleanup_args(emu, 3);
        call.set_return(emu, 0);
        return;
    }

    let flags = process_dep_flags(emu);
    let permanent = u32::from(flags & PROCESS_DEP_ENABLE != 0);
    for (address, value) in [(flags_output, flags), (permanent_output, permanent)] {
        if !emu.maps.write_dword(address, value) {
            emu.memory_exception(
                ExceptionType::WritingWord,
                address,
                size_of::<u32>(),
                MemoryAccess::Write,
            );
            return;
        }
    }
    call.cleanup_args(emu, 3);
    call.set_return(emu, 1);
}

pub(crate) fn dispatch(api: &str, library: &str, emu: &mut Emu) -> bool {
    let (params, handler): (&str, fn(&mut Emu)) = match api {
        "GetErrorMode" => ("", GetErrorMode),
        "SetErrorMode" => ("mode", SetErrorMode),
        "GetSystemDEPPolicy" if library == "kernel32" => ("", GetSystemDEPPolicy),
        "SetProcessDEPPolicy" if library == "kernel32" => ("flags", SetProcessDEPPolicy),
        "GetProcessDEPPolicy" if library == "kernel32" => {
            ("process flags permanent", GetProcessDEPPolicy)
        }
        "GetThreadErrorMode" => ("", GetThreadErrorMode),
        "SetThreadErrorMode" => ("mode old_mode", SetThreadErrorMode),
        "OpenProcess" => ("access inherit pid", OpenProcess),
        "GetProcessId" => ("process", GetProcessId),
        "GetProcessIdOfThread" => ("thread", GetProcessIdOfThread),
        "GetExitCodeProcess" => ("process code", GetExitCodeProcess),
        "GetExitCodeThread" => ("thread code", GetExitCodeThread),
        _ => return false,
    };
    trace::api(emu, &format!("{library}!{api}"), params, true, handler);
    true
}

pub(crate) fn query_information_thread(emu: &mut Emu, call: WindowsAbi) {
    trace::invoke(
        emu,
        call,
        "ntdll!NtQueryInformationThread",
        "thread class buffer length returned",
        true,
        |emu| {
            let handle = api_try!(emu, call.arg(emu, 0));
            let class = u64::from(api_try!(emu, call.arg_u32(emu, 1)));
            let buffer = api_try!(emu, call.arg(emu, 2));
            let length = api_try!(emu, call.arg_u32(emu, 3));
            let returned = api_try!(emu, call.arg(emu, 4));
            let status = query_thread(emu, handle, class, buffer, length, returned);
            call.cleanup_args(emu, 5);
            call.set_return(emu, status);
        },
    );
}

fn query_thread(
    emu: &mut Emu,
    handle: u64,
    class: u64,
    buffer: u64,
    length: u32,
    returned: u64,
) -> u64 {
    let width = pointer_size(emu) as usize;
    let size = match class {
        THREAD_INFORMATION_CLASS_THREAD_BASIC_INFORMATION => 5 * width + 2 * size_of::<i32>(),
        THREAD_INFORMATION_CLASS_THREAD_QUERY_SET_WIN32_START_ADDRESS => width,
        _ => return STATUS_NOT_IMPLEMENTED,
    };
    if (length as usize) < size {
        return STATUS_INFO_LENGTH_MISMATCH;
    }
    let Some(index) = threads::thread_index(emu, handle) else {
        return STATUS_INVALID_HANDLE;
    };
    if buffer == 0
        || !emu.maps.validate_write_range(buffer, size as u64)
        || (returned != 0
            && !emu
                .maps
                .validate_write_range(returned, size_of::<u32>() as u64))
    {
        return STATUS_ACCESS_VIOLATION;
    }
    let thread = &emu.threads[index];
    let mut data = vec![0u8; size];
    if class == THREAD_INFORMATION_CLASS_THREAD_BASIC_INFORMATION {
        let teb = emu.windows_thread_teb(index).unwrap_or(0);
        data[..4]
            .copy_from_slice(&(thread.exit_value.unwrap_or(STATUS_PENDING) as u32).to_le_bytes());
        for (slot, value) in data[width..5 * width].chunks_exact_mut(width).zip([
            teb,
            current_process_id(emu),
            thread.id,
            1,
        ]) {
            slot.copy_from_slice(&value.to_le_bytes()[..width]);
        }
        for priority in data[5 * width..].chunks_exact_mut(size_of::<i32>()) {
            priority.copy_from_slice(&(BASE_PRIORITY_NORMAL as i32).to_le_bytes());
        }
    } else if let Some(start) = thread.windows_start_address {
        data.copy_from_slice(&start.to_le_bytes()[..width]);
    } else {
        return STATUS_NOT_IMPLEMENTED;
    }
    if !emu.maps.write_bytes(buffer, &data)
        || (returned != 0 && !emu.maps.write_dword(returned, size as u32))
    {
        return STATUS_ACCESS_VIOLATION;
    }
    STATUS_SUCCESS
}

pub(crate) fn process_image(emu: &mut Emu, api: &str) {
    let call = WinApi(Stdcall);
    let [handle, flags, out, size] = api_try!(emu, call.args(emu));
    let result = (|| {
        if !query_current_process(emu, handle) {
            return Err(ERROR_INVALID_HANDLE);
        }
        if let Some(access) = emu.handle_management.process_access(handle)
            && access
                & (PROCESS_QUERY_INFORMATION
                    | PROCESS_QUERY_LIMITED_INFORMATION
                    | GENERIC_READ
                    | GENERIC_ALL
                    | MAXIMUM_ALLOWED)
                == 0
        {
            return Err(ERROR_ACCESS_DENIED);
        }
        let path = modules::main_module_path(emu);
        let path = match flags as u32 {
            0 => path,
            PROCESS_NAME_NATIVE => path
                .strip_prefix("C:\\")
                .map(|tail| format!("\\Device\\HarddiskVolume1\\{tail}"))
                .ok_or(ERROR_INVALID_PARAMETER)?,
            _ => return Err(ERROR_INVALID_PARAMETER),
        };
        let codec = Encoding::from_wide(api.ends_with('W'));
        let bytes = codec.terminated(&path.encode_utf16().collect::<Vec<_>>());
        let capacity = emu.maps.read_dword(size).ok_or(ERROR_NOACCESS)? as usize;
        let units = bytes.len() / codec.width();
        if units > capacity {
            return Err(ERROR_INSUFFICIENT_BUFFER);
        }
        if out == 0
            || size == 0
            || !emu.maps.validate_write_range(size, 4)
            || !emu.maps.validate_write_range(out, bytes.len() as u64)
        {
            return Err(ERROR_NOACCESS);
        }
        emu.maps.write_bytes(out, &bytes);
        emu.maps.write_dword(size, (units - 1) as u32);
        Ok(())
    })();
    log_red!(emu, "{api} process={handle:#x}");
    if let Err(error) = result {
        emu.set_last_error(error);
    }
    call.set_return(emu, u64::from(result.is_ok()));
    call.cleanup_args(emu, 4);
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub(crate) struct Priorities {
    class: u32,
    threads: std::collections::BTreeMap<u64, i32>,
}
impl Default for Priorities {
    fn default() -> Self {
        Self {
            class: NORMAL_PRIORITY_CLASS,
            threads: Default::default(),
        }
    }
}

pub(crate) fn priority(emu: &mut Emu, api: &str) {
    let call = WinApi(Stdcall);
    let handle = api_try!(emu, call.arg(emu, 0));
    let set = api.starts_with("Set");
    let value = if set {
        api_try!(emu, call.arg_i32(emu, 1))
    } else {
        0
    };
    let thread = api.ends_with("ThreadPriority");
    let result = (|| {
        if thread {
            let idx = threads::thread_index(emu, handle).ok_or(ERROR_INVALID_HANDLE)?;
            let id = emu.threads[idx].id;
            if set {
                if !matches!(
                    value,
                    THREAD_PRIORITY_IDLE | THREAD_PRIORITY_TIME_CRITICAL | -2..=2
                ) {
                    return Err(ERROR_INVALID_PARAMETER);
                }
                emu.priorities.threads.insert(id, value);
                Ok(1)
            } else {
                Ok(u64::from(
                    emu.priorities.threads.get(&id).copied().unwrap_or_default() as u32,
                ))
            }
        } else {
            if !query_current_process(emu, handle) {
                return Err(ERROR_INVALID_HANDLE);
            }
            if let Some(rights) = emu.handle_management.process_access(handle) {
                let allowed = if set {
                    PROCESS_SET_INFORMATION | GENERIC_WRITE
                } else {
                    PROCESS_QUERY_INFORMATION | PROCESS_QUERY_LIMITED_INFORMATION | GENERIC_READ
                };
                if rights & (allowed | GENERIC_ALL | MAXIMUM_ALLOWED) == 0 {
                    return Err(ERROR_ACCESS_DENIED);
                }
            }
            if set {
                if !matches!(
                    value as u32,
                    IDLE_PRIORITY_CLASS
                        | BELOW_NORMAL_PRIORITY_CLASS
                        | NORMAL_PRIORITY_CLASS
                        | ABOVE_NORMAL_PRIORITY_CLASS
                        | HIGH_PRIORITY_CLASS
                        | REALTIME_PRIORITY_CLASS
                ) {
                    return Err(ERROR_INVALID_PARAMETER);
                }
                emu.priorities.class = value as u32;
                Ok(1)
            } else {
                Ok(u64::from(emu.priorities.class))
            }
        }
    })();
    let value = result.unwrap_or_else(|e| {
        emu.set_last_error(e);
        if thread && !set {
            THREAD_PRIORITY_ERROR_RETURN
        } else {
            0
        }
    });
    log_red!(emu, "{api} handle={handle:#x}");
    call.set_return(emu, value);
    call.cleanup_args(emu, 1 + usize::from(set));
}

pub(crate) fn system_info(emu: &mut Emu, library: &str, api: &str) {
    let call = WinApi(Stdcall);
    let info = api_try!(emu, call.arg(emu, 0));
    let wide = emu.cfg.arch.is_64bits();
    log_red!(emu, "{library}!{api} system_info={info:#x}");
    if !save_system_info(
        &mut emu.maps,
        info,
        wide,
        wide || api == "GetNativeSystemInfo",
    ) {
        emu.exception(ExceptionType::BadAddressDereferencing);
        return;
    }
    call.cleanup_args(emu, 1);
}

pub(crate) fn process_platform(emu: &mut Emu, library: &str, api: &str) {
    let call = WinApi(Stdcall);
    let handle = api_try!(emu, call.arg(emu, 0));
    let output = api_try!(emu, call.arg(emu, 1));
    let machines = api == "IsWow64Process2";
    let native_machine = if machines {
        api_try!(emu, call.arg(emu, 2))
    } else {
        0
    };
    match api {
        "IsWow64Process2" => log_red!(
            emu,
            "{library}!{api} process={handle:#x} process_machine={output:#x} native_machine={native_machine:#x}"
        ),
        "IsWow64Process" => log_red!(emu, "{library}!{api} process={handle:#x} wow64={output:#x}"),
        _ => log_red!(
            emu,
            "{library}!{api} process={handle:#x} debugger_present={output:#x}"
        ),
    }
    let result = (|| {
        let debugger = api == "CheckRemoteDebuggerPresent";
        if debugger && (handle == 0 || output == 0) {
            return Err(ERROR_INVALID_PARAMETER);
        }
        if !query_current_process(emu, handle) {
            return Err(ERROR_INVALID_HANDLE);
        }
        let mut rights = PROCESS_QUERY_INFORMATION | GENERIC_READ | GENERIC_ALL | MAXIMUM_ALLOWED;
        if !debugger {
            rights |= PROCESS_QUERY_LIMITED_INFORMATION;
        }
        if emu
            .handle_management
            .process_access(handle)
            .is_some_and(|access| access & rights == 0)
        {
            return Err(ERROR_ACCESS_DENIED);
        }
        let wow64 = !emu.cfg.arch.is_64bits();
        if machines {
            if output == 0
                || !emu.maps.validate_write_range(output, 2)
                || (native_machine != 0 && !emu.maps.validate_write_range(native_machine, 2))
            {
                return Err(ERROR_NOACCESS);
            }
            emu.maps.write_word(
                output,
                if wow64 {
                    IMAGE_FILE_MACHINE_I386
                } else {
                    COFF_MACHINE_UNKNOWN
                },
            );
            if native_machine != 0 {
                emu.maps
                    .write_word(native_machine, IMAGE_FILE_MACHINE_AMD64);
            }
        } else {
            if output == 0 || !emu.maps.validate_write_range(output, 4) {
                return Err(ERROR_NOACCESS);
            }
            let present = api == "IsWow64Process" && wow64;
            emu.maps.write_dword(output, u32::from(present));
        }
        Ok(())
    })();
    if let Err(error) = result {
        emu.set_last_error(error);
    }
    call.set_return(emu, u64::from(result.is_ok()));
    call.cleanup_args(emu, if machines { 3 } else { 2 });
}

pub(crate) fn current_package_path(emu: &mut Emu, library: &str) {
    let call = WinApi(Stdcall);
    let [length, path] = api_try!(emu, call.args(emu));
    log_red!(
        emu,
        "{library}!GetCurrentPackagePath path_length={length:#x} path={path:#x}"
    );
    let result = match emu.maps.read_dword(length) {
        _ if length == 0 => ERROR_INVALID_PARAMETER,
        None => ERROR_NOACCESS,
        Some(n) if n != 0 && path == 0 => ERROR_INVALID_PARAMETER,
        Some(_) => APPMODEL_ERROR_NO_PACKAGE,
    };
    call.set_return(emu, result);
    call.cleanup_args(emu, 2);
}

pub(crate) fn terminate(emu: &mut Emu, library: &str) {
    let call = WinApi(Stdcall);
    let handle = api_try!(emu, call.arg(emu, 0));
    let exit_code = api_try!(emu, call.arg_u32(emu, 1));
    log_red!(
        emu,
        "{library}!TerminateProcess process={handle:#x} exit_code={exit_code}"
    );
    call.cleanup_args(emu, 2);
    if !query_current_process(emu, handle) {
        emu.set_last_error(ERROR_INVALID_HANDLE);
    } else if emu
        .handle_management
        .process_access(handle)
        .is_some_and(|rights| rights & (PROCESS_TERMINATE | GENERIC_ALL | MAXIMUM_ALLOWED) == 0)
    {
        emu.set_last_error(ERROR_ACCESS_DENIED);
    } else {
        threads::force_exit_process(emu, exit_code);
        return;
    }
    call.set_return(emu, 0);
}

pub(crate) fn hardware_support(emu: &mut Emu, library: &str, api: &str) {
    let call = WinApi(Stdcall);
    let feature = api_try!(emu, call.arg_u32(emu, 0));
    log_red!(emu, "{library}!{api} feature={feature:#x}");
    // The guest CPU has neither enclaves nor user-mode shadow stacks.
    call.set_return(emu, 0);
    call.cleanup_args(emu, 1);
}

pub(crate) fn processor_feature(emu: &mut Emu, library: &str) {
    let call = WinApi(Stdcall);
    let feature = api_try!(emu, call.arg_u32(emu, 0));
    let present = PROCESSOR_FEATURES.contains(&feature);
    log_red!(
        emu,
        "{library}!IsProcessorFeaturePresent feature={feature} present={present}"
    );
    call.set_return(emu, u64::from(present));
    call.cleanup_args(emu, 1);
}

pub(crate) fn process_masks(emu: &mut Emu, library: &str, api: &str) {
    let call = WinApi(Stdcall);
    let process = api_try!(emu, call.arg(emu, 0));
    let output = api_try!(emu, call.arg(emu, 1));
    let affinity = api == "GetProcessAffinityMask";
    let system = if affinity {
        api_try!(emu, call.arg(emu, 2))
    } else {
        0
    };
    if affinity {
        log_red!(
            emu,
            "{library}!{api} process={process:#x} process_mask={output:#x} system_mask={system:#x}"
        );
    } else {
        log_red!(
            emu,
            "{library}!{api} process={process:#x} critical={output:#x}"
        );
    }
    let result = if !query_current_process(emu, process) {
        Err(ERROR_INVALID_HANDLE)
    } else if emu
        .handle_management
        .process_access(process)
        .is_some_and(|a| {
            a & (GENERIC_ALL
                | MAXIMUM_ALLOWED
                | PROCESS_QUERY_INFORMATION
                | PROCESS_QUERY_LIMITED_INFORMATION)
                == 0
        })
    {
        Err(ERROR_ACCESS_DENIED)
    } else if affinity
        && writable_pointer(emu, output)
        && writable_pointer(emu, system)
        && output != 0
        && system != 0
    {
        write_pointer(emu, output, u64::from(ACTIVE_PROCESSOR_MASK));
        write_pointer(emu, system, u64::from(ACTIVE_PROCESSOR_MASK));
        Ok(())
    } else if !affinity
        && output != 0
        && emu.maps.validate_write_range(output, 4)
        && emu.maps.write_dword(output, 0)
    {
        Ok(())
    } else {
        Err(ERROR_NOACCESS)
    };
    if let Err(error) = result {
        emu.set_last_error(error);
    }
    call.set_return(emu, u64::from(result.is_ok()));
    call.cleanup_args(emu, 2 + usize::from(affinity));
}

pub(crate) fn startup_info(emu: &mut Emu, library: &str, api: &str) {
    let call = WinApi(Stdcall);
    let address = api_try!(emu, call.arg(emu, 0));
    const STARTUPINFO32_SIZE: usize = 68;
    const STARTUPINFO64_SIZE: usize = 104;
    let bytes = if emu.cfg.is_x64() {
        STARTUPINFO64_SIZE
    } else {
        STARTUPINFO32_SIZE
    };
    log_red!(emu, "{library}!{api} startup_info={address:#x}");
    if address == 0 || !emu.maps.validate_write_range(address, bytes as u64) {
        emu.exception(ExceptionType::BadAddressDereferencing);
        return;
    }
    emu.maps.fill_range(address, 0, bytes);
    emu.maps.write_dword(address, bytes as u32);
    call.cleanup_args(emu, 1);
}

const ALL_PROCESSOR_GROUPS: u16 = u16::MAX;
pub(crate) fn processor_count(emu: &mut Emu, library: &str, api: &str) {
    let call = WinApi(Stdcall);
    if api == "GetCurrentProcessorNumber" {
        log_red!(emu, "{library}!{api} processor=0");
        call.set_return(emu, 0);
        return;
    }
    let group = api_try!(emu, call.arg_u16(emu, 0));
    let count = if group == 0 || group == ALL_PROCESSOR_GROUPS {
        PROCESSORS
    } else {
        emu.set_last_error(ERROR_INVALID_PARAMETER);
        0
    };
    log_red!(emu, "{library}!{api} group={group} count={count}");
    call.set_return(emu, u64::from(count));
    call.cleanup_args(emu, 1);
}

fn being_debugged(emu: &Emu) -> bool {
    emu.maps
        .get_map_by_name("peb")
        .and_then(|map| {
            emu.maps
                .read_byte(map.get_base() + PEB_BEING_DEBUGGED_OFFSET)
        })
        .unwrap_or(0)
        != 0
}

pub(crate) fn debugger_present(emu: &mut Emu, library: &str) {
    let call = WinApi(Stdcall);
    let present = being_debugged(emu);
    log_red!(emu, "{library}!IsDebuggerPresent present={present}");
    call.set_return(emu, u64::from(present));
}

pub(crate) fn last_error(emu: &mut Emu, library: &str, api: &str) {
    let call = WinApi(Stdcall);
    if api != "GetLastError" {
        let error = api_try!(emu, call.arg_u32(emu, 0));
        log_red!(emu, "{library}!{api} error={error}");
        emu.set_last_error(u64::from(error));
        call.cleanup_args(emu, 1);
    } else {
        let error = emu.get_last_error();
        log_red!(emu, "{library}!{api} => {error}");
        call.set_return(emu, error);
    }
}

pub(crate) fn working_set(emu: &mut Emu, library: &str) {
    let call = WinApi(Stdcall);
    let [process, minimum, maximum] = api_try!(emu, call.args(emu));
    log_red!(
        emu,
        "{library}!GetProcessWorkingSetSize process={process:#x} minimum_size={minimum:#x} maximum_size={maximum:#x}"
    );
    const DEFAULT_MINIMUM_WORKING_SET: u64 = 200 * 1024;
    const DEFAULT_MAXIMUM_WORKING_SET: u64 = 345 * 4096;
    let outcome = if !query_current_process(emu, process) {
        Err(ERROR_INVALID_HANDLE)
    } else if emu
        .handle_management
        .process_access(process)
        .is_some_and(|access| {
            access & (PROCESS_QUERY_INFORMATION | PROCESS_QUERY_LIMITED_INFORMATION) == 0
        })
    {
        Err(ERROR_ACCESS_DENIED)
    } else if minimum == 0
        || maximum == 0
        || !writable_pointer(emu, minimum)
        || !writable_pointer(emu, maximum)
    {
        Err(ERROR_NOACCESS)
    } else {
        write_pointer(emu, minimum, DEFAULT_MINIMUM_WORKING_SET);
        write_pointer(emu, maximum, DEFAULT_MAXIMUM_WORKING_SET);
        Ok(())
    };
    if let Err(error) = outcome {
        emu.set_last_error(error);
    }
    call.set_return(emu, u64::from(outcome.is_ok()));
    call.cleanup_args(emu, 3);
}

pub(crate) fn debug_output(emu: &mut Emu, library: &str, api: &str) {
    let call = WinApi(Stdcall);
    let address = api_try!(emu, call.arg(emu, 0));
    let message = Encoding::from_wide(api.ends_with('W'))
        .read(emu, address)
        .map(|units| String::from_utf16_lossy(&units));
    log_red!(emu, "{library}!{api} message={message:?}");
    call.cleanup_args(emu, 1);
}

pub(crate) fn set_unhandled_exception_filter(emu: &mut Emu, library: &str) {
    let call = WinApi(Stdcall);
    let callback = api_try!(emu, call.arg(emu, 0));
    let previous = emu.uef();
    log_red!(
        emu,
        "{library}!SetUnhandledExceptionFilter callback={callback:#x} => {previous:#x}"
    );
    emu.set_uef(callback);
    call.set_return(emu, previous);
    call.cleanup_args(emu, 1);
}

pub(crate) fn unhandled_exception_filter(emu: &mut Emu, library: &str) {
    let call = WinApi(Stdcall);
    let info = api_try!(emu, call.arg(emu, 0));
    let Some(record) = read_pointer(emu, info) else {
        emu.memory_exception(
            ExceptionType::BadAddressDereferencing,
            info,
            pointer_size(emu) as usize,
            MemoryAccess::Read,
        );
        return;
    };
    let Some(code) = emu.maps.read_dword(record) else {
        emu.memory_exception(
            ExceptionType::BadAddressDereferencing,
            record,
            size_of::<u32>(),
            MemoryAccess::Read,
        );
        return;
    };
    let callback = emu.uef();
    let disposition = if being_debugged(emu) {
        EXCEPTION_CONTINUE_SEARCH
    } else if callback == 0 {
        EXCEPTION_EXECUTE_HANDLER
    } else {
        let Some(value) =
            policy::invoke_callback(emu, library, "UnhandledExceptionFilter", callback, [info])
        else {
            return;
        };
        match value as u32 {
            EXCEPTION_CONTINUE_SEARCH => EXCEPTION_EXECUTE_HANDLER,
            value => value,
        }
    };
    log_red!(
        emu,
        "{library}!UnhandledExceptionFilter info={info:#x} code={code:#x} callback={callback:#x} => {disposition:#x}"
    );
    call.set_return(emu, u64::from(disposition));
    call.cleanup_args(emu, 1);
}

pub(crate) fn raise_exception(emu: &mut Emu, library: &str) {
    let call = WinApi(Stdcall);
    let code = api_try!(emu, call.arg_u32(emu, 0));
    let flags = api_try!(emu, call.arg_u32(emu, 1));
    let count = api_try!(emu, call.arg_u32(emu, 2)) as usize;
    let arguments = api_try!(emu, call.arg(emu, 3));
    let width = pointer_size(emu);
    log_red!(
        emu,
        "{library}!RaiseException code={code:#x} flags={flags:#x} count={count} arguments={arguments:#x}"
    );
    if emu.cfg.arch.is_aarch64() {
        policy::unsupported(emu, library, "RaiseException", emu.pc());
        return;
    }
    let mut information = Vec::new();
    if arguments != 0 {
        for index in 0..count.min(EXCEPTION_MAXIMUM_PARAMETERS) {
            let Some(value) = arguments
                .checked_add(index as u64 * width)
                .and_then(|ptr| read_pointer(emu, ptr))
            else {
                emu.exception(ExceptionType::BadAddressDereferencing);
                return;
            };
            information.push(value);
        }
    }
    call.cleanup_args(emu, 4);
    let record = ExceptionRecord {
        code,
        flags,
        address: emu.pc(),
        nested: handlers::active_record(emu).unwrap_or(0),
        information,
    };
    if emu.cfg.is_x64() {
        if !raise::raise64(emu, code, flags, &record.information)
            && !emu.process_terminated
            && !emu.api_stopped()
        {
            emu.dispatch_structured_exception(&record);
        }
    } else {
        emu.dispatch_vectored_exception(&record, 0);
    }
}
