//! Dispatch frames for CPU exceptions, VEH and unhandled exception filters.

use crate::context::context32::Context32;
use crate::emu::Emu;
use crate::exception::seh32;
use crate::exception::types::{ExceptionRecord, ExceptionType};
use crate::exception::unwind64::{self, DispatcherContext, copy_context, next_exception_handler};
use crate::maps::mem64::Permission;
use crate::windows::constants::errors::{
    STATUS_INVALID_DISPOSITION, STATUS_NONCONTINUABLE_EXCEPTION,
};
use crate::windows::constants::exceptions::*;

// Private dispatcher-frame layout.
const FRAME_SIZE: u64 = 0x1000;
const AMD64_FRAME_SIZE: u64 = 0x2000;
const SEARCH_MAGIC: u32 = u32::from_le_bytes(*b"SE64");
const VISITED_OFFSET: u64 = 24;
const WORKING_OFFSET: u64 = 0x900;
const CURSOR_OFFSET: u64 = 0x1000;
const ORIGIN_OFFSET: u64 = 0x1500;
const HISTORY_OFFSET: u64 = 0x1a00;
const NESTED_FRAME_OFFSET: u64 = 0x30;
const ACTIVE_ESTABLISHER_OFFSET: u64 = 0x38;
// API dispatch keeps caller-owned exception records and contexts alive through guest callbacks.
const EXTERNAL_RECORD: u64 = 0x700;
const EXTERNAL_CONTEXT: u64 = 0x708;
const ACTIVE_FRAME_LIMIT: usize = 4096;
const FRAME_MAGIC: u32 = u32::from_le_bytes(*b"EXCP");
const PREVIOUS_CONTEXT: u64 = 8;
const HANDLER_KIND: u64 = 16;
const VEH_INDEX: u64 = 20;
const POINTERS_OFFSET: u64 = 0x20;
const RECORD_OFFSET: u64 = 0x40;
const CONTEXT_OFFSET: u64 = 0x100;
const DISPATCHER_OFFSET: u64 = 0x800;
const RETURN_COOKIE_OFFSET: u64 = 0xf00;

// Windows x64 callback stack layout.
const SHADOW_SPACE: u64 = 32;
const STACK_ALIGNMENT: u64 = 16;

#[derive(Clone, Copy, PartialEq)]
pub enum HandlerKind {
    Seh,
    Veh,
    Uef,
}

impl HandlerKind {
    pub fn as_u32(self) -> u32 {
        match self {
            Self::Seh => 1,
            Self::Veh => 2,
            Self::Uef => 3,
        }
    }
}

#[derive(Clone, Copy)]
struct Frame {
    base: u64,
}

impl Frame {
    fn active(emu: &Emu) -> Option<Self> {
        Self::at(emu, emu.eh_ctx())
    }

    fn at(emu: &Emu, context: u64) -> Option<Self> {
        let base = context.checked_sub(CONTEXT_OFFSET)?;
        matches!(emu.maps.read_dword(base), Some(FRAME_MAGIC | SEARCH_MAGIC))
            .then_some(Self { base })
    }

    fn is_search(self, emu: &Emu) -> bool {
        emu.maps.read_dword(self.base) == Some(SEARCH_MAGIC)
    }

    fn origin(self) -> u64 {
        self.base + ORIGIN_OFFSET
    }

    fn cursor(self) -> u64 {
        self.base + CURSOR_OFFSET
    }

    fn record_address(self, emu: &Emu) -> u64 {
        match emu.maps.read_qword(self.base + EXTERNAL_RECORD) {
            Some(address) if address != 0 => address,
            _ => self.record(),
        }
    }

    fn exception_context(self, emu: &Emu) -> u64 {
        match emu.maps.read_qword(self.base + EXTERNAL_CONTEXT) {
            Some(address) if address != 0 => address,
            _ => self.context(),
        }
    }

    fn next_vectored(self, emu: &mut Emu, index: usize) {
        if let Some(&handler) = emu.veh_list().get(index) {
            if !emu.maps.can_access_range(handler, 1, Permission::EXECUTE)
                || unwind64::restore_context(emu, self.origin()).is_err()
                || !self.prepare_call(emu, HandlerKind::Veh)
            {
                self.release(emu);
                emu.stop();
                return;
            }
            emu.maps
                .write_dword(self.base + HANDLER_KIND, HandlerKind::Veh.as_u32());
            emu.maps.write_dword(self.base + VEH_INDEX, index as u32);
            emu.flags_mut().f_tf = false;
            emu.set_pc(handler);
            emu.force_reload = true;
        } else {
            emu.maps
                .write_dword(self.base + HANDLER_KIND, HandlerKind::Seh.as_u32());
            let context = self.exception_context(emu);
            if copy_context(&mut emu.maps, context, self.cursor()).is_err() {
                self.release(emu);
                emu.stop();
                return;
            }
            self.search_next(emu);
        }
    }

    fn has_caller_context(self, emu: &Emu) -> bool {
        self.exception_context(emu) != self.context()
    }

    fn return_to_api_caller(self, emu: &mut Emu, exception_handled: bool) {
        // RtlDispatchException reports a disposition. Its caller owns the CONTEXT
        // and decides whether to resume from it.
        let restored = unwind64::restore_context(emu, self.origin()).is_ok();
        self.release(emu);
        if restored {
            emu.regs_mut().rax = u64::from(exception_handled);
        } else {
            emu.stop();
        }
    }

    fn search_next(self, emu: &mut Emu) {
        let mut visited = emu
            .maps
            .read_dword(self.base + VISITED_OFFSET)
            .unwrap_or(u32::MAX) as usize;
        let mut nested = emu
            .maps
            .read_qword(self.base + NESTED_FRAME_OFFSET)
            .unwrap_or(0);
        let record_address = self.record_address(emu);
        let next = next_exception_handler(
            emu,
            self.cursor(),
            self.base + WORKING_OFFSET,
            (self.base + DISPATCHER_OFFSET, self.base + HISTORY_OFFSET),
            record_address,
            &mut visited,
            &mut nested,
        );
        emu.maps
            .write_dword(self.base + VISITED_OFFSET, visited as u32);
        emu.maps
            .write_qword(self.base + NESTED_FRAME_OFFSET, nested);
        match next {
            Ok(Some((handler, establisher))) => {
                if emu
                    .maps
                    .get_mem_by_addr(handler)
                    .is_none_or(|mem| mem.read_code(handler, 1).is_empty())
                    || unwind64::restore_context(emu, self.origin()).is_err()
                    || !self.prepare_call(emu, HandlerKind::Seh)
                {
                    self.release(emu);
                    log::error!("cannot enter x64 exception handler at {handler:#x}");
                    emu.stop();
                    return;
                }
                emu.maps
                    .write_qword(self.base + ACTIVE_ESTABLISHER_OFFSET, establisher);
                emu.regs_mut().rdx = establisher;
                emu.flags_mut().f_tf = false;
                if emu.export_indexes.resolve_address_module("ntdll", handler)
                    == Some("__C_specific_handler")
                {
                    emu.set_rip_with_check(handler, false);
                } else {
                    emu.set_pc(handler);
                    emu.force_reload = true;
                }
            }
            result => {
                if result.is_err()
                    && let Some(flags) = emu
                        .maps
                        .read_dword(record_address + ExceptionRecord::FLAGS_OFFSET)
                {
                    emu.maps.write_dword(
                        record_address + ExceptionRecord::FLAGS_OFFSET,
                        flags | EXCEPTION_STACK_INVALID,
                    );
                }
                if self.has_caller_context(emu) {
                    self.return_to_api_caller(emu, false);
                    return;
                }
                let record = ExceptionRecord::load(&emu.maps, record_address, true);
                let restored = unwind64::restore_context(emu, self.origin()).is_ok();
                self.release(emu);
                if let Some(mut record) = record
                    && restored
                {
                    if let Err(reason) = result {
                        log::debug!("x64 exception search stopped: {reason:?}");
                        record.flags |= EXCEPTION_STACK_INVALID;
                    }
                    emu.dispatch_unhandled_exception(&record);
                } else {
                    log::error!("invalid x64 exception dispatch records");
                    emu.stop();
                }
            }
        }
    }

    fn finish_search(self, emu: &mut Emu, disposition: u32) {
        let record = self.record_address(emu);
        let Some(mut flags) = emu.maps.read_dword(record + ExceptionRecord::FLAGS_OFFSET) else {
            emu.stop();
            return;
        };
        if emu.maps.read_dword(self.base + HANDLER_KIND) == Some(HandlerKind::Veh.as_u32()) {
            if disposition == EXCEPTION_CONTINUE_EXECUTION32 {
                self.continue_exception(emu, flags);
            } else {
                let index = emu.maps.read_dword(self.base + VEH_INDEX).unwrap_or(0) as usize;
                self.next_vectored(emu, index + 1);
            }
            return;
        }
        let nested_at = self.base + NESTED_FRAME_OFFSET;
        let boundary = emu.maps.read_qword(nested_at).unwrap_or(0);
        let current = emu
            .maps
            .read_qword(self.base + ACTIVE_ESTABLISHER_OFFSET)
            .unwrap_or(0);
        if boundary != 0 && current >= boundary {
            flags &= !EXCEPTION_NESTED_CALL;
            emu.maps
                .write_dword(record + ExceptionRecord::FLAGS_OFFSET, flags);
            emu.maps.write_qword(nested_at, 0);
        }
        if disposition == ExceptionDisposition::ContinueSearch as u32 {
            self.search_next(emu);
        } else if disposition == ExceptionDisposition::NestedException as u32 {
            let reported = emu
                .maps
                .read_qword(self.base + DISPATCHER_OFFSET + DispatcherContext::ESTABLISHER_FRAME)
                .unwrap_or(current);
            emu.maps.write_qword(nested_at, reported.max(current));
            emu.maps.write_dword(
                record + ExceptionRecord::FLAGS_OFFSET,
                flags | EXCEPTION_NESTED_CALL,
            );
            self.search_next(emu);
        } else if disposition == ExceptionDisposition::ContinueExecution as u32 {
            self.continue_exception(emu, flags);
        } else {
            self.raise_dispatch_status(emu, STATUS_INVALID_DISPOSITION);
        }
    }

    fn continue_exception(self, emu: &mut Emu, flags: u32) {
        if flags & EXCEPTION_NONCONTINUABLE != 0 {
            self.raise_dispatch_status(emu, STATUS_NONCONTINUABLE_EXCEPTION);
        } else if self.has_caller_context(emu) {
            self.return_to_api_caller(emu, true);
        } else {
            let resumed = unwind64::restore_context(emu, self.context()).is_ok();
            self.release(emu);
            if resumed {
                emu.force_reload = true;
            } else {
                log::error!("invalid x64 exception continuation context");
                emu.stop();
            }
        }
    }

    fn raise_dispatch_status(self, emu: &mut Emu, code: u32) {
        // The guest language handler has returned to the synthetic dispatcher.
        // Keep its record alive until a handler unwinds past this callback.
        let cookie = self.base + RETURN_COOKIE_OFFSET;
        emu.set_pc(cookie);
        emu.force_reload = true;
        let record = ExceptionRecord {
            code,
            flags: EXCEPTION_NONCONTINUABLE,
            address: cookie,
            nested: self.record_address(emu),
            information: Vec::new(),
        };
        emu.dispatch_vectored_exception(&record, 0);
    }

    fn context(self) -> u64 {
        self.base + CONTEXT_OFFSET
    }

    fn record(self) -> u64 {
        self.base + RECORD_OFFSET
    }

    fn map_name(self) -> String {
        format!("exception_{:x}", self.base)
    }

    fn allocate(
        emu: &mut Emu,
        record: &ExceptionRecord,
        kind: HandlerKind,
        veh_index: usize,
    ) -> Option<Self> {
        let is_64 = emu.cfg.is_x64();
        let size = if is_64 { AMD64_FRAME_SIZE } else { FRAME_SIZE };
        let base = emu.maps.alloc(size)?;
        if !is_64 && base.checked_add(FRAME_SIZE)? > u32::MAX as u64 {
            return None;
        }
        let frame = Self { base };
        emu.maps
            .create_map(&frame.map_name(), base, size, Permission::READ_WRITE)
            .ok()?;
        let previous = emu.eh_ctx();
        emu.maps.write_dword(base, FRAME_MAGIC);
        emu.maps.write_qword(base + PREVIOUS_CONTEXT, previous);
        emu.maps.write_dword(base + HANDLER_KIND, kind.as_u32());
        emu.maps.write_dword(base + VEH_INDEX, veh_index as u32);
        record.save(&mut emu.maps, frame.record(), is_64);
        if is_64 {
            let pc = emu.pc();
            if unwind64::capture_context(emu, frame.context(), pc).is_err()
                || copy_context(&mut emu.maps, frame.context(), frame.origin()).is_err()
            {
                emu.maps.free(&frame.map_name());
                return None;
            }
        } else {
            save_context(emu, frame.context());
        }
        let pointers = base + POINTERS_OFFSET;
        if is_64 {
            emu.maps.write_qword(pointers, frame.record());
            emu.maps
                .write_qword(pointers + size_of::<u64>() as u64, frame.context());
        } else {
            emu.maps.write_dword(pointers, frame.record() as u32);
            emu.maps
                .write_dword(pointers + size_of::<u32>() as u64, frame.context() as u32);
        }
        emu.set_eh_ctx(frame.context());
        Some(frame)
    }

    fn prepare_call(self, emu: &mut Emu, kind: HandlerKind) -> bool {
        let pointers = self.base + POINTERS_OFFSET;
        let cookie = self.base + RETURN_COOKIE_OFFSET;
        if !emu.cfg.is_x64() {
            return emu.stack_push32(pointers as u32) && emu.stack_push32(cookie as u32);
        }
        let original_rsp = emu.regs().rsp;
        let Some(stack) = original_rsp.checked_sub(SHADOW_SPACE + STACK_ALIGNMENT) else {
            return false;
        };
        let stack = (stack & !(STACK_ALIGNMENT - 1)) + size_of::<u64>() as u64;
        if !emu.maps.can_access_range(
            stack,
            (SHADOW_SPACE + size_of::<u64>() as u64) as usize,
            Permission::WRITE,
        ) || !emu.maps.write_qword(stack, cookie)
        {
            return false;
        }
        emu.regs_mut().rsp = stack;
        if kind == HandlerKind::Seh {
            let record = self.record_address(emu);
            let context = self.exception_context(emu);
            let regs = emu.regs_mut();
            regs.rcx = record;
            regs.rdx = original_rsp;
            regs.r8 = context;
            regs.r9 = self.base + DISPATCHER_OFFSET;
        } else {
            emu.regs_mut().rcx = pointers;
        }
        true
    }

    fn release(self, emu: &mut Emu) {
        let previous = emu
            .maps
            .read_qword(self.base + PREVIOUS_CONTEXT)
            .unwrap_or(0);
        emu.set_eh_ctx(previous);
        emu.maps.free(&self.map_name());
    }

    fn finish(self, emu: &mut Emu) {
        let disposition = emu.regs().get_eax() as u32;
        if self.is_search(emu) {
            self.finish_search(emu, disposition);
            return;
        }
        let kind = emu.maps.read_dword(self.base + HANDLER_KIND).unwrap_or(0);
        let next_veh = emu.maps.read_dword(self.base + VEH_INDEX).unwrap_or(0) as usize + 1;
        let record = ExceptionRecord::load(&emu.maps, self.record(), emu.cfg.is_x64());
        if !restore_context(emu, self.context()) {
            self.release(emu);
            emu.stop();
            return;
        }
        self.release(emu);
        let is_seh = kind == HandlerKind::Seh.as_u32();
        let resume = if is_seh {
            ExceptionDisposition::ContinueExecution as u32
        } else {
            EXCEPTION_CONTINUE_EXECUTION32
        };
        let search = if is_seh {
            ExceptionDisposition::ContinueSearch as u32
        } else {
            EXCEPTION_CONTINUE_SEARCH
        };
        if disposition == resume {
            return;
        }
        let Some(record) = record else {
            log::error!("invalid exception record returned by handler");
            emu.stop();
            return;
        };
        if disposition != search {
            log::error!("unsupported exception disposition 0x{:x}", disposition);
            emu.stop();
        } else if kind == HandlerKind::Veh.as_u32() {
            emu.dispatch_vectored_exception(&record, next_veh);
        } else if is_seh {
            emu.dispatch_unhandled_exception(&record);
        } else {
            log::error!(
                "unhandled exception 0x{:x} at 0x{:x}",
                record.code,
                record.address
            );
            emu.stop();
        }
    }
}

pub(crate) fn save_context(emu: &mut Emu, address: u64) {
    let flags = emu.flags().dump();
    Context32::new(emu.regs()).save(address as u32, &mut emu.maps);
    emu.maps.write_dword(
        address + Context32::FLAGS_OFFSET,
        WOW64_CONTEXT_CONTROL | WOW64_CONTEXT_INTEGER | WOW64_CONTEXT_DEBUG_REGISTERS,
    );
    emu.maps
        .write_dword(address + Context32::EFLAGS_OFFSET, flags);
}

pub(crate) fn restore_context(emu: &mut Emu, address: u64) -> bool {
    if emu.cfg.is_x64() {
        if unwind64::restore_context(emu, address).is_err() {
            return false;
        }
    } else {
        let mut context = Context32::new(emu.regs());
        let parts = emu
            .maps
            .read_dword(address + Context32::FLAGS_OFFSET)
            .unwrap_or(0);
        context.load_selected(address, &emu.maps, parts);
        context.sync(emu.regs_mut());
        if parts & WOW64_CONTEXT_CONTROL == WOW64_CONTEXT_CONTROL {
            let flags = emu
                .maps
                .read_dword(address + Context32::EFLAGS_OFFSET)
                .unwrap_or(0);
            emu.flags_mut().load(flags);
        }
    }
    emu.force_reload = true;
    true
}

/// Captured callback frames form a per-thread chain. An outer frame stays alive when a nested search returns.
fn callback_chain(emu: &Emu) -> Vec<Frame> {
    let mut frames = Vec::new();
    let mut context = emu.eh_ctx();
    while let Some(frame) = Frame::at(emu, context) {
        if frames.len() == ACTIVE_FRAME_LIMIT
            || frames
                .iter()
                .any(|previous: &Frame| previous.base == frame.base)
        {
            break;
        }
        frames.push(frame);
        context = emu
            .maps
            .read_qword(frame.base + PREVIOUS_CONTEXT)
            .unwrap_or(0);
    }
    frames
}

fn callback_at_cookie(emu: &Emu, cookie: u64) -> Option<Frame> {
    // Ordinary guest code cannot lie in a host-owned callback allocation.
    let base = cookie.checked_sub(RETURN_COOKIE_OFFSET)?;
    if emu
        .maps
        .get_mem_by_addr(cookie)
        .is_none_or(|mem| mem.get_base() != base || !mem.get_name().starts_with("exception_"))
    {
        return None;
    }
    callback_chain(emu)
        .into_iter()
        .find(|frame| frame.base == base)
}

pub(crate) fn x64_callback_origin(emu: &Emu, cookie: u64) -> Option<(u64, u64)> {
    let frame = callback_at_cookie(emu, cookie)?;
    let origin = if emu.maps.get_mem_by_addr(cookie)?.size() < AMD64_FRAME_SIZE as usize
        || frame.has_caller_context(emu)
    {
        // Older snapshots have just one context. RtlDispatchException stores its
        // interrupted context here and reserves origin() for the API caller.
        frame.context()
    } else {
        frame.origin()
    };
    Some((frame.context(), origin))
}

pub(crate) fn x64_callback_search_origin(emu: &Emu, cookie: u64) -> Option<(u64, Option<u64>)> {
    let (id, origin) = x64_callback_origin(emu, cookie)?;
    let frame = Frame::at(emu, id)?;
    let boundary = (frame.is_search(emu)
        && emu.maps.read_dword(frame.base + HANDLER_KIND) == Some(HandlerKind::Seh.as_u32()))
    .then(|| emu.maps.read_qword(frame.base + ACTIVE_ESTABLISHER_OFFSET))
    .flatten();
    Some((origin, boundary))
}

pub(crate) fn release_x64_callbacks_through(emu: &mut Emu, context: u64) -> bool {
    let frames = callback_chain(emu);
    let Some(last) = frames.iter().position(|frame| frame.context() == context) else {
        return false;
    };
    for frame in &frames[..=last] {
        frame.release(emu);
    }
    true
}

pub(crate) fn release_stopped_x64_callbacks(emu: &mut Emu) {
    let active = emu.current_thread_id;
    for index in 0..emu.threads.len() {
        emu.current_thread_id = index;
        if emu.eh_ctx() != 0 {
            for frame in callback_chain(emu) {
                frame.release(emu);
            }
        }
    }
    emu.current_thread_id = active;
}

pub(crate) fn search_x64(emu: &mut Emu, record: &ExceptionRecord) {
    let Some(frame) = Frame::allocate(emu, record, HandlerKind::Seh, 0) else {
        log::error!("cannot allocate x64 exception search frame");
        emu.stop();
        return;
    };
    emu.maps.write_dword(frame.base, SEARCH_MAGIC);
    if copy_context(&mut emu.maps, frame.origin(), frame.cursor()).is_err() {
        frame.release(emu);
        emu.stop();
        return;
    }
    frame.search_next(emu);
}

pub(crate) fn dispatch_x64_api(
    emu: &mut Emu,
    record: &ExceptionRecord,
    address: u64,
    context: Option<u64>,
) -> bool {
    let Some(frame) = Frame::allocate(emu, record, HandlerKind::Seh, 0) else {
        return false;
    };
    emu.maps.write_dword(frame.base, SEARCH_MAGIC);
    emu.maps.write_qword(frame.base + EXTERNAL_RECORD, address);
    emu.maps.write_qword(frame.base + POINTERS_OFFSET, address);
    if let Some(context) = context {
        if copy_context(&mut emu.maps, context, frame.context()).is_err() {
            frame.release(emu);
            return false;
        }
        emu.maps.write_qword(frame.base + EXTERNAL_CONTEXT, context);
        emu.maps.write_qword(
            frame.base + POINTERS_OFFSET + size_of::<u64>() as u64,
            context,
        );
    }
    frame.next_vectored(emu, 0);
    true
}

pub(crate) fn active_record(emu: &Emu) -> Option<u64> {
    Frame::active(emu)
        .map(|frame| {
            if emu.cfg.is_x64() {
                frame.record_address(emu)
            } else {
                frame.record()
            }
        })
        .or_else(|| seh32::active_record(emu))
}

pub(crate) fn enter_record(
    emu: &mut Emu,
    record: &ExceptionRecord,
    kind: HandlerKind,
    veh_index: usize,
) {
    if !emu.cfg.is_x64() && kind == HandlerKind::Seh {
        seh32::enter_record(emu, record);
        return;
    }
    let Some(frame) = Frame::allocate(emu, record, kind, veh_index) else {
        log::error!("cannot allocate exception dispatch frame");
        emu.stop();
        return;
    };
    if !frame.prepare_call(emu, kind) {
        restore_context(emu, frame.context());
        frame.release(emu);
        log::error!("cannot build exception handler stack");
        emu.stop();
    }
}

pub fn enter(emu: &mut Emu, ex_type: ExceptionType) {
    enter_for_handler(emu, ex_type, HandlerKind::Seh);
}

pub fn enter_for_handler(emu: &mut Emu, ex_type: ExceptionType, kind: HandlerKind) {
    let mut record = ExceptionRecord::new(ex_type, emu.pc());
    record.nested = active_record(emu).unwrap_or(0);
    enter_record(emu, &record, kind, 0);
}

pub fn enter32(emu: &mut Emu, ex_type: ExceptionType, kind: HandlerKind) {
    enter_for_handler(emu, ex_type, kind);
}

pub fn enter64(emu: &mut Emu, ex_type: ExceptionType, kind: HandlerKind) {
    enter_for_handler(emu, ex_type, kind);
}

pub fn exit(emu: &mut Emu) {
    if seh32::is_active(emu) {
        seh32::exit(emu);
    } else if let Some(frame) = Frame::active(emu) {
        frame.finish(emu);
    }
}

pub fn exit32(emu: &mut Emu) {
    exit(emu);
}

pub fn exit64(emu: &mut Emu) {
    exit(emu);
}

pub(crate) fn handle_return(emu: &mut Emu, return_address: u64) -> bool {
    let Some(frame) = Frame::active(emu) else {
        return false;
    };
    if return_address != frame.base + RETURN_COOKIE_OFFSET {
        return false;
    }
    frame.finish(emu);
    true
}
