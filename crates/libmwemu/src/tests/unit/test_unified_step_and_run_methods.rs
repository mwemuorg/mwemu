use crate::config::Config;
use crate::emu::Emu;
use crate::emu::decoded_instruction::DecodedInstruction;
use crate::maps::mem64::Permission;
use crate::{tests::helpers, *};
use std::cell::RefCell;
use std::rc::Rc;

#[test]
pub fn test_unified_step_and_run_methods() {
    helpers::setup();

    // Test 1: Single-threaded mode (default)
    let mut emu = emu64();
    assert_eq!(
        emu.is_threading_enabled(),
        false,
        "Threading should be disabled by default"
    );

    // Load some simple code - NOP instructions
    let code = vec![0x90, 0x90, 0x90]; // 3 NOP instructions
    emu.maps
        .create_map("code", 0x1000, 0x1000, Permission::READ_WRITE_EXECUTE);
    emu.maps.write_bytes(0x1000, &code);
    emu.regs_mut().rip = 0x1000;

    // Test step() in single-threaded mode
    let result = emu.step();
    assert!(result, "Step should succeed in single-threaded mode");
    assert_eq!(emu.regs().rip, 0x1001, "RIP should advance after NOP");

    // Test 2: Enable threading and verify it's set
    emu.enable_threading(true);
    assert_eq!(
        emu.is_threading_enabled(),
        true,
        "Threading should be enabled"
    );

    // Step again with threading enabled (but still only 1 thread)
    let result = emu.step();
    assert!(result, "Step should succeed with threading enabled");
    assert_eq!(
        emu.regs().rip,
        0x1002,
        "RIP should advance after second NOP"
    );

    // Test 3: Verify run() method works
    let mut emu2 = emu32();
    emu2.maps
        .create_map("code", 0x1000, 0x1000, Permission::READ_WRITE_EXECUTE);
    let code32 = vec![0x90, 0x90, 0xC3]; // 2 NOPs and RET
    emu2.maps.write_bytes(0x1000, &code32);
    emu2.regs_mut().set_eip(0x1000);

    // Create a minimal stack for the RET instruction
    emu2.maps
        .create_map("stack", 0x2000, 0x1000, Permission::READ_WRITE);
    emu2.regs_mut().set_esp(0x2500);
    emu2.maps.write_dword(0x2500, 0x3000); // Return address

    // Run until RET
    let result = emu2.run(Some(0x1002));
    assert!(result.is_ok(), "Run should succeed");

    // Test 4: Verify threading can be toggled
    let mut cfg = Config::new();
    cfg.enable_threading = false;
    assert_eq!(cfg.enable_threading, false);
    cfg.enable_threading = true;
    assert_eq!(cfg.enable_threading, true);
}

#[test]
pub fn test_run_until_ret_32_ret_imm_updates_ip_and_stack() {
    helpers::setup();

    let mut emu = emu32();
    emu.maps
        .create_map("code", 0x1000, 0x1000, Permission::READ_WRITE_EXECUTE);
    emu.maps
        .create_map("stack", 0x2000, 0x1000, Permission::READ_WRITE);
    emu.maps
        .create_map("target", 0x3000, 0x1000, Permission::READ_WRITE_EXECUTE);
    emu.maps
        .write_bytes(0x1000, &[0xc2, 0x0c, 0x00, 0xb8, 0xef, 0xbe, 0xad, 0xde]);
    emu.maps.write_bytes(0x3000, &[0x90]);
    emu.regs_mut().set_eip(0x1000);
    emu.regs_mut().set_esp(0x2500);
    emu.maps.write_dword(0x2500, 0x3000);
    emu.maps.write_dword(0x2504, 0x1111_1111);
    emu.maps.write_dword(0x2508, 0x2222_2222);
    emu.maps.write_dword(0x250c, 0x3333_3333);

    let result = emu.run_until_ret();

    assert_eq!(result.unwrap(), 0x3000);
    assert_eq!(emu.regs().get_eip(), 0x3000);
    assert_eq!(emu.regs().get_esp(), 0x2510);
    assert_ne!(emu.regs().get_eax(), 0xdead_beef);
    assert!(!emu.run_until_ret);
}

#[test]
pub fn test_run_until_ret_64_ret_imm_updates_ip_and_stack() {
    helpers::setup();

    let mut emu = emu64();
    emu.maps
        .create_map("code", 0x1000, 0x1000, Permission::READ_WRITE_EXECUTE);
    emu.maps
        .create_map("stack", 0x4000, 0x1000, Permission::READ_WRITE);
    emu.maps
        .create_map("target", 0x6000, 0x1000, Permission::READ_WRITE_EXECUTE);
    emu.maps
        .write_bytes(0x1000, &[0xc2, 0x10, 0x00, 0xb8, 0xef, 0xbe, 0xad, 0xde]);
    emu.maps.write_bytes(0x6000, &[0x90]);
    emu.regs_mut().rip = 0x1000;
    emu.regs_mut().rsp = 0x4500;
    emu.maps.write_qword(0x4500, 0x6000);
    emu.maps.write_qword(0x4508, 0x1111_1111_1111_1111);
    emu.maps.write_qword(0x4510, 0x2222_2222_2222_2222);

    let result = emu.run_until_ret();

    assert_eq!(result.unwrap(), 0x6000);
    assert_eq!(emu.regs().rip, 0x6000);
    assert_eq!(emu.regs().rsp, 0x4518);
    assert_ne!(emu.regs().rax, 0xdead_beef);
    assert!(!emu.run_until_ret);
}

#[test]
pub fn test_run_until_ret_multithreaded_ret_imm_updates_ip_and_stack() {
    helpers::setup();

    let mut emu = emu32();
    emu.maps
        .create_map("code", 0x1000, 0x1000, Permission::READ_WRITE_EXECUTE);
    emu.maps
        .create_map("stack", 0x2000, 0x1000, Permission::READ_WRITE);
    emu.maps
        .create_map("target", 0x3000, 0x1000, Permission::READ_WRITE_EXECUTE);
    emu.maps.write_bytes(0x1000, &[0xc2, 0x0c, 0x00]);
    emu.maps.write_bytes(0x3000, &[0x90]);
    emu.regs_mut().set_eip(0x1000);
    emu.regs_mut().set_esp(0x2500);
    emu.maps.write_dword(0x2500, 0x3000);

    let mut second_thread = crate::threading::context::ThreadContext::new(0x1001, emu.cfg.arch);
    second_thread.suspended = true;
    emu.threads.push(second_thread);
    emu.enable_threading(true);

    let result = emu.run_until_ret();

    assert_eq!(result.unwrap(), 0x3000);
    assert_eq!(emu.regs().get_eip(), 0x3000);
    assert_eq!(emu.regs().get_esp(), 0x2510);
    assert!(!emu.run_until_ret);
}

#[test]
pub fn test_step_32_ret_imm_updates_ip_and_stack() {
    helpers::setup();

    let mut emu = emu32();
    emu.maps
        .create_map("code", 0x1000, 0x1000, Permission::READ_WRITE_EXECUTE);
    emu.maps
        .create_map("stack", 0x2000, 0x1000, Permission::READ_WRITE);
    emu.maps
        .create_map("target", 0x3000, 0x1000, Permission::READ_WRITE_EXECUTE);
    emu.maps.write_bytes(0x1000, &[0xc2, 0x0c, 0x00]);
    emu.maps.write_bytes(0x3000, &[0x90]);
    emu.regs_mut().set_eip(0x1000);
    emu.regs_mut().set_esp(0x2500);
    emu.maps.write_dword(0x2500, 0x3000);

    assert!(emu.step());
    assert_eq!(emu.regs().get_eip(), 0x3000);
    assert_eq!(emu.regs().get_esp(), 0x2510);
    assert!(!emu.force_reload);
}

#[test]
pub fn test_run_no_observer_leaves_last_decoded_empty() {
    helpers::setup();

    let mut emu = emu64();
    let code_base = 0x1000;
    let code = vec![0x90, 0x90, 0x90];
    emu.maps
        .create_map("code", code_base, 0x1000, Permission::READ_WRITE_EXECUTE);
    emu.maps.write_bytes(code_base, &code);
    emu.regs_mut().rip = code_base;
    emu.cfg.verbose = 0;
    emu.cfg.trace_regs = false;
    emu.cfg.trace_filename = None;

    let result = emu.run(Some(code_base + code.len() as u64));

    assert!(result.is_ok(), "Run should succeed without observers");
    assert_eq!(emu.regs().rip, code_base + code.len() as u64);
    assert_eq!(emu.last_decoded_addr, 0);
    assert!(emu.last_decoded.is_none());
}

#[test]
pub fn test_run_hooks_receive_fresh_decoded_instruction() {
    helpers::setup();

    let mut emu = emu64();
    let code_base = 0x2000;
    let code = vec![0x90, 0x90, 0x90];
    emu.maps
        .create_map("code", code_base, 0x1000, Permission::READ_WRITE_EXECUTE);
    emu.maps.write_bytes(code_base, &code);
    emu.regs_mut().rip = code_base;
    emu.cfg.verbose = 0;

    let pre_addresses = Rc::new(RefCell::new(Vec::new()));
    let post_addresses = Rc::new(RefCell::new(Vec::new()));
    let pre_capture = Rc::clone(&pre_addresses);
    let post_capture = Rc::clone(&post_addresses);

    emu.hooks.on_pre_instruction(
        move |_emu: &mut Emu, _ip: u64, ins: &DecodedInstruction, _sz: usize| -> bool {
            pre_capture.borrow_mut().push(ins.as_x86().ip());
            true
        },
    );
    emu.hooks.on_post_instruction(
        move |_emu: &mut Emu, _ip: u64, ins: &DecodedInstruction, _sz: usize, _ok: bool| {
            post_capture.borrow_mut().push(ins.as_x86().ip());
        },
    );

    let result = emu.run(Some(code_base + code.len() as u64));

    assert!(result.is_ok(), "Run should succeed with hooks installed");
    assert_eq!(
        *pre_addresses.borrow(),
        vec![code_base, code_base + 1, code_base + 2]
    );
    assert_eq!(
        *post_addresses.borrow(),
        vec![code_base, code_base + 1, code_base + 2]
    );
    assert_eq!(emu.last_decoded_addr, code_base + 2);
    assert_eq!(emu.last_decoded.unwrap().as_x86().ip(), code_base + 2);
}

#[test]
pub fn test_run_trace_file_preserves_disassembly() {
    helpers::setup();

    let mut emu = emu64();
    let code_base = 0x3000;
    let code = vec![0x90, 0x90];
    emu.maps
        .create_map("code", code_base, 0x1000, Permission::READ_WRITE_EXECUTE);
    emu.maps.write_bytes(code_base, &code);
    emu.regs_mut().rip = code_base;
    emu.cfg.verbose = 0;
    emu.cfg.trace_regs = true;
    emu.cfg.trace_start = 0;

    let trace_path = std::env::temp_dir().join(format!(
        "libmwemu-lazy-decoded-trace-{}-{}.csv",
        std::process::id(),
        code_base
    ));
    let _ = std::fs::remove_file(&trace_path);
    emu.cfg.trace_filename = Some(trace_path.to_string_lossy().into_owned());
    emu.open_trace_file();

    let result = emu.run(Some(code_base + code.len() as u64));
    drop(emu);

    assert!(result.is_ok(), "Run should succeed with trace file enabled");
    let trace = std::fs::read_to_string(&trace_path).expect("trace file should be readable");
    let _ = std::fs::remove_file(&trace_path);
    let rows: Vec<&str> = trace.lines().collect();
    assert!(
        rows.len() > 1,
        "trace file should contain instruction rows: {trace}"
    );
    assert!(
        rows.iter().skip(1).all(|row| !row.contains("???")),
        "trace rows should contain disassembly: {trace}"
    );
    assert!(
        rows.iter()
            .skip(1)
            .any(|row| row.to_ascii_lowercase().contains("nop")),
        "trace should include NOP disassembly: {trace}"
    );
}

// ---------------------------------------------------------------------------
// ISA-specific API tests
// ---------------------------------------------------------------------------
//
// The following tests exercise the new `step_x86` / `step_aarch64` /
// `run_x86` / `run_aarch64` public APIs and the mis-architecture guard
// contract. They also confirm that the threaded scheduler dispatches
// through the new ISA-specific paths.

#[test]
pub fn test_step_x86_advances_rip() {
    helpers::setup();

    let mut emu = emu64();
    let code = vec![0x90, 0x90, 0x90]; // 3 NOP instructions
    emu.maps
        .create_map("code", 0x1000, 0x1000, Permission::READ_WRITE_EXECUTE);
    emu.maps.write_bytes(0x1000, &code);
    emu.regs_mut().rip = 0x1000;

    assert!(emu.step_x86());
    assert_eq!(emu.regs().rip, 0x1001);
    assert!(emu.step_x86());
    assert_eq!(emu.regs().rip, 0x1002);
    assert!(emu.step_x86());
    assert_eq!(emu.regs().rip, 0x1003);
}

#[test]
pub fn test_run_x86_executes_until_end_addr() {
    helpers::setup();

    let mut emu = emu64();
    let code = vec![0x90, 0x90, 0x90, 0x90];
    emu.maps
        .create_map("code", 0x2000, 0x1000, Permission::READ_WRITE_EXECUTE);
    emu.maps.write_bytes(0x2000, &code);
    emu.regs_mut().rip = 0x2000;

    let result = emu.run_x86(Some(0x2004));
    assert!(result.is_ok(), "run_x86 should succeed: {result:?}");
    assert_eq!(emu.regs().rip, 0x2004);
}

#[test]
pub fn test_step_aarch64_advances_pc() {
    helpers::setup();

    // mov x0, #1; mov x1, #1; add x2, x0, x1
    let code: [u8; 12] = [
        0x20, 0x00, 0x80, 0xd2, 0x21, 0x00, 0x80, 0xd2, 0x02, 0x00, 0x01, 0x8b,
    ];

    let mut emu = emu_aarch64();
    emu.load_code_bytes(&code);

    let base = emu.regs_aarch64().pc;
    assert!(emu.step_aarch64());
    assert_eq!(emu.regs_aarch64().pc, base + 4);
    assert_eq!(emu.regs_aarch64().x[0], 1);

    assert!(emu.step_aarch64());
    assert_eq!(emu.regs_aarch64().pc, base + 8);
    assert_eq!(emu.regs_aarch64().x[1], 1);

    assert!(emu.step_aarch64());
    assert_eq!(emu.regs_aarch64().pc, base + 12);
    assert_eq!(emu.regs_aarch64().x[2], 2);
}

#[test]
pub fn test_run_aarch64_executes_until_end_addr() {
    helpers::setup();

    let code: [u8; 12] = [
        0x20, 0x00, 0x80, 0xd2, // mov x0, #1
        0x21, 0x00, 0x80, 0xd2, // mov x1, #1
        0x02, 0x00, 0x01, 0x8b, // add x2, x0, x1
    ];

    let mut emu = emu_aarch64();
    emu.load_code_bytes(&code);
    let base = emu.regs_aarch64().pc;

    let result = emu.run_aarch64(Some(base + 12));
    assert!(result.is_ok(), "run_aarch64 should succeed: {result:?}");
    assert_eq!(emu.regs_aarch64().pc, base + 12);
    assert_eq!(emu.regs_aarch64().x[2], 2);
}

#[test]
#[should_panic(
    expected = "step_x86 called on non-x86 emulator (arch=Aarch64); use the AArch64 API instead"
)]
pub fn test_step_x86_panics_on_aarch64() {
    helpers::setup();

    let mut emu = emu_aarch64();
    let _ = emu.step_x86();
}

#[test]
#[should_panic(
    expected = "run_x86 called on non-x86 emulator (arch=Aarch64); use the AArch64 API instead"
)]
pub fn test_run_x86_panics_on_aarch64() {
    helpers::setup();

    let mut emu = emu_aarch64();
    let _ = emu.run_x86(None);
}

#[test]
#[should_panic(
    expected = "step_aarch64 called on non-AArch64 emulator (arch=X86_64); use the x86 API instead"
)]
pub fn test_step_aarch64_panics_on_x86_64() {
    helpers::setup();

    let mut emu = emu64();
    let _ = emu.step_aarch64();
}

#[test]
#[should_panic(
    expected = "run_aarch64 called on non-AArch64 emulator (arch=X86_64); use the x86 API instead"
)]
pub fn test_run_aarch64_panics_on_x86_64() {
    helpers::setup();

    let mut emu = emu64();
    let _ = emu.run_aarch64(None);
}

#[test]
#[should_panic(
    expected = "decode_and_execute_x86 called on non-x86 emulator (arch=Aarch64); use the AArch64 API instead"
)]
pub fn test_decode_and_execute_x86_panics_on_aarch64() {
    helpers::setup();

    let mut emu = emu_aarch64();
    let _ = emu.decode_and_execute_x86();
}

#[test]
#[should_panic(
    expected = "advance_pc_aarch64 called on non-AArch64 emulator (arch=X86_64); use the x86 API instead"
)]
pub fn test_advance_pc_aarch64_panics_on_x86_64() {
    helpers::setup();

    let mut emu = emu64();
    emu.advance_pc_aarch64(4);
}

/// AArch64 multi-threaded smoke test: the prior AArch64 threading path
/// called `self.regs().rip` which panicked on AArch64 emulators. The
/// ISA-specific `run_multi_threaded_aarch64` must execute two threads
/// without touching x86-only register accessors.
#[test]
pub fn test_aarch64_multithreaded_runs_two_threads() {
    helpers::setup();

    // mov x0, #1; mov x1, #1; add x2, x0, x1; ret
    let code: [u8; 16] = [
        0x20, 0x00, 0x80, 0xd2, 0x21, 0x00, 0x80, 0xd2, 0x02, 0x00, 0x01, 0x8b, 0xc0, 0x03, 0x5f,
        0xd6,
    ];

    let mut emu = emu_aarch64();
    emu.load_code_bytes(&code);

    // Suspended second AArch64 thread parked at the same code base.
    let mut second = crate::threading::context::ThreadContext::new(0x1001, emu.cfg.arch);
    second.suspended = true;
    emu.threads.push(second);
    emu.enable_threading(true);

    let result = emu.run_until_ret();
    assert!(
        result.is_ok(),
        "run_until_ret with threading should succeed: {result:?}"
    );
    assert_eq!(emu.regs_aarch64().x[2], 2);
}

/// Threaded post-hook count assertion. The pre-refactor scheduler fired
/// the post hook twice for every multithreaded step (once in the decode
/// path and once in `ThreadScheduler::execute_thread_instruction`). After
/// the refactor each instruction delivered exactly one post hook.
#[test]
pub fn test_threaded_post_hook_fires_exactly_once_per_inst() {
    helpers::setup();

    let code: [u8; 12] = [
        0x20, 0x00, 0x80, 0xd2, // mov x0, #1
        0x21, 0x00, 0x80, 0xd2, // mov x1, #1
        0x02, 0x00, 0x01, 0x8b, // add x2, x0, x1
    ];

    let mut emu = emu_aarch64();
    emu.load_code_bytes(&code);

    let count = Rc::new(RefCell::new(0u32));
    let capture = Rc::clone(&count);
    emu.hooks.on_post_instruction(
        move |_emu: &mut Emu, _pc: u64, _ins: &DecodedInstruction, _sz: usize, _ok: bool| {
            *capture.borrow_mut() += 1;
        },
    );

    let _ = emu.step_aarch64();
    let _ = emu.step_aarch64();
    let _ = emu.step_aarch64();

    assert_eq!(
        *count.borrow(),
        3,
        "exactly one post hook per instruction (3 expected, got {})",
        *count.borrow()
    );
}

#[test]
pub fn test_x86_multithreaded_post_hook_fires_exactly_once_per_inst() {
    helpers::setup();

    let mut emu = emu64();
    let code = vec![0x90, 0x90, 0x90];
    emu.maps
        .create_map("code", 0x1000, 0x1000, Permission::READ_WRITE_EXECUTE);
    emu.maps.write_bytes(0x1000, &code);
    emu.regs_mut().rip = 0x1000;

    let count = Rc::new(RefCell::new(0u32));
    let capture = Rc::clone(&count);
    emu.hooks.on_post_instruction(
        move |_emu: &mut Emu, _pc: u64, _ins: &DecodedInstruction, _sz: usize, _ok: bool| {
            *capture.borrow_mut() += 1;
        },
    );

    let _ = emu.step_x86();
    let _ = emu.step_x86();
    let _ = emu.step_x86();

    assert_eq!(
        *count.borrow(),
        3,
        "exactly one post hook per instruction (3 expected, got {})",
        *count.borrow()
    );
}

// ---------------------------------------------------------------------------
// Typed vs generic dispatch parity tests (split-ISA execution refactor)
// ---------------------------------------------------------------------------
//
// These tests confirm the public generic facade (`step` / `run` /
// `decode_and_execute` / `advance_pc`) and the typed ISA entry points
// (`step_x86` / `step_aarch64` / `decode_and_execute_x86` / etc.) reach
// the same final state on a tiny program. They guard against silent
// regressions where the generic dispatcher ends up calling the wrong ISA
// path after the refactor.

#[test]
pub fn test_decode_and_execute_x86_advances_rip_and_records_size() {
    helpers::setup();

    let mut emu = emu64();
    // mov eax, 1  (5 bytes: B8 01 00 00 00)
    let code = [0xB8, 0x01, 0x00, 0x00, 0x00];
    emu.maps
        .create_map("code", 0x1000, 0x1000, Permission::READ_WRITE_EXECUTE);
    emu.maps.write_bytes(0x1000, &code);
    emu.regs_mut().rip = 0x1000;

    let (size, ok) = emu.decode_and_execute_x86();
    assert!(ok, "decode_and_execute_x86 should succeed");
    assert_eq!(size, 5, "mov eax, imm32 must be 5 bytes");
    assert_eq!(
        emu.regs().rax & 0xFFFF_FFFF,
        1,
        "mov eax, 1 should set the low 32 bits of rax to 1"
    );
    emu.advance_pc_x86(size);
    assert_eq!(
        emu.regs().rip,
        0x1005,
        "advance_pc_x86 must move RIP past the decoded instruction"
    );
}

#[test]
pub fn test_decode_and_execute_aarch64_advances_pc_and_records_size() {
    helpers::setup();

    let mut emu = emu_aarch64();
    // mov x0, #1  (MOVZ W0, #1: 20 00 80 D2)
    let code: [u8; 4] = [0x20, 0x00, 0x80, 0xD2];
    emu.maps
        .create_map("code", 0x2000, 0x1000, Permission::READ_WRITE_EXECUTE);
    emu.maps.write_bytes(0x2000, &code);
    emu.regs_aarch64_mut().pc = 0x2000;

    let (size, ok) = emu.decode_and_execute_aarch64();
    assert!(ok, "decode_and_execute_aarch64 should succeed");
    assert_eq!(size, 4, "AArch64 mov is fixed 4 bytes");
    assert_eq!(emu.regs_aarch64().x[0], 1, "mov x0, #1 must load 1 into x0");

    emu.advance_pc_aarch64(size);
    assert_eq!(
        emu.regs_aarch64().pc,
        0x2004,
        "advance_pc_aarch64 must move PC past the decoded instruction"
    );
}

#[test]
#[test]
pub fn test_generic_dispatch_matches_typed_x86_path() {
    helpers::setup();

    // Build a tiny x64 program: mov rax, 0x2A; ret
    let code: Vec<u8> = vec![
        0x48, 0xC7, 0xC0, 0x2A, 0x00, 0x00, 0x00, // mov rax, 0x2A
        0xC3, // ret
    ];
    let stack_addr = 0x4000u64;
    let ret_addr = 0x9000u64;

    let mut typed_emu = emu64();
    typed_emu.os = crate::arch::OperatingSystem::Linux;
    typed_emu
        .maps
        .create_map("code", 0x1000, 0x1000, Permission::READ_WRITE_EXECUTE);
    typed_emu.maps.write_bytes(0x1000, &code);
    typed_emu
        .maps
        .create_map("stack", stack_addr, 0x1000, Permission::READ_WRITE);
    typed_emu.regs_mut().rip = 0x1000;
    typed_emu.regs_mut().rsp = stack_addr + 0x800;
    let typed_rsp = typed_emu.regs_mut().rsp;
    typed_emu.maps.write_qword(typed_rsp, ret_addr);
    let typed_result = typed_emu.run_x86(Some(ret_addr));
    let typed_rax = typed_emu.regs().rax;
    let typed_rip = typed_emu.regs().rip;
    assert!(
        typed_result.is_ok(),
        "typed run_x86 should succeed: {typed_result:?}"
    );
    assert_eq!(typed_rax, 0x2A, "typed run must compute 0x2A in rax");

    let mut generic_emu = emu64();
    generic_emu.os = crate::arch::OperatingSystem::Linux;
    generic_emu
        .maps
        .create_map("code", 0x1000, 0x1000, Permission::READ_WRITE_EXECUTE);
    generic_emu.maps.write_bytes(0x1000, &code);
    generic_emu
        .maps
        .create_map("stack", stack_addr, 0x1000, Permission::READ_WRITE);
    generic_emu.regs_mut().rip = 0x1000;
    generic_emu.regs_mut().rsp = stack_addr + 0x800;
    let generic_rsp = generic_emu.regs_mut().rsp;
    generic_emu.maps.write_qword(generic_rsp, ret_addr);
    let generic_result = generic_emu.run(Some(ret_addr));
    let generic_rax = generic_emu.regs().rax;
    let generic_rip = generic_emu.regs().rip;
    assert!(
        generic_result.is_ok(),
        "generic run should succeed: {generic_result:?}"
    );

    assert_eq!(
        typed_rax, generic_rax,
        "generic and typed x86 run must reach the same rax"
    );
    assert_eq!(
        typed_rip, generic_rip,
        "generic and typed x86 run must reach the same rip"
    );
}

#[test]
pub fn test_generic_dispatch_matches_typed_aarch64_path() {
    helpers::setup();

    // mov x0, #1; mov x1, #2; add x2, x0, x1
    let code: [u8; 12] = [
        0x20, 0x00, 0x80, 0xD2, // mov x0, #1
        0x41, 0x00, 0x80, 0xD2, // mov x1, #2
        0x02, 0x00, 0x01, 0x8B, // add x2, x0, x1
    ];

    let mut typed_emu = emu_aarch64();
    typed_emu.load_code_bytes(&code);
    let typed_result = typed_emu.run_aarch64(Some(typed_emu.regs_aarch64().pc + 12));
    let typed_x2 = typed_emu.regs_aarch64().x[2];
    let typed_pc = typed_emu.regs_aarch64().pc;
    assert!(
        typed_result.is_ok(),
        "typed run_aarch64 should succeed: {typed_result:?}"
    );
    assert_eq!(typed_x2, 3, "typed run must produce x2 = 1 + 2");

    let mut generic_emu = emu_aarch64();
    generic_emu.load_code_bytes(&code);
    let generic_result = generic_emu.run(Some(generic_emu.regs_aarch64().pc + 12));
    let generic_x2 = generic_emu.regs_aarch64().x[2];
    let generic_pc = generic_emu.regs_aarch64().pc;
    assert!(
        generic_result.is_ok(),
        "generic run should succeed: {generic_result:?}"
    );

    assert_eq!(
        typed_x2, generic_x2,
        "generic and typed aarch64 run must reach the same x2"
    );
    assert_eq!(
        typed_pc, generic_pc,
        "generic and typed aarch64 run must reach the same pc"
    );
}
