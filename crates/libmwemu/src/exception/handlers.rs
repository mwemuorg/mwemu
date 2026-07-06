/*
    SEH, VEH and UEF

    - SEH: structured exception handler (try/catch, stack items)
    - VEH: vectorized exception handler (API)
            ntdll!RtlAddVectoredExceptionHandler
            ntdll!RtlRemoveVectoredExceptionHandler
    - UEF: unhandled exception filter (API)
            ntdll!RtlSetUnhandledExceptionFilter
            kernelbase!SetUnhandledExceptionFilter

*/

use super::types;
use crate::context::context32::Context32;
use crate::context::context64::Context64;
use crate::emu;
use crate::maps::mem64::Permission;

#[derive(Clone, Copy)]
pub enum HandlerKind {
    Seh,
    Veh,
    Uef,
}

impl HandlerKind {
    #[inline]
    fn as_u32(self) -> u32 {
        match self {
            HandlerKind::Seh => 1,
            HandlerKind::Veh => 2,
            HandlerKind::Uef => 3,
        }
    }
}

pub fn enter(emu: &mut emu::Emu, ex_type: types::ExceptionType) {
    // Backwards-compatible default: generic exception context.
    enter_for_handler(emu, ex_type, HandlerKind::Seh);
}

pub fn enter_for_handler(
    emu: &mut emu::Emu,
    ex_type: types::ExceptionType,
    handler_kind: HandlerKind,
) {
    if emu.cfg.is_x64() {
        enter64(emu, ex_type, handler_kind);
    } else {
        enter32(emu, ex_type, handler_kind);
    }
}

pub fn exit(emu: &mut emu::Emu) {
    if emu.cfg.is_x64() {
        exit64(emu);
    } else {
        exit32(emu);
    }
}

pub fn enter32(emu: &mut emu::Emu, ex_type: types::ExceptionType, handler_kind: HandlerKind) {
    let ctx_addr = emu.maps.alloc(0x1000).expect("out of memory");
    if (ctx_addr + 0x1000) > u32::MAX as u64 {
        panic!("32bits allocator is giving a too big memory!! for the context32");
    }
    let ctx = emu
        .maps
        .create_map("ctx", ctx_addr, 0x1000, Permission::READ_WRITE_EXECUTE);
    emu.set_eh_ctx(ctx_addr + 0x100);

    emu.stack_push32(ctx_addr as u32); // 0x10f00
    emu.stack_push32(emu.regs().get_eip() as u32);

    emu.set_eh_ctx(ctx_addr + 8); // 0x10f08
    // ctx header:
    // +0x00 => handler kind (SEH/VEH/UEF), +0x04 => context ptr
    emu.maps.write_dword(ctx_addr, handler_kind.as_u32());
    emu.maps.write_dword(ctx_addr + 4, emu.eh_ctx() as u32); // 0x10f04
    emu.maps
        .write_dword(emu.eh_ctx(), types::exception_type_code(ex_type)); // STATUS_BREAKPOINT

    let ctx = Context32::new(emu.regs());
    ctx.save(emu.eh_ctx() as u32, &mut emu.maps);
}

pub fn exit32(emu: &mut emu::Emu) {
    let disposition = emu.regs().get_eax() as u32;
    let ctx_ptr = emu.eh_ctx();
    let ex_code = emu.maps.read_dword(ctx_ptr).unwrap_or(0);
    let handler_kind = emu.maps.read_dword(ctx_ptr.saturating_sub(8)).unwrap_or(0);

    let mut ctx = Context32::new(emu.regs());
    ctx.load(emu.eh_ctx() as u32, &mut emu.maps);
    ctx.sync(emu.regs_mut());
    emu.set_eh_ctx(0);
    emu.force_reload = true;
    emu.maps.free("ctx");

    if disposition == crate::windows::constants::EXCEPTION_CONTINUE_SEARCH {
        // Continue search from next handler in chain (VEH -> SEH* -> UEF).
        if (handler_kind == HandlerKind::Veh.as_u32() || handler_kind == HandlerKind::Seh.as_u32())
            && (emu.seh() > 0 || emu.uef() > 0)
        {
            let ex_type = types::exception_type_from_code(ex_code);
            emu.exception(ex_type);
        }
    }
}

pub fn enter64(emu: &mut emu::Emu, ex_type: types::ExceptionType, handler_kind: HandlerKind) {
    let ctx_addr = emu.maps.alloc(0x1000).expect("out of memory");
    let ctx = emu
        .maps
        .create_map("ctx", ctx_addr, 0x1000, Permission::READ_WRITE_EXECUTE);

    emu.stack_push64(ctx_addr); // 0x10f00
    emu.stack_push64(emu.regs().rip);

    emu.set_eh_ctx(ctx_addr + 16);
    // ctx header:
    // +0x00 => handler kind (SEH/VEH/UEF), +0x08 => context ptr
    emu.maps.write_dword(ctx_addr, handler_kind.as_u32());
    emu.maps.write_qword(ctx_addr + 8, emu.eh_ctx());
    // Store the exception code at the start of the context area (mirrors enter32),
    // so exit64 can retrieve it from `eh_ctx()` (offset +0).
    emu.maps
        .write_dword(emu.eh_ctx(), types::exception_type_code(ex_type));
    let ctx = Context64::new(emu.regs());
    ctx.save(emu.eh_ctx(), &mut emu.maps);
}

pub fn exit64(emu: &mut emu::Emu) {
    let disposition = emu.regs().rax as u32;
    let ctx_ptr = emu.eh_ctx();
    let ex_code = emu.maps.read_dword(ctx_ptr).unwrap_or(0);
    let handler_kind = emu.maps.read_dword(ctx_ptr.saturating_sub(16)).unwrap_or(0);

    let mut ctx = Context64::new(emu.regs());
    ctx.load(emu.eh_ctx(), &mut emu.maps);
    ctx.sync(emu.regs_mut());
    emu.set_eh_ctx(0);
    emu.force_reload = true;
    emu.maps.free("ctx");

    if disposition == crate::windows::constants::EXCEPTION_CONTINUE_SEARCH {
        // Continue search from next handler in chain (VEH -> UEF on x64 here).
        if (handler_kind == HandlerKind::Veh.as_u32() || handler_kind == HandlerKind::Seh.as_u32())
            && (emu.seh() > 0 || emu.uef() > 0)
        {
            let ex_type = types::exception_type_from_code(ex_code);
            emu.exception(ex_type);
        }
    }
}
