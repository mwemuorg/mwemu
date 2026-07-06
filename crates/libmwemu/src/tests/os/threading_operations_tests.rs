#[test]
fn test_threading_disabled_by_default() {
    let emu = crate::emu64();

    assert!(
        !emu.is_threading_enabled(),
        "Threading should be disabled by default"
    );
}

#[test]
fn test_enable_threading() {
    let mut emu = crate::emu64();

    emu.enable_threading(true);

    assert!(
        emu.is_threading_enabled(),
        "Threading should be enabled after enable_threading(true)"
    );
    assert!(
        emu.cfg.enable_threading,
        "Config should reflect threading enabled"
    );
}

#[test]
fn test_disable_threading() {
    let mut emu = crate::emu64();

    // First enable it
    emu.enable_threading(true);
    assert!(emu.is_threading_enabled(), "Threading should be enabled");

    // Then disable it
    emu.enable_threading(false);

    assert!(
        !emu.is_threading_enabled(),
        "Threading should be disabled after enable_threading(false)"
    );
    assert!(
        !emu.cfg.enable_threading,
        "Config should reflect threading disabled"
    );
}

#[test]
fn test_threading_toggle() {
    let mut emu = crate::emu64();

    // Test multiple toggles
    emu.enable_threading(true);
    assert!(emu.is_threading_enabled());

    emu.enable_threading(false);
    assert!(!emu.is_threading_enabled());

    emu.enable_threading(true);
    assert!(emu.is_threading_enabled());

    emu.enable_threading(false);
    assert!(!emu.is_threading_enabled());
}

#[test]
fn test_threading_state_32bit() {
    let mut emu = crate::emu32();

    assert!(
        !emu.is_threading_enabled(),
        "32-bit emulator should have threading disabled by default"
    );

    emu.enable_threading(true);
    assert!(
        emu.is_threading_enabled(),
        "32-bit emulator should support threading"
    );
}

#[test]
fn test_threading_state_64bit() {
    let mut emu = crate::emu64();

    assert!(
        !emu.is_threading_enabled(),
        "64-bit emulator should have threading disabled by default"
    );

    emu.enable_threading(true);
    assert!(
        emu.is_threading_enabled(),
        "64-bit emulator should support threading"
    );
}

#[test]
fn test_threading_idempotent_enable() {
    let mut emu = crate::emu64();

    // Enable multiple times
    emu.enable_threading(true);
    emu.enable_threading(true);
    emu.enable_threading(true);

    assert!(
        emu.is_threading_enabled(),
        "Threading should remain enabled"
    );
}

#[test]
fn test_threading_idempotent_disable() {
    let mut emu = crate::emu64();

    // Disable multiple times (already disabled by default)
    emu.enable_threading(false);
    emu.enable_threading(false);
    emu.enable_threading(false);

    assert!(
        !emu.is_threading_enabled(),
        "Threading should remain disabled"
    );
}
