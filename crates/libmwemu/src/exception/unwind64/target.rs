use super::{
    DispatcherContext, FRAME_LIMIT, FrameResult, RSP, StackStep, WORD, add, capture_context,
    copy_context, load, restore_context, walk_frame,
};
use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::context::context64::Context64;
use crate::emu::Emu;
use crate::exception::handlers::{release_x64_callbacks_through, x64_callback_origin};
use crate::exception::types::ExceptionRecord;
use crate::winapi::policy::{ApiStopReason, api_try, invalid_unwind};
use crate::windows::constants::errors::STATUS_UNWIND;
use crate::windows::constants::exceptions::*;

struct Target {
    stack: u64,
    ip: u64,
    result: u64,
    context: u64,
    history: u64,
}

impl Target {
    fn resume(&self, emu: &mut Emu, abandoned: Option<u64>) -> FrameResult<()> {
        if !emu
            .maps
            .validate_write_range(self.context, Context64::GUEST_SIZE)
        {
            return Err(ApiStopReason::InvalidMemory);
        }
        emu.maps
            .write_qword(self.context + Context64::RAX_OFFSET, self.result);
        emu.maps
            .write_qword(self.context + Context64::RIP_OFFSET, self.ip);
        restore_context(emu, self.context)?;
        // The output context may belong to a callback frame we are about to release.
        if let Some(id) = abandoned
            && !release_x64_callbacks_through(emu, id)
        {
            return Err(ApiStopReason::InvalidFrame);
        }
        Ok(())
    }

    fn walk(&self, emu: &mut Emu, supplied_record: u64, scratch_top: u64) -> FrameResult<()> {
        let entry_sp = emu.sp();
        let scratch = scratch_top
            .checked_sub(ExceptionRecord::SIZE64 + DispatcherContext::SIZE + 16)
            .ok_or(ApiStopReason::InvalidFrame)?
            & !15;
        if !emu
            .maps
            .validate_write_range(scratch, ExceptionRecord::SIZE64 + DispatcherContext::SIZE)
            || self.stack != 0 && (self.stack < entry_sp || !self.stack.is_multiple_of(WORD))
        {
            return Err(ApiStopReason::InvalidFrame);
        }
        let dispatch = scratch + ExceptionRecord::SIZE64;
        let record = if supplied_record == 0 {
            scratch
        } else {
            supplied_record
        };
        if !emu
            .maps
            .validate_write_range(record, ExceptionRecord::SIZE64)
        {
            return Err(ApiStopReason::InvalidMemory);
        }
        capture_context(emu, self.context, emu.gateway_return)?;
        if supplied_record == 0 {
            emu.maps
                .fill_range(record, 0, ExceptionRecord::SIZE64 as usize);
            ExceptionRecord {
                code: STATUS_UNWIND,
                flags: 0,
                nested: 0,
                address: emu.gateway_return,
                information: Vec::new(),
            }
            .save(&mut emu.maps, record, true);
        }
        let flags_at = add(record, ExceptionRecord::FLAGS_OFFSET)?;
        let flags = emu
            .maps
            .read_dword(flags_at)
            .ok_or(ApiStopReason::InvalidMemory)?
            | EXCEPTION_UNWINDING
            | if self.stack == 0 {
                EXCEPTION_EXIT_UNWIND
            } else {
                0
            };
        emu.maps.write_dword(flags_at, flags);
        let mut crossed_callbacks = Vec::new();
        for _ in 0..FRAME_LIMIT {
            let pc = load(&emu.maps, self.context + Context64::RIP_OFFSET)?;
            if let Some((id, origin)) = x64_callback_origin(emu, pc) {
                if crossed_callbacks.contains(&id) {
                    return Err(ApiStopReason::InvalidFrame);
                }
                // The dispatcher return cookie stands for the interrupted guest context.
                copy_context(&mut emu.maps, origin, self.context)?;
                crossed_callbacks.push(id);
                continue;
            }
            if pc == 0 {
                if self.stack == 0 {
                    let exception = ExceptionRecord::load(&emu.maps, record, true)
                        .ok_or(ApiStopReason::InvalidMemory)?;
                    if let Some(id) = crossed_callbacks.last().copied()
                        && !release_x64_callbacks_through(emu, id)
                    {
                        return Err(ApiStopReason::InvalidFrame);
                    }
                    emu.dispatch_unhandled_exception(&exception);
                    return Ok(());
                }
                return Err(ApiStopReason::InvalidFrame);
            }
            let sp = load(&emu.maps, self.context + Context64::RSP_OFFSET)?;
            let StackStep {
                frame,
                image,
                entry,
                handler,
            } = walk_frame(emu, self.context, pc, UNW_FLAG_UHANDLER)?;
            if frame.establisher < sp || (self.stack != 0 && frame.establisher > self.stack) {
                return Err(ApiStopReason::InvalidFrame);
            }
            let reached = self.stack != 0 && frame.establisher == self.stack;
            if handler != 0 {
                let handler_flags = if reached {
                    flags | EXCEPTION_TARGET_UNWIND
                } else {
                    flags
                };
                emu.maps.write_dword(flags_at, handler_flags);
                DispatcherContext {
                    pc,
                    image,
                    entry,
                    establisher: frame.establisher,
                    target: self.ip,
                    context: self.context,
                    handler,
                    data: frame.handler_data.unwrap_or(0),
                    history: self.history,
                }
                .write(&mut emu.maps, dispatch)?;
                let registers = *emu.regs();
                let saved_flags = emu.flags_snapshot();
                let gateway_return = emu.gateway_return;
                emu.set_sp(scratch);
                let result = emu
                    .call64(
                        handler,
                        &[record, frame.establisher, self.context, dispatch],
                    )
                    .map_err(|_| ApiStopReason::InvalidFrame);
                if result.is_err() && (emu.api_policy.stopped() || emu.process_terminated) {
                    return Err(ApiStopReason::InvalidFrame);
                }
                emu.gateway_return = gateway_return;
                *emu.regs_mut() = registers;
                *emu.flags_overwrite_mut() = saved_flags;
                if result? as u32 != ExceptionDisposition::ContinueSearch as u32 {
                    return Err(ApiStopReason::InvalidFrame);
                }
            }
            if reached {
                return self.resume(emu, crossed_callbacks.last().copied());
            }
            if frame.registers[RSP] <= sp {
                return Err(ApiStopReason::InvalidFrame);
            }
            frame.commit(&mut emu.maps)?;
        }
        Err(ApiStopReason::InvalidFrame)
    }
}

pub(super) fn unwind_to_scope(
    emu: &mut Emu,
    stack: u64,
    ip: u64,
    record: u64,
    context: u64,
    history: u64,
) -> FrameResult<()> {
    let code = emu
        .maps
        .read_dword(record)
        .ok_or(ApiStopReason::InvalidMemory)?;
    Target {
        stack,
        ip,
        result: u64::from(code),
        context,
        history,
    }
    .walk(emu, record, emu.sp())
}

pub(crate) fn unwind(emu: &mut Emu, library: &str, api: &str) {
    let call = WinApi(Stdcall);
    let sp = emu.sp();
    let ([stack, ip, record, result], context, history, scratch_top) = if api == "RtlUnwindEx" {
        let [stack, ip, record, result, context, history] = api_try!(emu, call.args(emu));
        ([stack, ip, record, result], context, history, sp)
    } else {
        // RtlUnwind owns its CONTEXT. Keep callback stacks below it during the walk.
        let Some(context) = sp
            .checked_sub(Context64::GUEST_SIZE)
            .map(|address| address & !15)
        else {
            invalid_unwind(emu, library, api, ApiStopReason::InvalidFrame);
            return;
        };
        (api_try!(emu, call.args(emu)), context, 0, context)
    };
    log_red!(
        emu,
        "{library}!{api} target_frame={stack:#x} target_ip={ip:#x} context={context:#x}"
    );
    if let Err(reason) = (Target {
        stack,
        ip,
        result,
        context,
        history,
    })
    .walk(emu, record, scratch_top)
        && !emu.api_policy.stopped()
        && !emu.process_terminated
    {
        invalid_unwind(emu, library, api, reason);
    }
}
