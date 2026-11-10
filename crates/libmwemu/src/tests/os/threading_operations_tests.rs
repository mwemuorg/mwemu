use crate::emu64;
use crate::maps::mem64::Permission;
use crate::tests::helpers;
use crate::winapi::winapi64;

#[test]
fn threading_disabled_by_default() {
    let emu = crate::emu64();

    assert!(
        !emu.is_threading_enabled(),
        "Threading should be disabled by default"
    );
}

#[test]
fn enable_threading() {
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
fn disable_threading() {
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
fn threading_toggle() {
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
fn threading_state_32bit() {
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
fn threading_state_64bit() {
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
fn threading_idempotent_enable() {
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
fn threading_idempotent_disable() {
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

#[test]
fn srw_lock_roundtrip_64() {
    helpers::setup();
    let mut emu = emu64();

    let lock_addr = 0x100000u64;
    emu.maps
        .create_map("srw_lock", lock_addr, 0x1000, Permission::READ_WRITE);

    helpers::call_winapi64(
        &mut emu,
        winapi64::kernel32::InitializeSRWLock,
        &[lock_addr],
    );
    assert_eq!(
        emu.maps.read_qword(lock_addr).unwrap(),
        0,
        "InitializeSRWLock must zero the lock"
    );

    helpers::call_winapi64(
        &mut emu,
        winapi64::kernel32::AcquireSRWLockExclusive,
        &[lock_addr],
    );
    helpers::call_winapi64(
        &mut emu,
        winapi64::kernel32::ReleaseSRWLockExclusive,
        &[lock_addr],
    );

    let acquired = helpers::call_winapi64(
        &mut emu,
        winapi64::kernel32::TryAcquireSRWLockExclusive,
        &[lock_addr],
    );
    assert_eq!(acquired, 1, "TryAcquireSRWLockExclusive must succeed");

    let shared = helpers::call_winapi64(
        &mut emu,
        winapi64::kernel32::TryAcquireSRWLockShared,
        &[lock_addr],
    );
    assert_eq!(shared, 1, "TryAcquireSRWLockShared must succeed");
}

#[test]
fn wait_for_multiple_objects_64() {
    helpers::setup();
    let mut emu = emu64();

    let handles_addr = 0x100000u64;
    emu.maps
        .create_map("handles", handles_addr, 0x1000, Permission::READ_WRITE);
    emu.maps.write_qword(handles_addr, 0x100);
    emu.maps.write_qword(handles_addr + 8, 0x200);

    let ret = helpers::call_winapi64(
        &mut emu,
        winapi64::kernel32::WaitForMultipleObjects,
        &[2, handles_addr, 0, 0],
    );
    assert_eq!(ret, 0, "WaitForMultipleObjects must return WAIT_OBJECT_0");
}

#[test]
fn srw_lock_lifecycle() {
    helpers::setup();
    let mut emu = emu64();

    let lock_addr = 0x100000u64;
    emu.maps
        .create_map("srwlock", lock_addr, 0x1000, Permission::READ_WRITE);

    helpers::call_winapi64(
        &mut emu,
        winapi64::kernel32::InitializeSRWLock,
        &[lock_addr],
    );
    assert_eq!(
        emu.maps.read_qword(lock_addr).unwrap(),
        0,
        "SRWLock should be zeroed after init"
    );

    helpers::call_winapi64(
        &mut emu,
        winapi64::kernel32::AcquireSRWLockExclusive,
        &[lock_addr],
    );

    helpers::call_winapi64(
        &mut emu,
        winapi64::kernel32::ReleaseSRWLockExclusive,
        &[lock_addr],
    );

    helpers::call_winapi64(
        &mut emu,
        winapi64::kernel32::AcquireSRWLockShared,
        &[lock_addr],
    );

    helpers::call_winapi64(
        &mut emu,
        winapi64::kernel32::ReleaseSRWLockShared,
        &[lock_addr],
    );
}

#[test]
fn try_acquire_srw_lock() {
    helpers::setup();
    let mut emu = emu64();

    let lock_addr = 0x100000u64;
    emu.maps
        .create_map("srwlock", lock_addr, 0x1000, Permission::READ_WRITE);

    helpers::call_winapi64(
        &mut emu,
        winapi64::kernel32::InitializeSRWLock,
        &[lock_addr],
    );

    let ret = helpers::call_winapi64(
        &mut emu,
        winapi64::kernel32::TryAcquireSRWLockExclusive,
        &[lock_addr],
    );
    assert_eq!(
        ret, 1,
        "TryAcquireSRWLockExclusive should succeed (return 1)"
    );

    let ret = helpers::call_winapi64(
        &mut emu,
        winapi64::kernel32::TryAcquireSRWLockShared,
        &[lock_addr],
    );
    assert_eq!(ret, 1, "TryAcquireSRWLockShared should succeed (return 1)");
}

#[test]
fn wait_for_multiple_objects() {
    helpers::setup();
    let mut emu = emu64();

    let handles_addr = 0x100000u64;
    emu.maps
        .create_map("handles", handles_addr, 0x1000, Permission::READ_WRITE);
    emu.maps.write_qword(handles_addr, 0x10);
    emu.maps.write_qword(handles_addr + 8, 0x20);

    let ret = helpers::call_winapi64(
        &mut emu,
        winapi64::kernel32::WaitForMultipleObjects,
        &[2, handles_addr, 0, 100],
    );
    assert_eq!(
        ret, 0,
        "WaitForMultipleObjects should return WAIT_OBJECT_0 (0)"
    );
}
