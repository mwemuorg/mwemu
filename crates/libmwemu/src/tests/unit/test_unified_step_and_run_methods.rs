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
