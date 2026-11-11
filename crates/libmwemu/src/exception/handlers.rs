//! Dispatch frames for CPU exceptions, VEH and unhandled exception filters.

use crate::context::context32::Context32;
use crate::context::context64::Context64;
use crate::emu::Emu;
use crate::exception::seh32;
use crate::exception::types::{ExceptionRecord, ExceptionType};
use crate::maps::mem64::Permission;
use crate::windows::constants::exceptions::*;

// Private dispatcher-frame layout.
const FRAME_SIZE: u64 = 0x1000;
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
        let base = emu.eh_ctx().checked_sub(CONTEXT_OFFSET)?;
        (emu.maps.read_dword(base) == Some(FRAME_MAGIC)).then_some(Self { base })
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
        let base = emu.maps.alloc(FRAME_SIZE)?;
        let is_64 = emu.cfg.is_x64();
        if !is_64 && base.checked_add(FRAME_SIZE)? > u32::MAX as u64 {
            return None;
        }
        let frame = Self { base };
        emu.maps
            .create_map(&frame.map_name(), base, FRAME_SIZE, Permission::READ_WRITE)
            .ok()?;
        let previous = emu.eh_ctx();
        emu.maps.write_dword(base, FRAME_MAGIC);
        emu.maps.write_qword(base + PREVIOUS_CONTEXT, previous);
        emu.maps.write_dword(base + HANDLER_KIND, kind.as_u32());
        emu.maps.write_dword(base + VEH_INDEX, veh_index as u32);
        record.save(&mut emu.maps, frame.record(), is_64);
        save_context(emu, frame.context());
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
            // This preserves the existing direct .pdata dispatch; virtual unwinding
            // of caller frames and a complete DISPATCHER_CONTEXT are not implemented.
            let regs = emu.regs_mut();
            regs.rcx = self.record();
            regs.rdx = original_rsp;
            regs.r8 = self.context();
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
        let kind = emu.maps.read_dword(self.base + HANDLER_KIND).unwrap_or(0);
        let next_veh = emu.maps.read_dword(self.base + VEH_INDEX).unwrap_or(0) as usize + 1;
        let record = ExceptionRecord::load(&emu.maps, self.record(), emu.cfg.is_x64());
        restore_context(emu, self.context());
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
    if emu.cfg.is_x64() {
        Context64::new(emu.regs()).save(address, &mut emu.maps);
        emu.maps.write_dword(
            address + Context64::FLAGS_OFFSET,
            CONTEXT_CONTROL | CONTEXT_INTEGER,
        );
        emu.maps
            .write_dword(address + Context64::EFLAGS_OFFSET, flags);
    } else {
        Context32::new(emu.regs()).save(address as u32, &mut emu.maps);
        emu.maps.write_dword(
            address + Context32::FLAGS_OFFSET,
            WOW64_CONTEXT_CONTROL | WOW64_CONTEXT_INTEGER,
        );
        emu.maps
            .write_dword(address + Context32::EFLAGS_OFFSET, flags);
    }
}

pub(crate) fn restore_context(emu: &mut Emu, address: u64) {
    let flags_offset = if emu.cfg.is_x64() {
        let mut context = Context64::new(emu.regs());
        context.load(address, &mut emu.maps);
        context.sync(emu.regs_mut());
        Context64::EFLAGS_OFFSET
    } else {
        let mut context = Context32::new(emu.regs());
        context.load(address as u32, &mut emu.maps);
        context.sync(emu.regs_mut());
        Context32::EFLAGS_OFFSET
    };
    let flags = emu.maps.read_dword(address + flags_offset).unwrap_or(0);
    emu.flags_mut().load(flags);
    emu.force_reload = true;
}

pub(crate) fn active_record(emu: &Emu) -> Option<u64> {
    Frame::active(emu)
        .map(|frame| frame.record())
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
