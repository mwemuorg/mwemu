use super::exception_pdata64::build_minimal_pe64;
use crate::context::context64::Context64;
use crate::emu::Emu;
use crate::exception::types::{ExceptionRecord, MemoryAccess};
use crate::tests::helpers::{call_winapi64, invoke_winapi_export, prepare_winapi};
use crate::windows::constants::errors::STATUS_UNWIND;
use crate::windows::constants::exceptions::*;

const IMAGE: u64 = 0x1_4000_0000;
const FUNCTION: u64 = IMAGE + 0x2000;
const BODY: u64 = FUNCTION + 0x80;
const HANDLER_RVA: u32 = 0x5000;
const HANDLER: u64 = IMAGE + HANDLER_RVA as u64;
const OUTER_PC: u64 = IMAGE + 0x6000;
const CONTINUATION: u64 = OUTER_PC + 0x10;
const CALLER_PC: u64 = IMAGE + 0x7000;

fn image(emu: &mut Emu, flags: u8, prolog: &[u8], unwind_codes: &[[u8; 2]]) {
    let info = build_minimal_pe64(emu, IMAGE, HANDLER_RVA);
    emu.maps.write_bytes(FUNCTION, prolog);
    emu.maps.write_byte(BODY, 0x90);
    emu.maps.write_bytes(
        info,
        &[
            1 | (flags << 3),
            prolog.len() as u8,
            unwind_codes.len() as u8,
            0,
        ],
    );
    for (index, code) in unwind_codes.iter().enumerate() {
        emu.maps.write_bytes(info + 4 + index as u64 * 2, code);
    }
    if flags != UNW_FLAG_NHANDLER {
        let aligned_slots = unwind_codes.len().next_multiple_of(2);
        emu.maps
            .write_dword(info + 4 + aligned_slots as u64 * 2, HANDLER_RVA);
    }
}

fn ntdll_call(emu: &mut Emu, api: &str, args: &[u64]) {
    emu.gateway_return = emu.pc();
    call_winapi64(emu, |emu| invoke_winapi_export(emu, "ntdll.dll", api), args);
    assert!(
        !emu.api_policy.stopped(),
        "{api}: {:?}",
        emu.api_policy.last_outcome
    );
}

#[test]
fn both_unwind_exports_restore_nonvolatile_registers_and_stop_at_the_target_frame() {
    const SAVED_RBX: u64 = 0x1111_2222_3333_4444;
    const SAVED_RBP: u64 = 0x5555_6666_7777_8888;
    const RETURN_VALUE: u64 = 0x1234_5678_9abc_def0;
    for api in ["RtlUnwindEx", "RtlUnwind"] {
        let (mut emu, context) = prepare_winapi(true);
        // push rbp; push rbx; sub rsp, 0x38. This includes all six outgoing argument slots.
        let prolog = [0x55, 0x53, 0x48, 0x83, 0xec, 0x38];
        let codes = [
            [6, 0x62], // UWOP_ALLOC_SMALL(0x38)
            [2, 0x30], // UWOP_PUSH_NONVOL(RBX)
            [1, 0x50], // UWOP_PUSH_NONVOL(RBP)
        ];
        image(&mut emu, UNW_FLAG_NHANDLER, &prolog, &codes);
        let sp = emu.sp();
        let target_frame = sp + 0x50;
        emu.maps.write_qword(sp + 0x38, SAVED_RBX);
        emu.maps.write_qword(sp + 0x40, SAVED_RBP);
        emu.maps.write_qword(sp + 0x48, OUTER_PC);
        emu.maps.write_qword(target_frame, CALLER_PC);
        emu.regs_mut().rbx = 0;
        emu.regs_mut().rbp = 0;
        emu.set_pc(BODY);
        let args = [target_frame, CONTINUATION, 0, RETURN_VALUE, context, 0];
        let arg_count = if api == "RtlUnwindEx" { 6 } else { 4 };
        ntdll_call(&mut emu, api, &args[..arg_count]);

        assert_eq!((emu.pc(), emu.sp()), (CONTINUATION, target_frame), "{api}");
        assert_eq!(emu.regs().rax, RETURN_VALUE, "{api}");
        assert_eq!(
            (emu.regs().rbx, emu.regs().rbp),
            (SAVED_RBX, SAVED_RBP),
            "{api}"
        );
        if api == "RtlUnwindEx" {
            assert_eq!(
                emu.maps.read_qword(context + Context64::RIP_OFFSET),
                Some(CONTINUATION)
            );
            assert_eq!(
                emu.maps.read_qword(context + Context64::RSP_OFFSET),
                Some(target_frame)
            );
        }
    }
}

#[test]
fn rtl_unwind_with_no_target_frame_delivers_the_exit_unwind_record() {
    let (mut emu, _) = prepare_winapi(true);
    image(&mut emu, UNW_FLAG_NHANDLER, &[], &[]);
    emu.set_uef(HANDLER);
    emu.set_pc(OUTER_PC);
    emu.maps.write_qword(emu.sp(), 0);
    ntdll_call(&mut emu, "RtlUnwind", &[0, CONTINUATION, 0, 0]);

    assert_eq!(emu.pc(), HANDLER);
    let record = emu
        .maps
        .read_qword(emu.regs().rcx)
        .expect("EXCEPTION_POINTERS.ExceptionRecord");
    assert_eq!(emu.maps.read_dword(record), Some(STATUS_UNWIND));
    assert_eq!(
        emu.maps.read_dword(record + ExceptionRecord::FLAGS_OFFSET),
        Some(EXCEPTION_UNWINDING | EXCEPTION_EXIT_UNWIND)
    );
}

fn dispatch_fixture(flags: u8, fault_pc: u64) -> (Emu, u64, u64, u64) {
    let (mut emu, context) = prepare_winapi(true);
    let record = context + 0x600;
    // push rbp; sub rsp, 0x20. Without a frame register the establisher is the fault RSP.
    let prolog = [0x55, 0x48, 0x83, 0xec, 0x20];
    let codes = [[5, 0x32], [1, 0x50]];
    image(&mut emu, flags, &prolog, &codes);
    let fault_sp = emu.sp();
    emu.set_pc(fault_pc);
    emu.maps.write_qword(fault_sp + 0x20, 0xc031);
    emu.maps.write_qword(fault_sp + 0x28, OUTER_PC);
    emu.maps.write_qword(fault_sp + 0x30, 0);
    Context64::new(emu.regs()).save(context, &mut emu.maps);
    emu.maps.write_dword(
        context + Context64::FLAGS_OFFSET,
        CONTEXT_CONTROL | CONTEXT_INTEGER,
    );
    ExceptionRecord::access_violation(fault_pc, 0, MemoryAccess::Read).save(
        &mut emu.maps,
        record,
        true,
    );
    emu.set_pc(CALLER_PC);
    emu.set_sp(fault_sp - 0x100);
    (emu, record, context, fault_sp)
}

#[test]
fn rtl_dispatch_exception_returns_to_the_api_caller_after_the_guest_handler_returns() {
    let (mut emu, record, context, fault_sp) = dispatch_fixture(UNW_FLAG_EHANDLER, BODY);
    let caller_sp = emu.sp();
    let caller_rbx = 0xcafe_babe;
    emu.regs_mut().rbx = caller_rbx;
    // mov [r8 + CONTEXT.Rax],rdx; xor eax,eax (ExceptionContinueExecution); ret
    emu.maps.write_bytes(
        HANDLER,
        &[
            0x49,
            0x89,
            0x50,
            Context64::RAX_OFFSET as u8,
            0x31,
            0xc0,
            0xc3,
        ],
    );
    ntdll_call(&mut emu, "RtlDispatchException", &[record, context]);

    assert_eq!(emu.pc(), HANDLER);
    assert_eq!(
        (emu.regs().rcx, emu.regs().rdx, emu.regs().r8),
        (record, fault_sp, context)
    );
    // Direct gateway entry can leave the caller's pending PC reload for the first step.
    for _ in 0..4 {
        assert!(emu.step());
        if emu.eh_ctx() == 0 {
            break;
        }
    }
    assert_eq!(emu.last_ret.1, HANDLER + 6);
    assert_eq!((emu.pc(), emu.sp()), (CALLER_PC, caller_sp));
    assert_eq!((emu.regs().rax, emu.regs().rbx), (1, caller_rbx));
    assert_eq!(
        emu.maps.read_qword(context + Context64::RAX_OFFSET),
        Some(fault_sp)
    );
    assert_eq!(
        emu.maps.read_qword(context + Context64::RIP_OFFSET),
        Some(BODY)
    );
    assert_eq!(
        emu.maps.read_qword(context + Context64::RSP_OFFSET),
        Some(fault_sp)
    );
    assert_eq!(emu.eh_ctx(), 0);
}

#[test]
fn rtl_dispatch_exception_returns_false_for_a_termination_handler_or_a_leaf() {
    for (flags, fault_pc) in [(UNW_FLAG_UHANDLER, BODY), (UNW_FLAG_EHANDLER, OUTER_PC)] {
        let (mut emu, record, context, fault_sp) = dispatch_fixture(flags, fault_pc);
        let caller_sp = emu.sp();
        emu.regs_mut().rax = 1;
        ntdll_call(&mut emu, "RtlDispatchException", &[record, context]);

        assert_eq!((emu.pc(), emu.sp()), (CALLER_PC, caller_sp));
        assert_eq!(emu.regs().rax, 0);
        assert_eq!(emu.eh_ctx(), 0);
        assert_eq!(
            emu.maps.read_dword(record + ExceptionRecord::FLAGS_OFFSET),
            Some(0)
        );
        assert_eq!(
            emu.maps.read_qword(context + Context64::RIP_OFFSET),
            Some(fault_pc)
        );
        assert_eq!(
            emu.maps.read_qword(context + Context64::RSP_OFFSET),
            Some(fault_sp)
        );
    }
}
