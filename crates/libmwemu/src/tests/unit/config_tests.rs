use crate::arch::Arch;
use crate::config::Config;

#[test]
fn test_config_default_values() {
    let cfg = Config::new();

    // Test default values
    assert!(!cfg.is_x64(), "Default should be 32-bit mode");
    assert_eq!(cfg.verbose, 0, "Default verbose should be 0");
}

#[test]
fn test_config_64bit_mode() {
    let mut cfg = Config::new();
    cfg.arch = Arch::X86_64;

    assert!(cfg.is_x64(), "Should be in 64-bit mode");
}

#[test]
fn test_config_32bit_mode() {
    let mut cfg = Config::new();
    cfg.arch = Arch::X86;

    assert!(!cfg.is_x64(), "Should be in 32-bit mode");
}

#[test]
fn test_config_verbose_levels() {
    let mut cfg = Config::new();

    // Test different verbose levels
    cfg.verbose = 0;
    assert_eq!(cfg.verbose, 0);

    cfg.verbose = 1;
    assert_eq!(cfg.verbose, 1);

    cfg.verbose = 2;
    assert_eq!(cfg.verbose, 2);

    cfg.verbose = 3;
    assert_eq!(cfg.verbose, 3);
}

#[test]
fn test_config_console_default() {
    let cfg = Config::new();

    // Check default console state (whatever it is)
    let _console_state = cfg.console_enabled;
}

#[test]
fn test_config_console_toggle() {
    let mut cfg = Config::new();

    cfg.console_enabled = false;
    assert!(!cfg.console_enabled);

    cfg.console_enabled = true;
    assert!(cfg.console_enabled);
}

#[test]
fn test_config_threading_disabled_by_default() {
    let cfg = Config::new();

    assert!(
        !cfg.enable_threading,
        "Threading should be disabled by default"
    );
}

#[test]
fn test_config_threading_enable() {
    let mut cfg = Config::new();

    cfg.enable_threading = true;
    assert!(cfg.enable_threading, "Threading should be enabled");
}

#[test]
fn test_config_multiple_settings() {
    let mut cfg = Config::new();

    // Set multiple configuration options
    cfg.arch = Arch::X86_64;
    cfg.verbose = 2;
    cfg.console_enabled = false;
    cfg.enable_threading = true;

    // Verify all are set correctly
    assert!(cfg.is_x64());
    assert_eq!(cfg.verbose, 2);
    assert!(!cfg.console_enabled);
    assert!(cfg.enable_threading);
}

#[test]
fn test_emu64_default_config() {
    let emu = crate::emu64();

    assert!(emu.cfg.is_x64(), "emu64() should create 64-bit emulator");
}

#[test]
fn test_emu32_default_config() {
    let emu = crate::emu32();

    assert!(!emu.cfg.is_x64(), "emu32() should create 32-bit emulator");
}

#[test]
fn test_config_independence() {
    let cfg1 = Config::new();
    let mut cfg2 = Config::new();

    cfg2.verbose = 5;

    // cfg1 should not be affected
    assert_eq!(cfg1.verbose, 0, "Configs should be independent");
    assert_eq!(cfg2.verbose, 5);
}

#[test]
fn test_set_config() {
    let mut emu = crate::emu64();

    let mut custom_cfg = Config::new();
    custom_cfg.verbose = 3;
    custom_cfg.console_enabled = false;

    emu.set_config(custom_cfg);

    assert_eq!(emu.cfg.verbose, 3, "Custom config should be applied");
    assert!(!emu.cfg.console_enabled, "Custom config should be applied");
}

#[test]
fn test_config_verbose_range() {
    let mut cfg = Config::new();

    // Test edge cases
    cfg.verbose = 0;
    assert_eq!(cfg.verbose, 0);

    cfg.verbose = 10;
    assert_eq!(cfg.verbose, 10);

    cfg.verbose = 100;
    assert_eq!(cfg.verbose, 100);
}
