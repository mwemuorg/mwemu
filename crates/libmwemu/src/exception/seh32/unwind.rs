use super::{
    DISPATCHER_OFFSET, ESTABLISHER_FRAME, Frame, REGISTRATION_HANDLER, UNWIND_MAGIC, guard_owner,
};
use crate::emu::Emu;
use crate::exception::types::{ExceptionRecord, MemoryAccess};
use crate::maps::mem64::Permission;
use crate::windows::constants::errors::*;
use crate::windows::constants::exceptions::*;
use crate::windows::structures::TEB;

const TARGET_FRAME: u64 = 16;
const RECORD_POINTER: u64 = 20;
const REGISTRATION_SIZE: usize = 2 * size_of::<u32>();

pub(super) fn record_pointer(frame: Frame, emu: &Emu) -> Option<u64> {
    emu.maps
        .read_dword(frame.base + RECORD_POINTER)
        .map(u64::from)
}

fn stack_registration(emu: &Emu, registration: u32) -> bool {
    let address = registration as u64;
    if !registration.is_multiple_of(align_of::<u32>() as u32)
        || !emu
            .maps
            .can_access_range(address, REGISTRATION_SIZE, Permission::READ)
    {
        return false;
    }
    let bounds = emu
        .windows_teb()
        .and_then(|teb| {
            let limit = emu.maps.read_dword(teb + TEB::NT_TIB_STACK_LIMIT_OFFSET)? as u64;
            let base = emu.maps.read_dword(teb + TEB::NT_TIB_STACK_BASE_OFFSET)? as u64;
            (limit < base).then_some((limit, base))
        })
        .or_else(|| {
            let map = emu.maps.get_mem_by_addr(emu.sp().checked_sub(1)?)?;
            Some((map.get_base(), map.get_bottom()))
        });
    bounds.is_some_and(|(low, high)| address >= low && address + REGISTRATION_SIZE as u64 <= high)
}

fn invalid(frame: Frame, emu: &mut Emu, code: u32) {
    let nested = record_pointer(frame, emu)
        .filter(|p| *p != frame.record())
        .unwrap_or(0);
    frame.restore_context(emu);
    let fault = ExceptionRecord {
        code,
        flags: EXCEPTION_NONCONTINUABLE,
        address: emu.pc(),
        nested,
        information: Vec::new(),
    };
    frame.release(emu);
    // An invalid chain cannot safely receive its own failure.
    if code == STATUS_BAD_STACK {
        emu.dispatch_unhandled_exception(&fault);
    } else {
        emu.dispatch_vectored_exception(&fault, 0);
    }
}

fn next_handler(frame: Frame, emu: &mut Emu, target: u32, record: u64) {
    let mut registration = emu.seh() as u32;
    while registration != 0 && registration != target && registration != EXCEPTION_CHAIN_END {
        let Some(owner) = guard_owner(emu, registration) else {
            break;
        };
        let Some(mut next) = emu.maps.read_dword(registration as u64) else {
            invalid(frame, emu, STATUS_BAD_STACK);
            return;
        };
        if emu.maps.read_dword(owner.base) == Some(UNWIND_MAGIC) {
            let Some(outer) = emu
                .maps
                .read_dword(owner.base + ESTABLISHER_FRAME)
                .and_then(|outer| emu.maps.read_dword(outer as u64))
            else {
                invalid(frame, emu, STATUS_BAD_STACK);
                return;
            };
            next = outer;
        }
        if next != EXCEPTION_CHAIN_END && next <= registration {
            invalid(frame, emu, STATUS_BAD_STACK);
            return;
        }
        registration = next;
        emu.set_seh(next as u64);
    }
    if registration == target && target != 0 {
        frame.release(emu);
        return;
    }
    if registration == EXCEPTION_CHAIN_END || registration == 0 {
        if target != 0 {
            invalid(frame, emu, STATUS_INVALID_UNWIND_TARGET);
        } else if let Some(record) = ExceptionRecord::load(&emu.maps, record, false) {
            frame.release(emu);
            emu.dispatch_unhandled_exception(&record);
        } else {
            frame.stop(emu, "unreadable exit unwind record");
        }
        return;
    }
    if target != 0 && registration > target {
        invalid(frame, emu, STATUS_INVALID_UNWIND_TARGET);
        return;
    }
    if !stack_registration(emu, registration) {
        invalid(frame, emu, STATUS_BAD_STACK);
        return;
    }
    let handler_slot = registration as u64 + REGISTRATION_HANDLER;
    let Some(handler) = emu.maps.read_dword(handler_slot) else {
        invalid(frame, emu, STATUS_BAD_STACK);
        return;
    };
    if !frame.push_arguments(emu, registration, record) {
        invalid(frame, emu, STATUS_BAD_STACK);
        return;
    }
    emu.set_eip(handler as u64, false);
}

pub(super) fn finish(frame: Frame, emu: &mut Emu) {
    let disposition = emu.regs().get_eax() as u32;
    let registration = emu.maps.read_dword(frame.base + ESTABLISHER_FRAME);
    let target = emu.maps.read_dword(frame.base + TARGET_FRAME);
    let record = record_pointer(frame, emu);
    frame.restore_context(emu);
    let (Some(registration), Some(target), Some(record)) = (registration, target, record) else {
        frame.stop(emu, "unreadable unwind state");
        return;
    };
    let unwind_from = if disposition == ExceptionDisposition::ContinueSearch as u32 {
        registration
    } else if disposition == ExceptionDisposition::CollidedUnwind as u32 {
        let new_frame = emu.maps.read_dword(frame.base + DISPATCHER_OFFSET);
        match new_frame {
            Some(new_frame)
                if new_frame >= registration
                    && stack_registration(emu, new_frame)
                    && (target == 0 || new_frame < target) =>
            {
                new_frame
            }
            _ => {
                invalid(frame, emu, STATUS_BAD_STACK);
                return;
            }
        }
    } else {
        invalid(frame, emu, STATUS_INVALID_DISPOSITION);
        return;
    };
    let Some(next) = emu.maps.read_dword(unwind_from as u64) else {
        invalid(frame, emu, STATUS_BAD_STACK);
        return;
    };
    if next != EXCEPTION_CHAIN_END && next <= unwind_from {
        invalid(frame, emu, STATUS_BAD_STACK);
        return;
    }
    emu.set_seh(next as u64);
    next_handler(frame, emu, target, record);
}

pub(crate) fn start(emu: &mut Emu, target: u32, supplied_record: u32) {
    let flags = EXCEPTION_UNWINDING
        | if target == 0 {
            EXCEPTION_EXIT_UNWIND
        } else {
            0
        };
    let record = if supplied_record == 0 {
        ExceptionRecord {
            code: STATUS_UNWIND,
            flags,
            address: emu.pc(),
            nested: 0,
            information: Vec::new(),
        }
    } else {
        let pointer = supplied_record as u64;
        let Some(mut record) = ExceptionRecord::load(&emu.maps, pointer, false) else {
            emu.dispatch_vectored_exception(
                &ExceptionRecord::access_violation(emu.pc(), pointer, MemoryAccess::Read),
                0,
            );
            return;
        };
        record.flags |= flags;
        if !emu
            .maps
            .write_dword(pointer + ExceptionRecord::FLAGS_OFFSET, record.flags)
        {
            emu.dispatch_vectored_exception(
                &ExceptionRecord::access_violation(
                    emu.pc(),
                    pointer + ExceptionRecord::FLAGS_OFFSET,
                    MemoryAccess::Write,
                ),
                0,
            );
            return;
        }
        record
    };
    let Some(frame) = Frame::allocate(emu, &record) else {
        log::error!("x86 RtlUnwind: cannot allocate the unwind context");
        emu.stop();
        return;
    };
    let pointer = if supplied_record == 0 {
        frame.record()
    } else {
        supplied_record as u64
    };
    emu.maps.write_dword(frame.base, UNWIND_MAGIC);
    emu.maps.write_dword(frame.base + TARGET_FRAME, target);
    emu.maps
        .write_dword(frame.base + RECORD_POINTER, pointer as u32);
    next_handler(frame, emu, target, pointer);
}
