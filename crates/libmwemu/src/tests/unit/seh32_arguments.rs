use crate::emu::Emu;
use crate::maps::mem64::Permission;
use crate::windows::constants::errors::{
    STATUS_ACCESS_VIOLATION, STATUS_ILLEGAL_INSTRUCTION, STATUS_PRIVILEGED_INSTRUCTION,
    STATUS_SINGLE_STEP, STATUS_UNWIND,
};
use crate::windows::constants::exceptions::*;
use crate::{emu32, tests::helpers};

const CODE: u64 = 0x400000;
const HANDLER: u64 = CODE + 0x100;
const NEXT_HANDLER: u64 = CODE + 0x200;
const REGISTRATION: u64 = 0x500000;
const NEXT_REGISTRATION: u64 = REGISTRATION + 8;
const CONTEXT_DR6: u64 = 0x14;
const CONTEXT_EAX: u64 = 0xb0;
const CONTEXT_EIP: u64 = 0xb8;
const CONTEXT_EFLAGS: u64 = 0xc0;
const CONTEXT_ESP: u64 = 0xc4;

fn emulator(handler: &[u8]) -> Emu {
    helpers::setup();
    let mut emu = emu32();
    emu.cfg.console_enabled = false;
    emu.cfg.soft_exception_recovery = false;
    emu.running_script = true;
    emu.init_stack32();
    emu.maps
        .create_map("code", CODE, 0x1000, Permission::READ_WRITE_EXECUTE)
        .unwrap();
    emu.maps
        .create_map("registration", REGISTRATION, 0x1000, Permission::READ_WRITE)
        .unwrap();
    emu.maps.write_bytes(CODE, &[0x89, 0x08, 0x90]); // mov [eax], ecx; nop
    emu.maps.write_bytes(HANDLER, handler);
    emu.maps.write_dword(REGISTRATION, EXCEPTION_CHAIN_END);
    emu.maps.write_dword(REGISTRATION + 4, HANDLER as u32);
    emu.set_seh(REGISTRATION);
    emu.set_pc(CODE);
    emu.regs_mut().rax = 0;
    emu.regs_mut().rbx = 0x12345678;
    emu.flags_mut().load(0x246);
    emu
}

fn dword(emu: &Emu, address: u64) -> u64 {
    emu.maps.read_dword(address).expect("readable guest DWORD") as u64
}

fn argument(emu: &Emu, index: u64) -> u64 {
    dword(emu, emu.regs().get_esp() + 4 * index)
}

fn dispatch(emu: &mut Emu) {
    // Step once so this ABI test is independent of cached run-loop recovery.
    assert!(!emu.step(), "the write must fault");
    assert_eq!(emu.pc(), HANDLER);
    assert!(!emu.process_terminated);
}

fn step_until(emu: &mut Emu, pc: u64) {
    for _ in 0..32 {
        if emu.pc() == pc {
            return;
        }
        assert!(emu.step(), "handler instruction at 0x{:x}", emu.pc());
        assert!(!emu.process_terminated);
    }
    panic!("handler did not reach 0x{pc:x}");
}

#[test]
fn seh32_passes_windows_arguments_and_fault_time_context() {
    let mut emu = emulator(&[0x90]);
    let esp = emu.regs().get_esp();
    dispatch(&mut emu);

    let record = argument(&emu, 1);
    let context = argument(&emu, 3);
    assert_eq!(emu.regs().get_esp(), esp - 7 * size_of::<u32>() as u64);
    assert_eq!(argument(&emu, 2), REGISTRATION);
    assert!(emu.maps.is_mapped(argument(&emu, 4)));
    assert_ne!(record, context);
    assert_eq!(dword(&emu, record), STATUS_ACCESS_VIOLATION);
    assert_eq!(dword(&emu, record + 4), 0); // ExceptionFlags
    assert_eq!(dword(&emu, record + 8), 0); // nested ExceptionRecord
    assert_eq!(dword(&emu, record + 12), CODE);
    assert_eq!(dword(&emu, record + 16), 2); // NumberParameters
    assert_eq!(dword(&emu, record + 20), EXCEPTION_WRITE_FAULT); // write access
    assert_eq!(dword(&emu, record + 24), 0); // faulting address
    assert_eq!(
        dword(&emu, context),
        (WOW64_CONTEXT_CONTROL | WOW64_CONTEXT_INTEGER | WOW64_CONTEXT_DEBUG_REGISTERS) as u64
    );
    assert_eq!(dword(&emu, context + CONTEXT_EIP), CODE);
    assert_eq!(dword(&emu, context + CONTEXT_ESP), esp);
    assert_eq!(dword(&emu, context + CONTEXT_EFLAGS), 0x246);
    assert_eq!(dword(&emu, emu.seh()), REGISTRATION);
}

#[test]
fn seh32_guest_handler_patches_fault_site_and_resumes_context() {
    for return_bytes in [&[0xc3][..], &[0xc2, 0x10, 0x00][..]] {
        let mut handler = vec![
            0x8b, 0x54, 0x24, 0x04, // mov edx, [esp+4]: ExceptionRecord
            0x8b, 0x52, 0x0c, // mov edx, [edx+0xc]: ExceptionAddress (PECompact)
            0xc6, 0x02, 0x90, // mov byte [edx], 0x90
            0x8b, 0x54, 0x24, 0x0c, // mov edx, [esp+12]: ContextRecord
            0x83, 0x82, 0xb8, 0, 0, 0, 0x02, // add dword [edx+0xb8], 2
            0xc7, 0x82, 0xc0, 0, 0, 0, 0x02, 0x02, 0, 0, // EFlags = 0x202
            0x31, 0xdb, // xor ebx, ebx: must be restored from CONTEXT
            0x31, 0xc0, // ExceptionContinueExecution = 0
        ];
        handler.extend_from_slice(return_bytes);
        let mut emu = emulator(&handler);
        let esp = emu.regs().get_esp();
        dispatch(&mut emu);
        let record = argument(&emu, 1);
        step_until(&mut emu, CODE + 2);
        assert_eq!(emu.maps.read_byte(CODE), Some(0x90));
        assert_eq!(emu.regs().get_esp(), esp);
        assert_eq!(emu.regs().rbx, 0x12345678);
        assert_eq!(emu.flags().dump(), 0x202);
        assert_eq!(emu.eh_ctx(), 0);
        assert!(!emu.maps.is_mapped(record), "release dispatcher allocation");
        assert_eq!(emu.seh(), REGISTRATION);
    }
}

#[test]
fn seh32_helper_ret_does_not_finish_the_handler() {
    let mut emu = emulator(&[
        0xe8, 0x03, 0, 0, 0, // call helper at HANDLER+8
        0x31, 0xc0, 0xc3, // xor eax, eax; ret (handler)
        0x90, 0xc3, // nop; ret (helper)
    ]);
    dispatch(&mut emu);
    let context = emu.eh_ctx();
    assert!(emu.step()); // call
    assert_eq!(emu.pc(), HANDLER + 8);
    assert!(emu.step()); // nop
    assert!(emu.step()); // helper ret
    assert_eq!(emu.pc(), HANDLER + 5);
    assert_eq!(emu.eh_ctx(), context);
    step_until(&mut emu, CODE);
    assert_eq!(emu.eh_ctx(), 0);
}

#[test]
fn seh32_continue_search_calls_next_registration_with_same_records() {
    let mut emu = emulator(&[0xb8, 1, 0, 0, 0, 0xc3]); // return ContinueSearch
    emu.maps.write_dword(REGISTRATION, NEXT_REGISTRATION as u32);
    emu.maps.write_dword(NEXT_REGISTRATION, EXCEPTION_CHAIN_END);
    emu.maps
        .write_dword(NEXT_REGISTRATION + 4, NEXT_HANDLER as u32);
    emu.maps.write_bytes(NEXT_HANDLER, &[0x31, 0xc0, 0xc3]);
    let esp = emu.regs().get_esp();
    dispatch(&mut emu);
    let record = argument(&emu, 1);
    let context = argument(&emu, 3);
    step_until(&mut emu, NEXT_HANDLER);
    assert_eq!(argument(&emu, 1), record);
    assert_eq!(argument(&emu, 2), NEXT_REGISTRATION);
    assert_eq!(argument(&emu, 3), context);
    assert_eq!(emu.regs().get_esp(), esp - 7 * size_of::<u32>() as u64);
    assert_eq!(dword(&emu, emu.seh()), REGISTRATION);
    step_until(&mut emu, CODE);
    assert_eq!(emu.regs().get_esp(), esp);
    assert_eq!(emu.eh_ctx(), 0);
}

#[test]
fn seh32_continue_execution_keeps_registration_for_later_faults() {
    let mut emu = emulator(&[0x31, 0xc0, 0xc3]);
    let esp = emu.regs().get_esp();
    for _ in 0..3 {
        dispatch(&mut emu);
        step_until(&mut emu, CODE);
        assert_eq!(emu.seh(), REGISTRATION);
        assert_eq!(emu.regs().get_esp(), esp);
        assert_eq!(emu.eh_ctx(), 0);
    }
}

#[test]
fn seh32_nested_dispatch_restores_outer_handler_context() {
    let mut emu = emulator(&[0x31, 0xc0, 0xc3]);
    dispatch(&mut emu);
    let outer_context = emu.eh_ctx();
    let outer_record = argument(&emu, 1);
    let outer_esp = emu.regs().get_esp();
    // A real second write fault inside the outer handler.
    emu.maps
        .write_bytes(HANDLER, &[0x89, 0x08, 0x31, 0xc0, 0xc3]);
    dispatch(&mut emu);
    let inner_record = argument(&emu, 1);
    assert_ne!(emu.eh_ctx(), outer_context);
    assert_eq!(dword(&emu, inner_record + 8), outer_record);
    // Make the faulting instruction resumable while the nested handler runs.
    emu.maps.write_bytes(HANDLER, &[0x90, 0x90]);
    for _ in 0..4 {
        assert!(emu.step());
    }
    assert_eq!(emu.pc(), HANDLER);
    assert_eq!(emu.eh_ctx(), outer_context);
    assert_eq!(emu.regs().get_esp(), outer_esp);
    assert!(!emu.maps.is_mapped(inner_record));
    step_until(&mut emu, CODE);
    assert_eq!(emu.eh_ctx(), 0);
}

#[test]
fn seh32_exhausted_or_invalid_chain_stops_without_retrying_fault() {
    for next in [0, EXCEPTION_CHAIN_END, 0x900000] {
        let mut emu = emulator(&[0xb8, 1, 0, 0, 0, 0xc3]);
        emu.maps.write_dword(REGISTRATION, next);
        let esp = emu.regs().get_esp();
        dispatch(&mut emu);
        assert!(emu.step());
        assert!(emu.step());
        assert!(emu.process_terminated);
        assert_eq!(emu.eh_ctx(), 0);
        assert_eq!(emu.regs().get_esp(), esp);
    }
}

#[test]
fn seh32_cyclic_chain_stops_with_bounded_dispatch() {
    let mut emu = emulator(&[0xb8, 1, 0, 0, 0, 0xc3]);
    emu.maps.write_dword(REGISTRATION, REGISTRATION as u32);
    dispatch(&mut emu);
    for _ in 0..256 {
        if emu.process_terminated {
            break;
        }
        assert!(emu.step());
    }
    assert!(emu.process_terminated);
    assert_eq!(emu.eh_ctx(), 0);
}

fn call_rtl_unwind(emu: &mut Emu, dll: &str, target: u32, record: u32) {
    use crate::winapi::winapi32;

    for arg in [37, record, NEXT_HANDLER as u32, target] {
        assert!(emu.stack_push32(arg));
    }
    winapi32::gateway_by_import(emu, dll, "RtlUnwind", 0);
    assert!(!emu.api_policy.stopped());
    // The gateway was invoked directly; consume the reload normally consumed by the CALL step.
    emu.force_reload = false;
}

#[test]
fn rtl_unwind_keeps_target_and_restores_the_handler_modified_context() {
    for (dll, supplied) in [("kernel32.dll", false), ("ntdll.dll", true)] {
        let mut emu = emulator(&[
            0xb8, 1, 0, 0, 0,    // mov eax, ExceptionContinueSearch
            0xc3, // ret
        ]);
        let sp = emu.sp();
        let inner = sp as u32 + 0x40;
        let target = inner + 8;
        emu.maps.write_dword(inner as u64, target);
        emu.maps.write_dword(inner as u64 + 4, HANDLER as u32);
        emu.maps.write_dword(target as u64, EXCEPTION_CHAIN_END);
        emu.maps.write_dword(target as u64 + 4, NEXT_HANDLER as u32);
        emu.set_seh(inner as u64);
        emu.set_last_error(73);
        let record = if supplied {
            REGISTRATION as u32 + 0x100
        } else {
            0
        };
        if supplied {
            emu.maps.write_dword(record as u64, 0x1234);
            emu.maps
                .write_dword(record as u64 + 4, EXCEPTION_NONCONTINUABLE);
        }
        call_rtl_unwind(&mut emu, dll, target, record);
        assert_eq!(emu.pc(), HANDLER);
        let received = argument(&emu, 1);
        if supplied {
            assert_eq!(received, record as u64);
            assert_eq!(dword(&emu, received), 0x1234);
            assert_eq!(
                dword(&emu, received + 4),
                (EXCEPTION_UNWINDING | EXCEPTION_NONCONTINUABLE) as u64
            );
        } else {
            assert_eq!(dword(&emu, received), STATUS_UNWIND as u64);
            assert_eq!(dword(&emu, received + 4), EXCEPTION_UNWINDING as u64);
        }
        assert_eq!(argument(&emu, 2), inner as u64);
        let context = argument(&emu, 3);
        assert_eq!(dword(&emu, context + CONTEXT_EIP), CODE);
        assert_eq!(dword(&emu, context + CONTEXT_ESP), sp);
        assert_eq!(dword(&emu, context + CONTEXT_EAX), 37);
        emu.maps.write_dword(context + CONTEXT_EAX, 42);
        step_until(&mut emu, CODE);
        assert_eq!(emu.regs().get_eax(), 42);
        assert_eq!(emu.seh(), target as u64);
        assert_eq!(emu.sp(), sp);
        assert_eq!(emu.eh_ctx(), 0);
        assert_eq!(emu.get_last_error(), 73);
        call_rtl_unwind(&mut emu, dll, target, 0);
        assert_eq!(emu.pc(), CODE);
        assert_eq!(emu.regs().get_eax(), 37);
        assert_eq!(emu.seh(), target as u64);
    }
}

#[test]
fn rtl_unwind_exit_calls_termination_handler_and_raises_the_final_exception() {
    let mut emu = emulator(&[0xb8, 1, 0, 0, 0, 0xc3]); // ContinueSearch; ret
    let registration = emu.sp() + 0x40;
    emu.maps.write_dword(registration, EXCEPTION_CHAIN_END);
    emu.maps.write_dword(registration + 4, HANDLER as u32);
    emu.set_seh(registration);
    call_rtl_unwind(&mut emu, "kernel32.dll", 0, 0);
    assert_eq!(emu.pc(), HANDLER);
    let record = argument(&emu, 1);
    assert_eq!(
        dword(&emu, record + 4),
        (EXCEPTION_UNWINDING | EXCEPTION_EXIT_UNWIND) as u64
    );
    assert!(emu.step());
    assert!(emu.step());
    assert_eq!(emu.seh(), EXCEPTION_CHAIN_END as u64);
    assert!(emu.process_terminated);
    assert_eq!(emu.eh_ctx(), 0);
}

#[test]
fn rtl_unwind_detects_corrupt_stack_chains() {
    let mut emu = emulator(&[0xb8, 1, 0, 0, 0, 0xc3]); // ContinueSearch; ret
    let cycle = emu.sp() + 0x40;
    emu.maps.write_dword(cycle, cycle as u32);
    emu.maps.write_dword(cycle + 4, HANDLER as u32);
    emu.set_seh(cycle);
    call_rtl_unwind(&mut emu, "ntdll.dll", EXCEPTION_CHAIN_END, 0);
    assert_eq!(emu.pc(), HANDLER);
    assert!(emu.step());
    assert!(emu.step());
    assert!(emu.process_terminated);
    assert_eq!(emu.eh_ctx(), 0);
}

#[test]
fn windows_i386_debug_traps_report_the_resumption_address() {
    for (code, trap_at, expected) in [
        (&[0xf1][..], 0, 1),                         // ICEBP
        (&[0x9d, 0xf8][..], 1, 2),                   // POPFD sets TF; CLC completes before the trap
        (&[0x9d, 0xeb, 0x02, 0x90, 0x90][..], 1, 5), // A stepped jump reports its destination
    ] {
        for cached in [false, true] {
            let mut emu = emulator(&[0x90]);
            emu.maps.write_bytes(CODE, code);
            if trap_at != 0 {
                assert!(emu.stack_push32(0x102));
            }
            if cached {
                assert!(emu.run(Some(HANDLER)).is_ok());
            } else {
                for _ in 0..=trap_at {
                    assert!(emu.step());
                }
            }
            assert_eq!(emu.pc(), HANDLER);
            let record = argument(&emu, 1);
            let context = argument(&emu, 3);
            assert_eq!(dword(&emu, record), STATUS_SINGLE_STEP as u64);
            assert_eq!(dword(&emu, record + 12), CODE + expected);
            assert_eq!(dword(&emu, context + CONTEXT_EIP), CODE + expected);
            assert_eq!(dword(&emu, context + CONTEXT_EFLAGS) & 0x100, 0);
            assert_eq!(
                dword(&emu, context + CONTEXT_DR6) & (1 << 14) != 0,
                trap_at != 0
            );
            assert!(!emu.flags_ref().f_tf);
        }
    }
}

#[test]
fn windows_i386_execution_breakpoint_resumes_once_with_rf() {
    const DR0: u64 = 4;
    const DR1: u64 = 8;
    const DR2: u64 = 12;
    const DR3: u64 = 16;
    const DR7: u64 = 24;
    const RESUME_FLAG: u32 = 1 << 16;
    // Slot 0 is local execute, slot 1 is local write, slot 2 is disabled, slot 3 is global execute.
    const BREAKPOINT_CONTROL: u32 = 0b1000_0101 | (1 << 20);
    let breakpoint = CODE + 2;

    for cached in [false, true] {
        let mut emu = emulator(&[0x31, 0xc0, 0xc3]); // xor eax,eax; ret: ContinueExecution
        emu.maps.write_bytes(breakpoint, &[0x43, 0x90]); // inc ebx; nop
        let starting_ebx = emu.regs().get_ebx();
        dispatch(&mut emu);

        let context = argument(&emu, 3);
        for offset in [DR0, DR1, DR2, DR3, CONTEXT_EIP] {
            emu.maps.write_dword(context + offset, breakpoint as u32);
        }
        emu.maps.write_dword(context + DR7, BREAKPOINT_CONTROL);
        step_until(&mut emu, breakpoint);

        if cached {
            assert!(emu.run(Some(HANDLER)).is_ok());
        } else {
            assert!(!emu.step());
        }
        assert_eq!(emu.pc(), HANDLER);
        let record = argument(&emu, 1);
        let context = argument(&emu, 3);
        assert_eq!(dword(&emu, record), STATUS_SINGLE_STEP as u64);
        assert_eq!(dword(&emu, record + 12), breakpoint);
        assert_eq!(dword(&emu, context + CONTEXT_EIP), breakpoint);
        assert_eq!(dword(&emu, context + CONTEXT_DR6) & 0xf, 0b1001);
        assert_eq!(dword(&emu, context + DR7), BREAKPOINT_CONTROL as u64);
        assert_eq!(emu.regs().get_ebx(), starting_ebx);
        let flags = dword(&emu, context + CONTEXT_EFLAGS) as u32;
        emu.maps
            .write_dword(context + CONTEXT_EFLAGS, flags | RESUME_FLAG);
        step_until(&mut emu, breakpoint);
        assert!(emu.flags_ref().f_rf);
        if cached {
            assert!(emu.run(Some(breakpoint + 1)).is_ok());
        } else {
            assert!(emu.step());
        }
        assert_eq!(emu.pc(), breakpoint + 1);
        assert_eq!(emu.regs().get_ebx(), starting_ebx + 1);
        assert!(!emu.flags_ref().f_rf);
        emu.set_pc(breakpoint);
        assert!(!emu.step());
        assert_eq!(emu.pc(), HANDLER);
    }
}

#[test]
fn windows_instruction_faults_report_the_faulting_address() {
    for (instruction, status) in [
        ([0xe4, 0x36], STATUS_PRIVILEGED_INSTRUCTION), // IN AL, imm8
        ([0xe6, 0x36], STATUS_PRIVILEGED_INSTRUCTION), // OUT imm8, AL
        ([0x0f, 0x0b], STATUS_ILLEGAL_INSTRUCTION),    // UD2
    ] {
        let mut emu = emulator(&[0x90]);
        emu.maps.write_bytes(CODE, &instruction);
        let old_eax = emu.regs().get_eax();
        assert!(!emu.step());
        assert_eq!(emu.pc(), HANDLER);
        assert_eq!(emu.regs().get_eax(), old_eax);
        let record = argument(&emu, 1);
        assert_eq!(dword(&emu, record), status as u64);
        assert_eq!(dword(&emu, record + 12), CODE);
        assert_eq!(dword(&emu, argument(&emu, 3) + CONTEXT_EIP), CODE);
    }
}

#[test]
fn seh4_search_validates_cookies_and_uses_the_compilers_filter_frame() {
    use crate::api::windows::policy::{ApiDisposition, ApiStopReason};
    use crate::api::windows::winapi32;
    use crate::windows::constants::exceptions::ExceptionDisposition;
    use std::{cell::RefCell, rc::Rc};
    const TABLE: u64 = REGISTRATION + 0x200;
    const COOKIE: u64 = REGISTRATION + 0x300;
    const CHECK: u64 = CODE + 0x300;
    const FILTER: u64 = CODE + 0x400;
    let mut results = Vec::new();
    for (filter_result, good_cookie) in [(0_i32, true), (-1, true), (1, true), (0, false)] {
        let mut emu = emulator(&[0xc3]);
        let registration = emu.sp() + 0x40;
        let frame_pointer = registration + 16;
        let cookie = 0x3123_4abc;
        let saved_cookie = if good_cookie { cookie } else { !cookie };
        for (address, value) in [
            (registration, EXCEPTION_CHAIN_END),
            (registration + 4, HANDLER as u32),
            (registration + 8, TABLE as u32 ^ cookie),
            (registration + 12, 0),
            (TABLE, u32::MAX - 1),
            (TABLE + 4, 0),
            (TABLE + 8, (-32i32) as u32),
            (TABLE + 12, 0),
            (TABLE + 16, u32::MAX - 1),
            (TABLE + 20, FILTER as u32),
            (COOKIE, cookie),
            (frame_pointer - 32, saved_cookie ^ frame_pointer as u32),
        ] {
            assert!(emu.maps.write_dword(address, value));
        }
        emu.set_seh(registration);
        let mut check = vec![0x81, 0xf9]; // cmp ecx, cookie
        check.extend_from_slice(&cookie.to_le_bytes());
        check.extend_from_slice(&[0x74, 7, 0xb9, 2, 0, 0, 0, 0xcd, 0x29, 0xc3]); // je ret; mov ecx,2; int 29h; ret
        assert!(emu.maps.write_bytes(CHECK, &check));
        let mut code = vec![0xb8]; // mov eax, filter result; ret
        code.extend_from_slice(&filter_result.to_le_bytes());
        code.push(0xc3);
        assert!(emu.maps.write_bytes(FILTER, &code));
        let observations = Rc::new(RefCell::new(Vec::new()));
        let observed = observations.clone();
        emu.set_hook_on_pre_instruction(move |emu, pc, _, _| {
            if pc == CHECK {
                observed.borrow_mut().push((pc, emu.regs().rcx));
            }
            if pc == FILTER {
                assert_eq!(emu.regs().rbp, frame_pointer);
                let pointers = dword(emu, registration - 4);
                let record = dword(emu, pointers);
                assert_eq!(dword(emu, record), STATUS_ACCESS_VIOLATION);
                assert_eq!(dword(emu, pointers + 4), emu.eh_ctx());
                observed.borrow_mut().push((pc, frame_pointer));
            }
            true
        });
        dispatch(&mut emu);
        let args = [
            COOKIE,
            CHECK,
            argument(&emu, 1),
            registration,
            argument(&emu, 3),
            argument(&emu, 4),
        ];
        let sp = emu.sp();
        for arg in args.into_iter().rev() {
            assert!(emu.stack_push32(arg as u32));
        }
        let hook_sp = emu.sp();
        winapi32::gateway_by_import(&mut emu, "ntdll.dll", "_except_handler4_common", 0);
        if !good_cookie {
            assert!(emu.api_policy.stopped() || emu.process_terminated);
            assert_eq!(
                observations.borrow().as_slice(),
                &[(CHECK, u64::from(saved_cookie))]
            );
            continue;
        }
        assert_eq!(emu.sp(), hook_sp);
        if filter_result > 0 {
            let reason = &emu.api_policy.last_outcome.as_ref().unwrap().disposition;
            assert_eq!(
                *reason,
                ApiDisposition::Stopped(ApiStopReason::UnsupportedUnwind)
            );
            assert_eq!(
                observations.borrow().as_slice(),
                &[(CHECK, u64::from(cookie)), (FILTER, frame_pointer)]
            );
        } else {
            assert_eq!(
                observations.borrow().as_slice(),
                &[
                    (CHECK, u64::from(cookie)),
                    (FILTER, frame_pointer),
                    (CHECK, u64::from(cookie))
                ]
            );
            results.push(emu.regs().get_eax());
            // A normal cdecl return leaves the six arguments to the CRT stub.
            emu.set_sp(sp);
            assert!(emu.step());
            assert_eq!(emu.process_terminated, filter_result == 0);
        }
    }
    assert_eq!(
        results,
        [
            ExceptionDisposition::ContinueSearch as u64,
            ExceptionDisposition::ContinueExecution as u64
        ]
    );
}
