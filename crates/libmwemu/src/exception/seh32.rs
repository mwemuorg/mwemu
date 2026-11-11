//! Guest arguments and continuation state for x86 stack-chain SEH.

use crate::emu::Emu;
use crate::exception::handlers;
use crate::exception::types::ExceptionRecord;
use crate::maps::mem64::Permission;
use crate::windows::constants::exceptions::*;

const FRAME_SIZE: u64 = 0x1000;
const FRAME_MAGIC: u32 = u32::from_le_bytes(*b"SEH3");
const PREVIOUS_CONTEXT: u64 = 4;
const ESTABLISHER_FRAME: u64 = 8;
const HANDLERS_VISITED: u64 = 12;
const MAX_HANDLERS: u32 = 64;
const RECORD_OFFSET: u64 = 0x20;
const DISPATCHER_OFFSET: u64 = 0x80;
const CONTEXT_OFFSET: u64 = 0x100;
const RETURN_COOKIE_OFFSET: u64 = 0xf00;

const REGISTRATION_HANDLER: u64 = 4;

#[derive(Clone, Copy)]
struct Frame {
    base: u64,
}

impl Frame {
    fn active(emu: &Emu) -> Option<Self> {
        if emu.cfg.is_x64() {
            return None;
        }
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
        format!("seh32_{:x}", self.base)
    }

    fn allocate(emu: &mut Emu, record: &ExceptionRecord) -> Option<Self> {
        let base = emu.maps.alloc(FRAME_SIZE)?;
        if base.checked_add(FRAME_SIZE)? > u32::MAX as u64 {
            return None;
        }
        let frame = Self { base };
        emu.maps
            .create_map(&frame.map_name(), base, FRAME_SIZE, Permission::READ_WRITE)
            .ok()?;

        let previous = emu.eh_ctx();
        emu.maps.write_dword(base, FRAME_MAGIC);
        emu.maps
            .write_dword(base + PREVIOUS_CONTEXT, previous as u32);
        emu.maps.write_dword(base + HANDLERS_VISITED, 1);
        record.save(&mut emu.maps, frame.record(), false);
        handlers::save_context(emu, frame.context());
        emu.set_eh_ctx(frame.context());
        Some(frame)
    }

    fn push_arguments(self, emu: &mut Emu, establisher: u32) -> bool {
        emu.maps
            .write_dword(self.base + ESTABLISHER_FRAME, establisher);
        emu.maps.write_dword(self.base + DISPATCHER_OFFSET, 0);
        // Right-to-left: DispatcherContext, ContextRecord, EstablisherFrame,
        // ExceptionRecord, then a private return address handled by RET.
        [
            (self.base + DISPATCHER_OFFSET) as u32,
            self.context() as u32,
            establisher,
            self.record() as u32,
            (self.base + RETURN_COOKIE_OFFSET) as u32,
        ]
        .into_iter()
        .all(|value| emu.stack_push32(value))
    }

    fn restore_context(self, emu: &mut Emu) {
        handlers::restore_context(emu, self.context());
    }

    fn release(self, emu: &mut Emu) {
        let previous = emu
            .maps
            .read_dword(self.base + PREVIOUS_CONTEXT)
            .unwrap_or(0);
        emu.set_eh_ctx(previous as u64);
        emu.maps.free(&self.map_name());
    }

    fn stop(self, emu: &mut Emu, reason: &str) {
        log::error!("x86 SEH: {}", reason);
        self.release(emu);
        emu.stop();
    }

    fn continue_search(self, emu: &mut Emu) {
        let frame = emu
            .maps
            .read_dword(self.base + ESTABLISHER_FRAME)
            .unwrap_or(0);
        let Some(next) = emu.maps.read_dword(frame as u64) else {
            self.stop(emu, "unreadable registration record");
            return;
        };
        if next == 0 || next == EXCEPTION_CHAIN_END {
            let record = ExceptionRecord::load(&emu.maps, self.record(), false);
            self.release(emu);
            if let Some(record) = record {
                emu.dispatch_unhandled_exception(&record);
            } else {
                log::error!("x86 SEH: invalid exception record");
                emu.stop();
            }
            return;
        }

        let visited = emu
            .maps
            .read_dword(self.base + HANDLERS_VISITED)
            .unwrap_or(MAX_HANDLERS);
        if visited >= MAX_HANDLERS {
            self.stop(emu, "registration chain limit exceeded");
            return;
        }
        let Some(handler) = emu.maps.read_dword(next as u64 + REGISTRATION_HANDLER) else {
            self.stop(emu, "unreadable handler pointer");
            return;
        };
        if !emu.maps.is_mapped(handler as u64) {
            self.stop(emu, "unmapped handler address");
            return;
        }
        emu.maps
            .write_dword(self.base + HANDLERS_VISITED, visited + 1);
        if !self.push_arguments(emu, next) {
            self.restore_context(emu);
            self.stop(emu, "cannot build handler stack");
            return;
        }
        emu.set_eip(handler as u64, false);
    }

    fn finish(self, emu: &mut Emu) {
        let disposition = emu.regs().get_eax() as u32;
        self.restore_context(emu);
        match disposition {
            value if value == ExceptionDisposition::ContinueExecution as u32 => self.release(emu),
            value if value == ExceptionDisposition::ContinueSearch as u32 => {
                self.continue_search(emu)
            }
            // Nested/collided unwind dispositions require an unwinder.
            _ => self.stop(emu, "unsupported exception disposition"),
        }
    }
}

pub(crate) fn enter_record(emu: &mut Emu, record: &ExceptionRecord) {
    let Some(frame) = Frame::allocate(emu, record) else {
        log::error!("x86 SEH: cannot allocate exception records");
        emu.stop();
        return;
    };
    if !frame.push_arguments(emu, emu.seh() as u32) {
        frame.restore_context(emu);
        frame.stop(emu, "cannot build handler stack");
    }
}

pub(crate) fn is_active(emu: &Emu) -> bool {
    Frame::active(emu).is_some()
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

pub(crate) fn exit(emu: &mut Emu) {
    if let Some(frame) = Frame::active(emu) {
        frame.finish(emu);
    }
}

pub(crate) fn active_record(emu: &Emu) -> Option<u64> {
    Frame::active(emu).map(|frame| frame.record())
}
