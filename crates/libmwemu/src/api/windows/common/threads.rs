//! Windows thread creation and integer/control context operations.
use crate::api::abi::windows::{
    CallingConvention::{Cdecl, Stdcall},
    WindowsAbi,
    WindowsAbi::WinApi,
    X64_SHADOW_SPACE,
};
use crate::context::{context32::Context32, context64::Context64};
use crate::emu::Emu;
use crate::exception::types::ExceptionType;
use crate::maps::mem64::Permission;
use crate::maps::virtual_memory::{ALLOCATION_GRANULARITY, PAGE_SIZE, align_up};
use crate::threading::context::{ArchThreadState, ThreadContext};
use crate::winapi::common::trace;
use crate::winapi::common::{fls, memory, process};
use crate::winapi::helper;
use crate::winapi::policy::{self, api_try};
use crate::windows::constants::errors::*;
use crate::windows::constants::exceptions::*;
use crate::windows::constants::memory::*;
use crate::windows::constants::process::*;
use crate::windows::structures::{TEB, TEB64};

const FIRST_THREAD_ID: u64 = 0x1000;

pub(super) fn thread_index(emu: &Emu, handle: u64) -> Option<usize> {
    if handle == NT_CURRENT_THREAD
        || (!emu.cfg.arch.is_64bits() && handle == NT_CURRENT_THREAD as u32 as u64)
    {
        return Some(emu.current_thread_id);
    }
    let id = emu.handle_management.thread_handle_id(handle).or_else(|| {
        // Retain handles created by older entry paths while they migrate.
        let uri = helper::handler_get_uri(handle);
        let id = uri.strip_prefix("tid://")?;
        if let Some(hex) = id.strip_prefix("0x") {
            u64::from_str_radix(hex, 16).ok()
        } else {
            id.parse().ok()
        }
    })?;
    emu.threads.iter().position(|thread| thread.id == id)
}

fn finish(emu: &mut Emu, result: Result<u64, u64>, failure: u64, args: usize) {
    let call = WinApi(Stdcall);
    let value = match result {
        Ok(value) => value,
        Err(error) => {
            emu.set_last_error(error);
            failure
        }
    };
    call.set_return(emu, value);
    call.cleanup_args(emu, args);
}

fn stack_size(emu: &Emu, requested: u64, flags: u64) -> Option<u64> {
    let reserve = if emu.cfg.is_x64() {
        emu.pe64.as_ref().map(|pe| pe.opt.size_of_stack_reserve)
    } else {
        emu.pe32
            .as_ref()
            .map(|pe| pe.opt.size_of_stack_reserve as u64)
    }
    .filter(|size| *size != 0)
    .unwrap_or(DEFAULT_STACK_RESERVE);
    let size = if requested == 0 {
        reserve
    } else if flags & STACK_SIZE_PARAM_IS_A_RESERVATION != 0 {
        requested
    } else {
        reserve.max(requested)
    };
    align_up(size.max(PAGE_SIZE), ALLOCATION_GRANULARITY)
}

fn create_teb(emu: &mut Emu, id: u64, stack_base: u64, stack_size: u64) -> Result<u64, u64> {
    let source = emu.windows_teb();
    let template = source
        .and_then(|base| emu.maps.get_mem_by_addr(base))
        .map(|map| map.get_mem());
    let size = template
        .as_ref()
        .map_or(PAGE_SIZE, |bytes| (bytes.len() as u64).max(PAGE_SIZE));
    let (base, _) = memory::allocate(emu, 0, size, MEM_RESERVE | MEM_COMMIT, PAGE_READWRITE)
        .map_err(|_| ERROR_NOT_ENOUGH_MEMORY)?;
    if let Some(bytes) = template
        && !emu.maps.write_bytes(base, &bytes)
    {
        let _ = emu.maps.release_memory(base);
        return Err(ERROR_NOACCESS);
    }
    let pid = process::current_process_id(emu);
    let peb = emu
        .maps
        .get_map_by_name("peb")
        .map_or(0, |map| map.get_base());
    if emu.cfg.arch.is_64bits() {
        let mut teb =
            source.map_or_else(|| TEB64::new(peb), |source| TEB64::load(source, &emu.maps));
        teb.process_id = pid;
        teb.thread_id = id;
        teb.last_error_value = 0;
        teb.nt_tib.self_pointer = base;
        teb.nt_tib.stack_base = stack_base + stack_size;
        teb.nt_tib.stack_limit = stack_base;
        teb.nt_tib.exception_list = 0;
        let map = emu.maps.get_mem_by_addr_mut(base).ok_or(ERROR_NOACCESS)?;
        teb.save(map);
    } else {
        let mut teb = source.map_or_else(
            || TEB::new(peb as u32),
            |source| TEB::load(source, &emu.maps),
        );
        teb.process_id = pid as u32;
        teb.thread_id = id as u32;
        teb.last_error_value = 0;
        teb.nt_tib.self_pointer = base as u32;
        teb.nt_tib.stack_base = (stack_base + stack_size) as u32;
        teb.nt_tib.stack_limit = stack_base as u32;
        teb.nt_tib.exception_list = EXCEPTION_CHAIN_END;
        let map = emu.maps.get_mem_by_addr_mut(base).ok_or(ERROR_NOACCESS)?;
        teb.save(map);
        map.write_dword(base + TEB::DEALLOCATION_STACK_OFFSET, stack_base as u32);
    }
    Ok(base)
}

pub(crate) fn create(
    emu: &mut Emu,
    [requested, start, parameter, flags, id_ptr]: [u64; 5],
) -> Result<u64, u64> {
    if flags & !(CREATE_SUSPENDED | STACK_SIZE_PARAM_IS_A_RESERVATION) != 0 {
        return Err(ERROR_INVALID_PARAMETER);
    }
    if id_ptr != 0 && !emu.maps.validate_write_range(id_ptr, 4) {
        return Err(ERROR_NOACCESS);
    }
    let size = stack_size(emu, requested, flags).ok_or(ERROR_NOT_ENOUGH_MEMORY)?;
    // Commit the usable stack eagerly: secondary-thread guard-page growth is not modelled.
    let (base, length) = memory::allocate(emu, 0, size, MEM_RESERVE | MEM_COMMIT, PAGE_READWRITE)
        .map_err(|_| ERROR_NOT_ENOUGH_MEMORY)?;
    let id = emu
        .threads
        .iter()
        .map(|thread| thread.id)
        .max()
        .unwrap_or(FIRST_THREAD_ID)
        .max(FIRST_THREAD_ID)
        + 1;
    let mut thread = ThreadContext::new(id, emu.cfg.arch);
    thread.windows_start_address = Some(start);
    let regs = thread.regs_x86_mut();
    regs.rip = start;
    let word_size = memory::pointer_size(emu);
    if emu.cfg.is_x64() {
        regs.rsp = base + length - X64_SHADOW_SPACE - word_size;
        regs.rcx = parameter;
        emu.maps.write_qword(regs.rsp, RETURN_THREAD as u64);
    } else {
        regs.rsp = base + length - 2 * word_size;
        emu.maps.write_dword(regs.rsp, RETURN_THREAD);
        emu.maps.write_dword(regs.rsp + word_size, parameter as u32);
    }
    thread.windows.suspend_count = u32::from(flags & CREATE_SUSPENDED != 0);
    thread.suspended = thread.windows.suspend_count != 0;
    thread.windows.teb = match create_teb(emu, id, base, length) {
        Ok(teb) => teb,
        Err(error) => {
            let _ = emu.maps.release_memory(base);
            return Err(error);
        }
    };
    if id_ptr != 0 && !emu.maps.write_dword(id_ptr, id as u32) {
        let _ = emu.maps.release_memory(base);
        let _ = emu.maps.release_memory(thread.windows.teb);
        return Err(ERROR_NOACCESS);
    }
    let handle = emu.handle_management.insert_thread_handle(id) as u64;
    thread.handle = handle;
    emu.threads.push(thread);
    Ok(handle)
}

pub(crate) fn create_thread(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let arguments = [
        api_try!(emu, call.arg(emu, 1)),
        api_try!(emu, call.arg(emu, 2)),
        api_try!(emu, call.arg(emu, 3)),
        api_try!(emu, call.arg_u32(emu, 4)) as u64,
        api_try!(emu, call.arg(emu, 5)),
    ];
    let result = create(emu, arguments);
    finish(emu, result, 0, 6);
}

pub(crate) fn resume_thread(emu: &mut Emu) {
    change_suspend_count(emu, false);
}

pub(crate) fn suspend_thread(emu: &mut Emu) {
    change_suspend_count(emu, true);
}

fn change_suspend_count(emu: &mut Emu, suspend: bool) {
    let call = WinApi(Stdcall);
    let handle = api_try!(emu, call.arg(emu, 0));
    let result = thread_index(emu, handle)
        .ok_or(ERROR_INVALID_HANDLE)
        .and_then(|index| {
            let thread = &mut emu.threads[index];
            // Older snapshots and host callers can mark a thread suspended without a count.
            let previous = thread
                .windows
                .suspend_count
                .max(u32::from(thread.suspended));
            if suspend && previous >= MAXIMUM_SUSPEND_COUNT {
                return Err(ERROR_SIGNAL_REFUSED);
            }
            thread.windows.suspend_count = if suspend {
                previous + 1
            } else {
                previous.saturating_sub(1)
            };
            thread.suspended = thread.windows.suspend_count != 0;
            Ok(previous as u64)
        });
    finish(emu, result, u32::MAX as u64, 1);
}

pub fn GetCurrentThreadId(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let id = emu.current_thread().id;
    log_red!(emu, "kernel32!GetCurrentThreadId => {}", id);
    call.set_return(emu, id);
}

pub fn GetThreadId(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let handle = api_try!(emu, call.arg(emu, 0));
    let result = thread_index(emu, handle)
        .map(|index| emu.threads[index].id)
        .ok_or(ERROR_INVALID_HANDLE);
    log_red!(
        emu,
        "kernel32!GetThreadId handle=0x{:x} => {:?}",
        handle,
        result
    );
    finish(emu, result, 0, 1);
}

pub fn OpenThread(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let access = api_try!(emu, call.arg_u32(emu, 0));
    let inherit = api_try!(emu, call.arg_u32(emu, 1));
    let id = api_try!(emu, call.arg_u32(emu, 2)) as u64;
    let result = if emu
        .threads
        .iter()
        .any(|thread| thread.id == id && thread.exit_value.is_none())
    {
        Ok(emu.handle_management.insert_thread_handle(id) as u64)
    } else {
        Err(ERROR_INVALID_PARAMETER)
    };
    log_red!(
        emu,
        "kernel32!OpenThread id={} access=0x{:x} inherit={} => {:?}",
        id,
        access,
        inherit,
        result
    );
    finish(emu, result, 0, 3);
}

pub fn SuspendThread(emu: &mut Emu) {
    trace::api(
        emu,
        "kernel32!SuspendThread",
        "thread",
        true,
        suspend_thread,
    );
}

fn context(emu: &mut Emu, set: bool, handle: u64, address: u64) -> Result<u64, u64> {
    let index = thread_index(emu, handle).ok_or(ERROR_INVALID_HANDLE)?;
    let (flags_offset, size, supported) = if emu.cfg.is_x64() {
        (
            Context64::FLAGS_OFFSET,
            Context64::INTEGER_CONTROL_SIZE,
            CONTEXT_CONTROL | CONTEXT_INTEGER,
        )
    } else {
        (
            Context32::FLAGS_OFFSET,
            Context32::INTEGER_CONTROL_SIZE,
            WOW64_CONTEXT_CONTROL | WOW64_CONTEXT_INTEGER | WOW64_CONTEXT_DEBUG_REGISTERS,
        )
    };
    if !emu
        .maps
        .can_access_range(address, size as usize, Permission::READ)
        || (!set && !emu.maps.validate_write_range(address, size))
    {
        return Err(ERROR_NOACCESS);
    }
    let flags = emu
        .maps
        .read_dword(address + flags_offset)
        .ok_or(ERROR_NOACCESS)?;
    if flags & !supported != 0 {
        return Err(ERROR_NOT_SUPPORTED);
    }
    let architecture = if emu.cfg.is_x64() {
        CONTEXT_AMD64
    } else {
        WOW64_CONTEXT_I386
    };
    if flags & architecture != architecture || flags == architecture {
        return Err(ERROR_INVALID_PARAMETER);
    }
    let thread = &mut emu.threads[index];
    let ArchThreadState::X86 {
        regs,
        flags: cpu_flags,
        ..
    } = &mut thread.arch
    else {
        return Err(ERROR_NOT_SUPPORTED);
    };
    let (control, eflags_offset) = if emu.cfg.is_x64() {
        let mut context = Context64::new(regs);
        if set {
            context.load_selected(address, &emu.maps, flags);
            context.sync(regs);
        } else {
            context.save_selected(address, &mut emu.maps, flags, cpu_flags.dump());
        }
        (CONTEXT_CONTROL, Context64::EFLAGS_OFFSET)
    } else {
        let mut context = Context32::new(regs);
        if set {
            context.load_selected(address, &emu.maps, flags);
            context.sync(regs);
        } else {
            context.save_selected(address, &mut emu.maps, flags, cpu_flags.dump());
        }
        (WOW64_CONTEXT_CONTROL, Context32::EFLAGS_OFFSET)
    };
    if set && flags & control == control {
        cpu_flags.load(emu.maps.read_dword(address + eflags_offset).unwrap_or(0));
        if index == emu.current_thread_id {
            emu.force_reload = true;
        }
    }
    Ok(1)
}

pub(crate) fn set_thread_context(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let handle = api_try!(emu, call.arg(emu, 0));
    let address = api_try!(emu, call.arg(emu, 1));
    let result = context(emu, true, handle, address);
    finish(emu, result, 0, 2);
}

pub(crate) fn get_thread_context(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let handle = api_try!(emu, call.arg(emu, 0));
    let address = api_try!(emu, call.arg(emu, 1));
    let result = context(emu, false, handle, address);
    finish(emu, result, 0, 2);
}

pub(crate) fn exit_thread(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let code = api_try!(emu, call.arg_u32(emu, 0));
    call.set_return(emu, code as u64);
    call.cleanup_args(emu, 1);
    finish_thread(emu, code);
}

pub(crate) fn finish_thread(emu: &mut Emu, code: u32) {
    if fls::detach_current(emu, "ExitThread") {
        terminate_thread(emu, emu.current_thread_id, code);
    }
}

fn terminate_thread(emu: &mut Emu, index: usize, code: u32) {
    let thread = &mut emu.threads[index];
    thread.exit_value = Some(u64::from(code));
    emu.user_timers.remove_thread(thread.id);
    if (index == emu.current_thread_id && !emu.cfg.enable_threading)
        || emu.threads.iter().all(|thread| thread.exit_value.is_some())
    {
        emu.stop();
    }
}

pub(crate) fn dispatch(api: &str, library: &str, emu: &mut Emu) -> bool {
    match api {
        "TlsAlloc" | "TlsFree" | "TlsGetValue" | "TlsGetValue2" | "TlsSetValue" => {
            tls(emu, api);
            return true;
        }
        "AddVectoredExceptionHandler" | "RemoveVectoredExceptionHandler" => {
            vectored(emu, api, api == "AddVectoredExceptionHandler");
            return true;
        }
        _ => {}
    }
    let (parameters, handler): (&str, fn(&mut Emu)) = match api {
        "CreateThread" => (
            "attributes stack_size start parameter flags thread_id",
            create_thread,
        ),
        "ResumeThread" => ("thread", resume_thread),
        "SuspendThread" => ("thread", suspend_thread),
        "OpenThread" => ("access inherit thread_id", OpenThread),
        "GetThreadId" => ("thread", GetThreadId),
        "GetCurrentThreadId" => ("", GetCurrentThreadId),
        "SetThreadContext" => ("thread context", set_thread_context),
        "GetThreadContext" => ("thread context", get_thread_context),
        "ExitThread" => ("exit_code", exit_thread),
        _ => return false,
    };
    trace::api(
        emu,
        &format!("{library}!{api}"),
        parameters,
        api != "ExitThread",
        handler,
    );
    true
}

pub(crate) fn nt_terminate_thread(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let handle = api_try!(emu, call.arg(emu, 0));
    let code = api_try!(emu, call.arg_u32(emu, 1));
    log_red!(emu, "NtTerminateThread handle=0x{:x} code={}", handle, code);
    let thread = if handle == 0 {
        Some(emu.current_thread_id)
    } else {
        thread_index(emu, handle)
    };
    let status = match thread {
        Some(i) => {
            terminate_thread(emu, i, code);
            STATUS_SUCCESS
        }
        None => STATUS_INVALID_HANDLE,
    };
    call.set_return(emu, status);
    call.cleanup_args(emu, 2);
}

pub(crate) fn nt_get_context_thread(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let [handle, out] = api_try!(emu, call.args(emu));
    log_red!(
        emu,
        "ntdll!NtGetContextThread handle={handle:#x} context={out:#x}"
    );
    let status = match context(emu, false, handle, out) {
        Ok(_) => STATUS_SUCCESS,
        Err(ERROR_INVALID_HANDLE) => STATUS_INVALID_HANDLE,
        Err(ERROR_NOACCESS) => STATUS_ACCESS_VIOLATION,
        Err(ERROR_NOT_SUPPORTED) => STATUS_NOT_SUPPORTED,
        Err(_) => STATUS_INVALID_PARAMETER,
    };
    call.set_return(emu, status);
    call.cleanup_args(emu, 2);
}

pub(crate) fn exit_process(emu: &mut Emu, api: &str) {
    terminate_process(emu, api, WinApi(Stdcall));
}

pub(crate) fn terminate_crt(emu: &mut Emu, api: &str) {
    terminate_process(emu, api, WinApi(Cdecl));
}

fn terminate_process(emu: &mut Emu, api: &str, call: WindowsAbi) {
    let code = api_try!(emu, call.arg_u32(emu, 0));
    log_red!(emu, "{api} code={code}");
    call.cleanup_args(emu, 1);
    exit_with_code(emu, code);
}

pub(crate) fn exit_with_code(emu: &mut Emu, code: u32) {
    if fls::detach_current(emu, "ExitProcess") {
        force_exit_process(emu, code);
    }
}

pub(crate) fn force_exit_process(emu: &mut Emu, code: u32) {
    for thread in &mut emu.threads {
        emu.user_timers.remove_thread(thread.id);
        if thread.exit_value.is_none() {
            thread.exit_value = Some(u64::from(code));
        }
    }
    emu.stop();
}

pub(crate) fn capture_context(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let out = api_try!(emu, call.arg(emu, 0));
    let is64 = emu.cfg.is_x64();
    let (size, offset, flags) = if is64 {
        (
            Context64::INTEGER_CONTROL_SIZE,
            Context64::FLAGS_OFFSET,
            CONTEXT_CONTROL | CONTEXT_INTEGER,
        )
    } else {
        (
            Context32::INTEGER_CONTROL_SIZE,
            Context32::FLAGS_OFFSET,
            WOW64_CONTEXT_CONTROL | WOW64_CONTEXT_INTEGER,
        )
    };
    if out == 0 || !emu.maps.validate_write_range(out, size) {
        emu.exception(ExceptionType::BadAddressDereferencing);
        return;
    }
    let mut regs = *emu.regs();
    // API gateways consume the return address before dispatch. On x86 the
    // caller also expects the single stdcall argument to have been removed.
    regs.rip = emu.gateway_return;
    if !is64 {
        regs.rsp = u64::from((regs.get_esp() as u32).wrapping_add(4));
    }
    let eflags = emu.flags().dump();
    emu.maps.write_dword(out + offset, flags);
    if is64 {
        Context64::new(&regs).save_selected(out, &mut emu.maps, flags, eflags);
    } else {
        Context32::new(&regs).save_selected(out, &mut emu.maps, flags, eflags);
    }
    log_red!(emu, "RtlCaptureContext context={out:#x}");
    call.cleanup_args(emu, 1);
}

pub(crate) fn tls(emu: &mut Emu, api: &str) {
    let call = WinApi(Stdcall);
    let name = api.strip_prefix("Rtl").unwrap_or(api);
    let is64 = emu.cfg.is_x64();
    if name == "TlsAlloc" {
        let index = emu
            .windows_tls_slots
            .iter()
            .position(|used| !used)
            .unwrap_or(emu.windows_tls_slots.len());
        if index >= TLS_MAXIMUM_AVAILABLE {
            emu.set_last_error(ERROR_NO_MORE_ITEMS);
            call.set_return(emu, u64::from(TLS_OUT_OF_INDEXES));
            return;
        }
        if index == emu.windows_tls_slots.len() {
            emu.windows_tls_slots.push(true);
        } else {
            emu.windows_tls_slots[index] = true;
        }
        log_red!(emu, "{api} slot={index}");
        call.set_return(emu, index as u64);
        return;
    }
    let index = api_try!(emu, call.arg_u32(emu, 0)) as usize;
    let value = if name == "TlsSetValue" {
        Some(api_try!(emu, call.arg(emu, 1)))
    } else {
        None
    };
    let allocated = emu.windows_tls_slots.get(index) == Some(&true);
    let result = if !allocated {
        if name != "TlsGetValue2" {
            emu.set_last_error(ERROR_INVALID_PARAMETER);
        }
        0
    } else if let Some(value) = value {
        if is64 {
            let slots = emu.tls64_mut();
            if slots.len() <= index {
                slots.resize(index + 1, 0);
            }
            slots[index] = value;
        } else {
            let slots = emu.tls32_mut();
            if slots.len() <= index {
                slots.resize(index + 1, 0);
            }
            slots[index] = value as u32;
        }
        1
    } else if name == "TlsFree" {
        emu.windows_tls_slots[index] = false;
        for thread in &mut emu.threads {
            if let ArchThreadState::X86 { tls32, tls64, .. } = &mut thread.arch {
                if let Some(v) = tls32.get_mut(index) {
                    *v = 0;
                }
                if let Some(v) = tls64.get_mut(index) {
                    *v = 0;
                }
            }
        }
        1
    } else {
        if name != "TlsGetValue2" {
            emu.set_last_error(ERROR_SUCCESS);
        }
        if is64 {
            emu.tls64().get(index).copied().unwrap_or(0)
        } else {
            emu.tls32().get(index).copied().map_or(0, u64::from)
        }
    };
    log_red!(emu, "{api} slot={index} result={result:#x}");
    call.set_return(emu, result);
    call.cleanup_args(emu, if value.is_some() { 2 } else { 1 });
}

pub(crate) fn vectored(emu: &mut Emu, api: &str, add: bool) {
    let call = WinApi(Stdcall);
    let first = api_try!(emu, call.arg(emu, 0));
    let result = if add {
        let callback = api_try!(emu, call.arg(emu, 1));
        emu.register_veh(first != 0, callback)
    } else {
        u64::from(emu.unregister_veh(first))
    };
    log_red!(emu, "{api} result={result:#x}");
    call.set_return(emu, result);
    call.cleanup_args(emu, if add { 2 } else { 1 });
}

pub fn TlsAlloc(emu: &mut Emu) {
    tls(emu, "TlsAlloc");
}

pub fn TlsGetValue(emu: &mut Emu) {
    tls(emu, "TlsGetValue");
}

pub fn TlsSetValue(emu: &mut Emu) {
    tls(emu, "TlsSetValue");
}

pub fn TlsFree(emu: &mut Emu) {
    tls(emu, "TlsFree");
}

pub fn AddVectoredExceptionHandler(emu: &mut Emu) {
    vectored(emu, "AddVectoredExceptionHandler", true);
}

pub fn RemoveVectoredExceptionHandler(emu: &mut Emu) {
    vectored(emu, "RemoveVectoredExceptionHandler", false);
}

pub fn OpenThreadToken(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let [thread, access, open_as_self, output] = api_try!(emu, call.args(emu));
    log_red!(
        emu,
        "OpenThreadToken thread={thread:#x} access={access:#x} open_as_self={open_as_self} token_handle={output:#x}"
    );
    // Guest threads have no impersonation token.
    let error = if thread_index(emu, thread).is_none() {
        ERROR_INVALID_HANDLE
    } else {
        ERROR_NO_TOKEN
    };
    emu.set_last_error(error);
    call.set_return(emu, 0);
    call.cleanup_args(emu, 4);
}

pub(crate) fn create_remote(emu: &mut Emu, library: &str, api: &str) {
    let call = WinApi(Stdcall);
    let process = api_try!(emu, call.arg(emu, 0));
    let attributes = api_try!(emu, call.arg(emu, 1));
    let size = api_try!(emu, call.arg(emu, 2));
    let start = api_try!(emu, call.arg(emu, 3));
    let parameter = api_try!(emu, call.arg(emu, 4));
    let flags = api_try!(emu, call.arg_u32(emu, 5));
    let extended = api.ends_with("Ex");
    let attribute_list = if extended {
        api_try!(emu, call.arg(emu, 6))
    } else {
        0
    };
    let thread_id = api_try!(emu, call.arg(emu, 6 + usize::from(extended)));
    log_red!(
        emu,
        "{library}!{api} process={process:#x} stack_size={size} start={start:#x} parameter={parameter:#x} flags={flags:#x} thread_id={thread_id:#x}"
    );
    if attributes != 0 || attribute_list != 0 {
        policy::unsupported(emu, library, api, emu.pc());
        return;
    }
    let rights = PROCESS_CREATE_THREAD
        | PROCESS_QUERY_INFORMATION
        | PROCESS_VM_OPERATION
        | PROCESS_VM_WRITE
        | PROCESS_VM_READ;
    let result = match process::require_memory_access(emu, process, rights) {
        Ok(()) => create(emu, [size, start, parameter, u64::from(flags), thread_id]),
        Err(STATUS_INVALID_HANDLE) => Err(ERROR_INVALID_HANDLE),
        Err(_) => Err(ERROR_ACCESS_DENIED),
    };
    finish(emu, result, 0, 7 + usize::from(extended));
}
