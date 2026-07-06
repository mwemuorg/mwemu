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
