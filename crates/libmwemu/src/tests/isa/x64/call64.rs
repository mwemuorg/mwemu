use itertools::iproduct;

use crate::api::windows::policy::ApiMode;
use crate::emu::Emu;
use crate::emu64;
use crate::maps::mem64::Permission;
use crate::tests::helpers;
use crate::threading::context::ThreadContext;

const CODE: u64 = 0x400000;
const RETURN_ADDRESS: u64 = CODE + 0x100;
const STACK_BASE: u64 = 0x200000;
const STACK_SIZE: u64 = 0x1000;
const STACK_TOP: u64 = STACK_BASE + STACK_SIZE;
const STACK_ALIGNMENT: u64 = 16;
const RETURN_SLOT_SIZE: u64 = 8;
const SHADOW_SPACE: u64 = 32;
const REGISTER_ARGUMENTS: usize = 4;
const ARGUMENTS: [u64; 8] = [
    0x11,
    0x2233,
    0x445566,
    0x778899aa,
    0x123456789,
    0xabcdef012,
    0x13579bdf,
    0xfedcba9876543210,
];

const RETURN: &[u8] = &[0xc3]; // ret
const NOP_RETURN: &[u8] = &[0x90, 0xc3]; // nop; ret
const HOME_AND_ADD_STACK_ARGS: &[u8] = &[
    0x48, 0x89, 0x4c, 0x24, 0x08, // mov [rsp + 8], rcx
    0x48, 0x89, 0x54, 0x24, 0x10, // mov [rsp + 16], rdx
    0x4c, 0x89, 0x44, 0x24, 0x18, // mov [rsp + 24], r8
    0x4c, 0x89, 0x4c, 0x24, 0x20, // mov [rsp + 32], r9
    0x48, 0x8b, 0x44, 0x24, 0x28, // mov rax, [rsp + 40]
    0x48, 0x03, 0x44, 0x24, 0x30, // add rax, [rsp + 48]
    0xc3, // ret
];

fn prepare(code: &[u8]) -> Emu {
    helpers::setup();
    let mut emu = emu64();
    emu.cfg.console_enabled = false;
    emu.cfg.max_instructions = Some(100);
    emu.maps
        .create_map("code", CODE, 0x1000, Permission::READ_WRITE_EXECUTE)
        .unwrap();
    emu.maps
        .create_map("stack", STACK_BASE, STACK_SIZE, Permission::READ_WRITE)
        .unwrap();
    emu.maps.write_bytes(CODE, code);
    emu.regs_mut().rip = RETURN_ADDRESS;
    emu.regs_mut().rsp = STACK_TOP;
    emu
}

#[test]
fn integer_arguments_follow_microsoft_x64_layout() {
    for count in 0..=ARGUMENTS.len() {
        let mut emu = prepare(RETURN);
        emu.set_hook_on_pre_instruction(move |emu, pc, _, _| {
            assert_eq!(pc, CODE);
            let sp = emu.regs().rsp;
            assert_eq!(emu.maps.read_qword(sp), Some(RETURN_ADDRESS));
            let registers = [emu.regs().rcx, emu.regs().rdx, emu.regs().r8, emu.regs().r9];
            assert_eq!(
                &registers[..count.min(REGISTER_ARGUMENTS)],
                &ARGUMENTS[..count.min(REGISTER_ARGUMENTS)]
            );
            for (index, expected) in ARGUMENTS[..count]
                .iter()
                .skip(REGISTER_ARGUMENTS)
                .enumerate()
            {
                let address =
                    sp + RETURN_SLOT_SIZE + SHADOW_SPACE + index as u64 * RETURN_SLOT_SIZE;
                assert_eq!(
                    emu.maps.read_qword(address),
                    Some(*expected),
                    "argument {}",
                    index + REGISTER_ARGUMENTS + 1
                );
            }
            true
        });
        emu.call64(CODE, &ARGUMENTS[..count]).unwrap();
        assert_eq!(emu.pos, 1);
        assert_eq!(emu.regs().rip, RETURN_ADDRESS);
        assert_eq!(emu.regs().rsp, STACK_TOP);
    }
}

#[test]
fn entry_stack_is_aligned_after_the_return_address_push() {
    for (count, displacement) in iproduct!(0..=ARGUMENTS.len(), [0, 8, 3]) {
        let mut emu = prepare(RETURN);
        let original_sp = STACK_TOP - displacement;
        emu.regs_mut().rsp = original_sp;
        emu.set_hook_on_pre_instruction(move |emu, _, _, _| {
            let sp = emu.regs().rsp;
            assert_eq!(
                sp % STACK_ALIGNMENT,
                RETURN_SLOT_SIZE,
                "count={count}, displacement={displacement}"
            );
            assert!(
                emu.maps
                    .validate_write_range(sp + RETURN_SLOT_SIZE, SHADOW_SPACE)
            );
            assert!(sp + RETURN_SLOT_SIZE + SHADOW_SPACE <= original_sp);
            true
        });
        emu.call64(CODE, &ARGUMENTS[..count]).unwrap();
        assert_eq!(emu.regs().rsp, original_sp);
    }
}

#[test]
fn homing_registers_preserves_stack_arguments() {
    let mut emu = prepare(HOME_AND_ADD_STACK_ARGS);
    let original_sp = STACK_TOP - STACK_ALIGNMENT;
    let canary = 0xfeedface12345678;
    emu.regs_mut().rsp = original_sp;
    assert!(emu.maps.write_qword(original_sp, canary));
    assert_eq!(
        emu.call64(CODE, &ARGUMENTS[..6]).unwrap(),
        ARGUMENTS[4] + ARGUMENTS[5]
    );
    assert_eq!(emu.regs().rsp, original_sp);
    assert_eq!(emu.maps.read_qword(original_sp), Some(canary));
}

#[test]
fn failed_frame_setup_preserves_registers() {
    for stack_pointer in [SHADOW_SPACE, STACK_BASE + SHADOW_SPACE] {
        let mut emu = prepare(RETURN);
        if stack_pointer >= STACK_BASE {
            emu.maps
                .get_map_by_name_mut("stack")
                .unwrap()
                .set_permission(Permission::READ);
        }
        emu.regs_mut().rsp = stack_pointer;
        emu.regs_mut().rip = CODE;
        emu.regs_mut().rcx = 0xface;
        let registers = *emu.regs();
        assert!(emu.call64(CODE, &ARGUMENTS).is_err());
        assert_eq!(emu.regs().rsp, registers.rsp);
        assert_eq!(emu.regs().rip, registers.rip);
        assert_eq!(emu.regs().rcx, registers.rcx);
        assert_eq!(emu.pos, 0);
    }
}

#[test]
fn instruction_limit_is_not_a_successful_host_return() {
    let mut emu = prepare(NOP_RETURN);
    emu.cfg.max_instructions = Some(1);
    emu.force_reload = true;
    emu.force_break = true;
    let error = emu.call64(CODE, &[]).unwrap_err();
    assert!(error.message.contains("host call stopped"), "{error}");
    assert_eq!(emu.regs().rip, CODE);
    assert!(emu.regs().rsp < STACK_TOP);
    assert_eq!(emu.call_depth, 0);
    assert!(emu.force_reload);
    assert!(emu.force_break);
}

#[test]
fn execution_errors_are_preserved() {
    let mut emu = prepare(RETURN);
    let unmapped = CODE + 0x2000;
    let error = emu.call64(unmapped, &[]).unwrap_err();
    assert!(!error.message.contains("host call stopped"), "{error}");
    assert_eq!(emu.regs().rip, unmapped);
    assert!(emu.regs().rsp < STACK_TOP);
    assert_eq!(emu.call_depth, 0);
}

#[test]
fn termination_at_the_return_address_is_not_success() {
    let mut emu = prepare(RETURN);
    emu.set_hook_on_post_instruction(|emu, _, _, _, _| emu.stop());
    let error = emu.call64(CODE, &[]).unwrap_err();
    assert!(error.message.contains("host call stopped"), "{error}");
    assert!(emu.process_terminated);
    assert_eq!(emu.regs().rip, RETURN_ADDRESS);
    assert!(emu.regs().rsp < STACK_TOP);
}

#[test]
fn api_policy_stop_is_preserved() {
    let mut emu = prepare(NOP_RETURN);
    emu.set_api_mode(ApiMode::Strict);
    emu.set_hook_on_pre_instruction(|emu, _, _, _| {
        helpers::invoke_winapi_export(emu, "ntdll.dll", "NtSetInformationProcess");
        true
    });
    let error = emu.call64(CODE, &[]).unwrap_err();
    assert!(emu.api_policy.stopped());
    assert!(!error.message.contains("host call stopped"), "{error}");
    assert_eq!(emu.call_depth, 0);
}

#[test]
fn same_entry_call_returns_without_terminating_the_process() {
    for threaded in [false, true] {
        let mut emu = prepare(RETURN);
        if threaded {
            let mut second = ThreadContext::new(0x1001, emu.cfg.arch);
            second.suspended = true;
            emu.threads.push(second);
            emu.enable_threading(true);
        }
        emu.regs_mut().rip = CODE;
        emu.regs_mut().rax = 0x1234;
        assert_eq!(emu.call64(CODE, &[]).unwrap(), 0x1234);
        assert_eq!(emu.regs().rip, 0);
        assert_eq!(emu.regs().rsp, STACK_TOP);
        assert!(!emu.process_terminated);
        assert_eq!(emu.call_depth, 0);
    }
}

#[test]
fn nested_host_call_restores_the_outer_return_target() {
    let mut emu = prepare(RETURN);
    let inner = CODE + 0x80;
    emu.maps.write_bytes(inner, RETURN);
    emu.regs_mut().rip = CODE;
    emu.set_hook_on_pre_instruction(move |emu, _, _, _| {
        let stack_pointer = emu.regs().rsp;
        emu.call64(inner, &[]).unwrap();
        assert_eq!(emu.regs().rsp, stack_pointer);
        assert_eq!(emu.regs().rip, CODE);
        true
    });
    emu.call64(CODE, &[]).unwrap();
    assert_eq!(emu.regs().rip, 0);
    assert_eq!(emu.regs().rsp, STACK_TOP);
    assert_eq!(emu.pos, 2);
    assert!(!emu.process_terminated);
}

#[test]
fn frame_setup_preserves_stack_growth() {
    let mut emu = prepare(HOME_AND_ADD_STACK_ARGS);
    let original_sp = STACK_BASE + RETURN_SLOT_SIZE;
    emu.regs_mut().rsp = original_sp;
    assert_eq!(
        emu.call64(CODE, &ARGUMENTS[..6]).unwrap(),
        ARGUMENTS[4] + ARGUMENTS[5]
    );
    assert_eq!(emu.regs().rsp, original_sp);
    assert!(emu.maps.is_mapped(STACK_BASE - RETURN_SLOT_SIZE));
}

#[test]
fn unmapped_return_target_is_a_valid_host_sentinel() {
    let mut emu = prepare(RETURN);
    let unmapped_return = CODE + 0x2000;
    emu.regs_mut().rip = unmapped_return;
    emu.call64(CODE, &[]).unwrap();
    assert_eq!(emu.regs().rip, unmapped_return);
    assert_eq!(emu.regs().rsp, STACK_TOP);
    assert!(!emu.process_terminated);
}
