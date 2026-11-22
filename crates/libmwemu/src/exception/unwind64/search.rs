use super::{
    DispatcherContext, FRAME_LIMIT, FrameResult, RSP, WORD, copy_context, load, walk_frame,
};
use crate::context::context64::Context64;
use crate::emu::Emu;
use crate::exception::handlers::x64_callback_search_origin;
use crate::exception::types::ExceptionRecord;
use crate::winapi::policy::ApiStopReason;
use crate::windows::constants::exceptions::{EXCEPTION_NESTED_CALL, UNW_FLAG_EHANDLER};
use crate::windows::structures::TEB64;

fn stack_bounds(emu: &Emu) -> Option<(u64, u64)> {
    emu.windows_teb()
        .and_then(|teb| {
            let low = emu
                .maps
                .read_qword(teb.checked_add(TEB64::NT_TIB_STACK_LIMIT_OFFSET)?)?;
            let high = emu
                .maps
                .read_qword(teb.checked_add(TEB64::NT_TIB_STACK_BASE_OFFSET)?)?;
            (low < high).then_some((low, high))
        })
        .or_else(|| {
            let map = emu.maps.get_mem_by_addr(emu.sp().checked_sub(1)?)?;
            Some((map.get_base(), map.get_bottom()))
        })
}

fn set_nested(emu: &mut Emu, record: u64, active: bool) -> FrameResult<()> {
    let address = record + ExceptionRecord::FLAGS_OFFSET;
    let flags = emu
        .maps
        .read_dword(address)
        .ok_or(ApiStopReason::InvalidMemory)?;
    let flags = if active {
        flags | EXCEPTION_NESTED_CALL
    } else {
        flags & !EXCEPTION_NESTED_CALL
    };
    emu.maps
        .write_dword(address, flags)
        .then_some(())
        .ok_or(ApiStopReason::InvalidMemory)
}

/// The guest receives a writable copy of the cursor. Its original CONTEXT stays separate.
pub(crate) fn next_exception_handler(
    emu: &mut Emu,
    cursor: u64,
    working: u64,
    (dispatcher, history): (u64, u64),
    record: u64,
    visited: &mut usize,
    nested: &mut u64,
) -> FrameResult<Option<(u64, u64)>> {
    let (low, high) = stack_bounds(emu).ok_or(ApiStopReason::InvalidFrame)?;
    let mut interrupted = *visited == 0;
    while *visited < FRAME_LIMIT {
        *visited += 1;
        let pc = load(&emu.maps, cursor + Context64::RIP_OFFSET)?;
        if pc == 0 && !interrupted {
            return Ok(None);
        }

        let sp = load(&emu.maps, cursor + Context64::RSP_OFFSET)?;
        if sp == high && !interrupted {
            return Ok(None);
        }
        if let Some((origin, establisher)) = x64_callback_search_origin(emu, pc) {
            if let Some(boundary) = establisher {
                *nested = (*nested).max(boundary);
                set_nested(emu, record, true)?;
            }
            copy_context(&mut emu.maps, origin, cursor)?;
            interrupted = true;
            continue;
        }
        // The leaf rule reads the saved return address from the stack even if the faulting PC is unmapped.
        interrupted = false;
        if !sp.is_multiple_of(WORD) || !(low..high).contains(&sp) {
            return Err(ApiStopReason::InvalidFrame);
        }
        let step = walk_frame(emu, cursor, pc, UNW_FLAG_EHANDLER)?;
        let establisher = step.frame.establisher;
        let caller_sp = step.frame.registers[RSP];
        if !establisher.is_multiple_of(WORD)
            || !(sp..high).contains(&establisher)
            || caller_sp <= sp
            || caller_sp > high
        {
            return Err(ApiStopReason::InvalidFrame);
        }
        if *nested != 0 && establisher > *nested {
            *nested = 0;
            set_nested(emu, record, false)?;
        }
        let data = step.frame.handler_data.unwrap_or(0);
        step.frame.commit(&mut emu.maps)?;
        if step.handler == 0 {
            continue;
        }

        copy_context(&mut emu.maps, cursor, working)?;
        DispatcherContext {
            pc,
            image: step.image,
            entry: step.entry,
            establisher,
            target: 0,
            context: working,
            handler: step.handler,
            data,
            history,
        }
        .write(&mut emu.maps, dispatcher)?;
        return Ok(Some((step.handler, establisher)));
    }
    Err(ApiStopReason::InvalidFrame)
}
