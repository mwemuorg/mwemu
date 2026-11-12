use crate::emu::Emu;
use crate::maps::mem64::Permission;
use crate::windows::constants::errors::STATUS_ACCESS_VIOLATION;
use crate::windows::constants::exceptions::*;
use crate::{emu32, tests::helpers};

const CODE: u64 = 0x400000;
const HANDLER: u64 = CODE + 0x100;
const NEXT_HANDLER: u64 = CODE + 0x200;
const REGISTRATION: u64 = 0x500000;
const NEXT_REGISTRATION: u64 = REGISTRATION + 8;
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
    assert_eq!(emu.regs().get_esp(), esp - 20);
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
        (WOW64_CONTEXT_CONTROL | WOW64_CONTEXT_INTEGER) as u64
    );
    assert_eq!(dword(&emu, context + CONTEXT_EIP), CODE);
    assert_eq!(dword(&emu, context + CONTEXT_ESP), esp);
    assert_eq!(dword(&emu, context + CONTEXT_EFLAGS), 0x246);
    assert_eq!(emu.seh(), REGISTRATION);
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
    assert_eq!(emu.regs().get_esp(), esp - 20);
    assert_eq!(emu.seh(), REGISTRATION);
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
