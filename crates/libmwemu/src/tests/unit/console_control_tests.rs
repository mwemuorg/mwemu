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
