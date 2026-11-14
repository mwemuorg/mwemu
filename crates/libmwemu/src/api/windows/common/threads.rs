//! Windows thread creation and integer/control context operations.
use crate::context::{context32::Context32, context64::Context64};
use crate::emu::Emu;
use crate::maps::mem64::Permission;
use crate::maps::virtual_memory::{ALLOCATION_GRANULARITY, PAGE_SIZE, align_up};
use crate::threading::context::{ArchThreadState, ThreadContext};
use crate::winapi::common::memory::{self, Call};
use crate::winapi::common::trace;
use crate::winapi::helper;
use crate::windows::constants::errors::*;
use crate::windows::constants::exceptions::*;
use crate::windows::constants::memory::*;
use crate::windows::constants::process::*;

const DEFAULT_STACK_RESERVE: u64 = 1024 * 1024;
const X64_SHADOW_SPACE: u64 = 4 * 8;
const FIRST_THREAD_ID: u64 = 0x1000;

fn thread_index(emu: &Emu, handle: u64) -> Option<usize> {
    if handle == NT_CURRENT_THREAD || handle == NT_CURRENT_THREAD as u32 as u64 {
        return Some(emu.current_thread_id);
    }
    let uri = helper::handler_get_uri(handle);
    let id = uri
        .strip_prefix("tid://0x")
        .and_then(|id| u64::from_str_radix(id, 16).ok())?;
    emu.threads.iter().position(|thread| thread.id == id)
}

fn finish(emu: &mut Emu, result: Result<u64, u64>, failure: u64, args: usize) {
    let value = match result {
        Ok(value) => value,
        Err(error) => {
            emu.set_last_error(error);
            failure
        }
    };
    Call::Api.finish(emu, value, args);
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

fn create(emu: &mut Emu) -> Result<u64, u64> {
    let requested = Call::Api.arg(emu, 1);
    let start = Call::Api.arg(emu, 2);
    let parameter = Call::Api.arg(emu, 3);
    let flags = Call::Api.arg(emu, 4);
    let id_ptr = Call::Api.arg(emu, 5);
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
    thread.suspended = flags & CREATE_SUSPENDED != 0;
    if id_ptr != 0 && !emu.maps.write_dword(id_ptr, id as u32) {
        let _ = emu.maps.release_memory(base);
        return Err(ERROR_NOACCESS);
    }
    let handle = helper::handler_create(&format!("tid://0x{id:x}"));
    thread.handle = handle;
    emu.threads.push(thread);
    Ok(handle)
}

pub(crate) fn create_thread(emu: &mut Emu) {
    let result = create(emu);
    finish(emu, result, 0, 6);
}

pub(crate) fn resume_thread(emu: &mut Emu) {
    let result = thread_index(emu, Call::Api.arg(emu, 0))
        .ok_or(ERROR_INVALID_HANDLE)
        .map(|index| {
            let thread = &mut emu.threads[index];
            let previous = thread.suspended as u64;
            thread.suspended = false;
            previous
        });
    finish(emu, result, u32::MAX as u64, 1);
}

fn context(emu: &mut Emu, set: bool) -> Result<u64, u64> {
    let index = thread_index(emu, Call::Api.arg(emu, 0)).ok_or(ERROR_INVALID_HANDLE)?;
    let address = Call::Api.arg(emu, 1);
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
            WOW64_CONTEXT_CONTROL | WOW64_CONTEXT_INTEGER,
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
    let result = context(emu, true);
    finish(emu, result, 0, 2);
}

pub(crate) fn get_thread_context(emu: &mut Emu) {
    let result = context(emu, false);
    finish(emu, result, 0, 2);
}

pub(crate) fn exit_thread(emu: &mut Emu) {
    let code = Call::Api.arg(emu, 0) as u32;
    Call::Api.finish(emu, code as u64, 1);
    finish_thread(emu, code);
}

pub(crate) fn finish_thread(emu: &mut Emu, code: u32) {
    emu.current_thread_mut().exit_value = Some(code as u64);
    if !emu.cfg.enable_threading || emu.threads.iter().all(|thread| thread.exit_value.is_some()) {
        emu.stop();
    }
}

pub(crate) fn dispatch(api: &str, library: &str, emu: &mut Emu) -> bool {
    let (parameters, handler): (&str, fn(&mut Emu)) = match api {
        "CreateThread" => (
            "attributes stack_size start parameter flags thread_id",
            create_thread,
        ),
        "ResumeThread" => ("thread", resume_thread),
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
