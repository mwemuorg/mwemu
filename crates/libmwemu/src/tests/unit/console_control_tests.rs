#[test]
fn test_console_enabled_by_default() {
    let emu = crate::emu64();

    // Console settings should have default values
    // The enabled_ctrlc field tracks Ctrl+C handling
    assert!(
        !emu.enabled_ctrlc,
        "Ctrl+C should be disabled by default due to disable_ctrlc() call"
    );
}

#[test]
fn test_enable_ctrlc() {
    let mut emu = crate::emu64();

    emu.enable_ctrlc();

    assert!(
        emu.enabled_ctrlc,
        "Ctrl+C should be enabled after enable_ctrlc()"
    );
}

#[test]
fn test_disable_ctrlc() {
    let mut emu = crate::emu64();

    // First enable it
    emu.enable_ctrlc();
    assert!(emu.enabled_ctrlc, "Ctrl+C should be enabled");

    // Then disable it
    emu.disable_ctrlc();

    assert!(
        !emu.enabled_ctrlc,
        "Ctrl+C should be disabled after disable_ctrlc()"
    );
}

#[test]
fn test_ctrlc_toggle() {
    let mut emu = crate::emu64();

    emu.enable_ctrlc();
    assert!(emu.enabled_ctrlc);

    emu.disable_ctrlc();
    assert!(!emu.enabled_ctrlc);

    emu.enable_ctrlc();
    assert!(emu.enabled_ctrlc);

    emu.disable_ctrlc();
    assert!(!emu.enabled_ctrlc);
}

#[test]
fn test_console_enable() {
    let mut emu = crate::emu64();

    emu.enable_console();

    // Console should be enabled
    assert!(
        emu.cfg.console_enabled,
        "Console should be enabled after enable_console()"
    );
}

#[test]
fn test_console_disable() {
    let mut emu = crate::emu64();

    // First enable it
    emu.enable_console();
    assert!(emu.cfg.console_enabled, "Console should be enabled");

    // Then disable it
    emu.disable_console();

    assert!(
        !emu.cfg.console_enabled,
        "Console should be disabled after disable_console()"
    );
}

#[test]
fn test_console_toggle() {
    let mut emu = crate::emu64();

    emu.enable_console();
    assert!(emu.cfg.console_enabled);

    emu.disable_console();
    assert!(!emu.cfg.console_enabled);

    emu.enable_console();
    assert!(emu.cfg.console_enabled);
}

#[test]
fn test_console_ctrlc_independent() {
    let mut emu = crate::emu64();

    // Enable console but disable Ctrl+C
    emu.enable_console();
    emu.disable_ctrlc();

    assert!(emu.cfg.console_enabled, "Console should be enabled");
    assert!(!emu.enabled_ctrlc, "Ctrl+C should be disabled");

    // Disable console but enable Ctrl+C
    emu.disable_console();
    emu.enable_ctrlc();

    assert!(!emu.cfg.console_enabled, "Console should be disabled");
    assert!(emu.enabled_ctrlc, "Ctrl+C should be enabled");
}

#[test]
fn test_ctrlc_32bit_mode() {
    let mut emu = crate::emu32();

    emu.enable_ctrlc();
    assert!(emu.enabled_ctrlc, "Ctrl+C should work in 32-bit mode");

    emu.disable_ctrlc();
    assert!(
        !emu.enabled_ctrlc,
        "Ctrl+C disable should work in 32-bit mode"
    );
}

#[test]
fn test_console_32bit_mode() {
    let mut emu = crate::emu32();

    emu.enable_console();
    assert!(
        emu.cfg.console_enabled,
        "Console should work in 32-bit mode"
    );

    emu.disable_console();
    assert!(
        !emu.cfg.console_enabled,
        "Console disable should work in 32-bit mode"
    );
}

#[test]
fn test_ctrlc_idempotent_enable() {
    let mut emu = crate::emu64();

    emu.enable_ctrlc();
    emu.enable_ctrlc();
    emu.enable_ctrlc();

    assert!(emu.enabled_ctrlc, "Multiple enable calls should work");
}

#[test]
fn test_ctrlc_idempotent_disable() {
    let mut emu = crate::emu64();

    emu.disable_ctrlc();
    emu.disable_ctrlc();
    emu.disable_ctrlc();

    assert!(!emu.enabled_ctrlc, "Multiple disable calls should work");
}

#[test]
fn test_console_idempotent_enable() {
    let mut emu = crate::emu64();

    emu.enable_console();
    emu.enable_console();
    emu.enable_console();

    assert!(emu.cfg.console_enabled, "Multiple enable calls should work");
}

#[test]
fn test_console_idempotent_disable() {
    let mut emu = crate::emu64();

    emu.disable_console();
    emu.disable_console();
    emu.disable_console();

    assert!(
        !emu.cfg.console_enabled,
        "Multiple disable calls should work"
    );
}

#[test]
fn console_reads_only_strip_the_line_ending() {
    use crate::debug::console::Console;

    for (input, expected) in [
        ("", None),
        ("\n", Some("")),
        ("Ab\r\n", Some("Ab")),
        ("A\rB\n", Some("A\rB")),
        ("A\r", Some("A\r")),
        ("ABC", Some("ABC")),
        ("ABCЖ", Some("ABCЖ")),
    ] {
        assert_eq!(
            Console::read_line(input.as_bytes()).unwrap().as_deref(),
            expected
        );
    }
    assert!(Console::read_line([0xff].as_slice()).is_err());
}

#[test]
fn loop_limit_counts_each_instruction_address_independently() {
    let mut emu = crate::emu64();
    emu.cfg.set_loop_limit(2);
    // Three NOPs, increment EAX, jump back to the increment.
    emu.load_code_bytes(&[0x90, 0x90, 0x90, 0xff, 0xc0, 0xeb, 0xfc]);
    let increment = emu.pc() + 3;
    emu.regs_mut().set_eax(0);
    assert_eq!(emu.run(None).unwrap(), increment);
    assert_eq!(emu.regs().get_eax(), 2);
}

#[test]
fn loop_limit_counts_a_bulk_rep_once_per_entry() {
    let mut emu = crate::emu64();
    emu.cfg.set_loop_limit(1);
    // REP STOSB, reload ECX with 16, jump back to REP STOSB.
    emu.load_code_bytes(&[0xf3, 0xaa, 0xb9, 16, 0, 0, 0, 0xeb, 0xf7])
        .expect("load REP loop");
    let rep = emu.pc();
    let output = emu.regs().rsp - 32;
    emu.regs_mut().rdi = output;
    emu.regs_mut().rcx = 16;

    assert_eq!(emu.run(None).expect("stop at second REP"), rep);
    assert_eq!(emu.regs().rdi, output + 16);
    assert_eq!(emu.regs().rcx, 16);
}

#[test]
fn threaded_loop_limit_counts_only_the_scheduled_instruction() {
    // Each program executes a NOP and jumps back to it.
    for (mut emu, code, jump_offset) in [
        (crate::emu64(), &[0x90, 0xeb, 0xfd][..], 1),
        (
            crate::emu_aarch64(),
            &[0x1f, 0x20, 0x03, 0xd5, 0xff, 0xff, 0xff, 0x17],
            4,
        ),
    ] {
        emu.load_code_bytes(code).expect("load threaded loop");
        let start = emu.pc();
        let mut worker = emu.threads[0].clone();
        worker.id += 1;
        emu.threads.push(worker);
        emu.threads[0].suspended = true;
        emu.set_pc(start + jump_offset);
        emu.enable_threading(true);
        emu.cfg.set_loop_limit(1);

        assert_eq!(emu.run(None).expect("stop before the second NOP"), start);
        assert_eq!(emu.current_thread_id, 1);
        assert_eq!(emu.instruction_count, 2);
    }
}

#[test]
fn loop_limit_survives_the_switch_to_thread_scheduling() {
    let mut emu = crate::emu64();
    // INC EAX, NOP, jump back to INC EAX.
    emu.load_code_bytes(&[0xff, 0xc0, 0x90, 0xeb, 0xfb])
        .expect("load increment loop");
    let start = emu.pc();
    emu.regs_mut().set_eax(0);
    emu.cfg.set_loop_limit(1);
    emu.set_hook_on_pre_instruction(move |emu, pc, _, _| {
        if pc == start + 2 {
            let mut worker = emu.current_thread().clone();
            worker.id += 1;
            worker.suspended = true;
            emu.threads.push(worker);
            emu.enable_threading(true);
        }
        true
    });

    assert_eq!(emu.run(None).expect("stop on the second increment"), start);
    assert_eq!(emu.regs().get_eax(), 1);
}

#[test]
fn emulator_logger_preserves_the_host_logger() {
    crate::tests::helpers::setup();
    let before = log::logger();
    let level = log::max_level();
    let mut emu = crate::emu64();
    emu.init_logger();
    emu.init_logger();
    assert!(std::ptr::eq(before, log::logger()));
    assert_eq!(log::max_level(), level);
}
