// Tests for instruction pointer operations
// Note: Most set_rip/set_eip functionality requires complex setup
// These tests verify basic behavior

#[test]
fn test_set_rip_nonmapped_linux() {
    let mut emu = crate::emu64();
    emu.os = crate::arch::OperatingSystem::Linux;

    // In Linux mode, unmapped addresses return false
    let result = emu.set_rip(0xdeadbeef, false);
    assert!(!result);
}

#[test]
fn test_set_eip_nonmapped_linux() {
    let mut emu = crate::emu32();
    emu.os = crate::arch::OperatingSystem::Linux;

    // In Linux mode, unmapped addresses return false
    let result = emu.set_eip(0xdeadbeef, false);
    assert!(!result);
}

#[test]
fn test_force_reload_flag_exists() {
    let mut emu = crate::emu64();

    emu.force_reload = false;
    assert!(!emu.force_reload);

    emu.force_reload = true;
    assert!(emu.force_reload);
}

#[test]
fn test_os_default_is_windows() {
    let emu = crate::emu64();
    assert!(emu.os.is_windows());
}

#[test]
fn test_skip_apicall_flag() {
    let mut emu = crate::emu64();

    emu.skip_apicall = false;
    assert!(!emu.skip_apicall);

    emu.skip_apicall = true;
    assert!(emu.skip_apicall);
}
