/*
    Software exceptions (kernel32!RaiseException) on x64.

    Windows builds an EXCEPTION_RECORD and a CONTEXT, wraps them in an
    EXCEPTION_POINTERS and calls every vectored handler in order:

        LONG handler(EXCEPTION_POINTERS *info);

    The handlers are ordinary functions (they use the stack, call helpers and
    may edit the CONTEXT), so each one runs as a nested call64 instead of the
    single-`ret` model used for CPU faults. EXCEPTION_CONTINUE_EXECUTION
    resumes from the CONTEXT; EXCEPTION_CONTINUE_SEARCH tries the next one.

    Anti-debug code relies on this: DBG_PRINTEXCEPTION_C (OutputDebugString)
    only reaches the VEH when no debugger swallows it.
*/

use crate::context::context64::Context64;
use crate::emu;
use crate::maps::mem64::Permission;
use crate::windows::constants::exceptions::*;

const BLOCK_SIZE: u64 = 0x1000;
const RECORD_OFF: u64 = 0x10;
const CONTEXT_OFF: u64 = 0x100;

/// Dispatch a raised exception to the VEH list.
/// Returns true when a handler answered EXCEPTION_CONTINUE_EXECUTION and the
/// registers were resumed from its CONTEXT; false leaves the registers as they
/// were before the dispatch.
pub fn raise64(emu: &mut emu::Emu, code: u32, flags: u32, params: &[u64]) -> bool {
    let handlers = emu.veh_list().to_vec();
    if handlers.is_empty() {
        return false;
    }

    let Some(block) = emu.maps.alloc(BLOCK_SIZE) else {
        log::error!("out of memory allocating the RaiseException context");
        return false;
    };
    let map_name = format!("raise_ctx_{:x}", block);
    if let Err(e) = emu
        .maps
        .create_map(&map_name, block, BLOCK_SIZE, Permission::READ_WRITE)
    {
        log::error!("cannot map the RaiseException context: {}", e);
        return false;
    }

    let saved = *emu.regs();
    let record = block + RECORD_OFF;
    let context = block + CONTEXT_OFF;
    write_record(emu, record, code, flags, saved.rip, params);
    write_context(emu, context);
    emu.maps.write_qword(block, record);
    emu.maps.write_qword(block + 8, context);

    let resumed = call_handlers(emu, &handlers, block);
    if resumed {
        let mut ctx = Context64::new(&saved);
        ctx.load(context, &mut emu.maps);
        ctx.sync(emu.regs_mut());
    } else {
        *emu.regs_mut() = saved;
    }

    emu.maps.free(&map_name);
    resumed
}

/// Call each VEH with the EXCEPTION_POINTERS until one continues execution.
fn call_handlers(emu: &mut emu::Emu, handlers: &[u64], pointers: u64) -> bool {
    for &handler in handlers {
        match emu.call64(handler, &[pointers]) {
            Ok(disposition) => {
                if disposition as u32 == EXCEPTION_CONTINUE_EXECUTION32 {
                    return true;
                }
            }
            Err(e) => {
                log::warn!("VEH 0x{:x} failed: {}", handler, e.message);
                return false;
            }
        }
    }
    false
}

/// EXCEPTION_RECORD64: code, flags, nested record, address, parameter count
/// and up to 15 ExceptionInformation entries.
fn write_record(
    emu: &mut emu::Emu,
    addr: u64,
    code: u32,
    flags: u32,
    exception_address: u64,
    params: &[u64],
) {
    let count = params.len().min(EXCEPTION_MAXIMUM_PARAMETERS);
    emu.maps.write_dword(addr, code);
    emu.maps.write_dword(addr + 4, flags);
    emu.maps.write_qword(addr + 8, 0);
    emu.maps.write_qword(addr + 0x10, exception_address);
    emu.maps.write_dword(addr + 0x18, count as u32);
    for (i, value) in params.iter().take(count).enumerate() {
        emu.maps.write_qword(addr + 0x20 + i as u64 * 8, *value);
    }
}

/// CONTEXT captured at the RaiseException return address, so resuming it
/// unmodified returns to the caller.
fn write_context(emu: &mut emu::Emu, addr: u64) {
    let ctx = Context64::new(emu.regs());
    ctx.save(addr, &mut emu.maps);
    let eflags = emu.flags().dump();
    emu.maps
        .write_dword(addr + Context64::FLAGS_OFFSET, CONTEXT_FULL);
    emu.maps
        .write_dword(addr + Context64::EFLAGS_OFFSET, eflags);
}
