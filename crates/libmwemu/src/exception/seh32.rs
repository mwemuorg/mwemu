//! Guest arguments and continuation state for x86 stack-chain SEH.

use crate::context::context32::Context32;
use crate::emu::Emu;
use crate::maps::mem64::Permission;
use crate::windows::constants::errors::*;

use super::{handlers, types};

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

const EXCEPTION_CODE: u64 = 0;
const EXCEPTION_FLAGS: u64 = 4;
const EXCEPTION_RECORD: u64 = 8;
const EXCEPTION_ADDRESS: u64 = 12;
const NUMBER_PARAMETERS: u64 = 16;
const CONTEXT_CONTROL_INTEGER: u32 = 0x0001_0003;
const CONTEXT_EFLAGS: u64 = 0xc0;
const REGISTRATION_HANDLER: u64 = 4;

// EXCEPTION_DISPOSITION values, not the EXCEPTION_* filter return values.
const CONTINUE_EXECUTION: u32 = 0;
const CONTINUE_SEARCH: u32 = 1;
const END_OF_CHAIN: u32 = u32::MAX;

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

    fn allocate(emu: &mut Emu, ex_type: types::ExceptionType) -> Option<Self> {
        let base = emu.maps.alloc(FRAME_SIZE)?;
        if base.checked_add(FRAME_SIZE)? > u32::MAX as u64 {
            return None;
        }
        let frame = Self { base };
        emu.maps
            .create_map(&frame.map_name(), base, FRAME_SIZE, Permission::READ_WRITE)
            .ok()?;

        let previous = emu.eh_ctx();
        let nested_record = Self::active(emu).map(|f| f.record()).unwrap_or(0);
        emu.maps.write_dword(base, FRAME_MAGIC);
        emu.maps
            .write_dword(base + PREVIOUS_CONTEXT, previous as u32);
        emu.maps.write_dword(base + HANDLERS_VISITED, 1);
        emu.maps
            .write_dword(frame.record() + EXCEPTION_CODE, exception_code(ex_type));
        emu.maps.write_dword(frame.record() + EXCEPTION_FLAGS, 0);
        emu.maps
            .write_dword(frame.record() + EXCEPTION_RECORD, nested_record as u32);
        emu.maps
            .write_dword(frame.record() + EXCEPTION_ADDRESS, emu.pc() as u32);
        // ExceptionType does not carry access type/address or software arguments.
        emu.maps.write_dword(frame.record() + NUMBER_PARAMETERS, 0);

        // Capture the fault-time ESP before pushing the dispatcher call frame.
        Context32::new(emu.regs()).save(frame.context() as u32, &mut emu.maps);
        emu.maps
            .write_dword(frame.context(), CONTEXT_CONTROL_INTEGER);
        let flags = emu.flags().dump();
        emu.maps
            .write_dword(frame.context() + CONTEXT_EFLAGS, flags);
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
        let mut context = Context32::new(emu.regs());
        context.load(self.context() as u32, &mut emu.maps);
        context.sync(emu.regs_mut());
        let flags = emu
            .maps
            .read_dword(self.context() + CONTEXT_EFLAGS)
            .unwrap_or(0);
        emu.flags_mut().load(flags);
        emu.force_reload = true;
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
        if next == 0 || next == END_OF_CHAIN {
            let code = emu
                .maps
                .read_dword(self.record() + EXCEPTION_CODE)
                .unwrap_or(0);
            self.release(emu);
            let uef = emu.uef();
            if uef != 0 && emu.maps.is_mapped(uef) {
                handlers::enter_for_handler(
                    emu,
                    types::exception_type_from_code(code),
                    handlers::HandlerKind::Uef,
                );
                emu.set_eip(uef, false);
            } else {
                log::error!("x86 SEH: no handler accepted the exception");
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
            CONTINUE_EXECUTION => self.release(emu),
            CONTINUE_SEARCH => self.continue_search(emu),
            // Nested/collided unwind dispositions require an unwinder.
            _ => self.stop(emu, "unsupported exception disposition"),
        }
    }
}

fn exception_code(ex_type: types::ExceptionType) -> u32 {
    use types::ExceptionType::*;
    match ex_type {
        PopfCannotReadStack
        | WritingWord
        | SettingRipToNonMappedAddr
        | QWordDereferencing
        | DWordDereferencing
        | WordDereferencing
        | ByteDereferencing
        | BadAddressDereferencing
        | SettingXmmOperand
        | ReadingXmmOperand => STATUS_ACCESS_VIOLATION as u32,
        _ => types::exception_type_code(ex_type),
    }
}

pub(crate) fn enter(emu: &mut Emu, ex_type: types::ExceptionType) {
    let Some(frame) = Frame::allocate(emu, ex_type) else {
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
