use crate::context::{context32::Context32, context64::Context64};
use crate::maps::mem64::Permission;
use crate::tests::helpers;
use crate::threading::scheduler::ThreadScheduler;
use crate::winapi::winapi64;
use crate::windows::constants::errors::*;
use crate::windows::constants::exceptions::*;
use crate::windows::constants::process::*;
use crate::{emu32, emu64};
use itertools::iproduct;

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

#[test]
fn windows_thread_creation_prepares_stack_and_preserves_parent() {
    for (is_64, library, size) in iproduct!(
        [false, true],
        ["kernel32.dll", "kernelbase.dll"],
        [0, 0x2345]
    ) {
        let mut emu = if is_64 { emu64() } else { emu32() };
        emu.cfg.console_enabled = false;
        let output = 0x500000;
        emu.maps
            .create_map("thread_output", output, 0x1000, Permission::READ_WRITE)
            .unwrap();
        emu.regs_mut().rbx = 0x1234;
        let handle = helpers::call_winapi_export(
            &mut emu,
            library,
            "CreateThread",
            &[0, size, 0x600000, 0x9876, CREATE_SUSPENDED, output],
        );
        assert_ne!(handle, 0, "{library}, x64={is_64}");
        assert_eq!(emu.current_thread_id, 0);
        assert_eq!(emu.regs().rbx, 0x1234);
        assert_eq!(emu.threads.len(), 2);
        let child = &emu.threads[1];
        let regs = child.regs_x86();
        assert!(child.suspended);
        assert_eq!(child.handle, handle);
        assert_eq!(emu.maps.read_dword(output), Some(child.id as u32));
        assert_eq!(regs.rip, 0x600000);
        assert!(emu.maps.validate_write_range(regs.rsp - 16, 16));
        if is_64 {
            assert_eq!(regs.rsp % 16, 8);
            assert_eq!(emu.maps.read_qword(regs.rsp), Some(RETURN_THREAD as u64));
            assert_eq!(regs.rcx, 0x9876);
            assert!(emu.maps.validate_write_range(regs.rsp + 8, 32));
        } else {
            assert_eq!(emu.maps.read_dword(regs.rsp), Some(RETURN_THREAD));
            assert_eq!(emu.maps.read_dword(regs.rsp + 4), Some(0x9876));
        }
        assert_eq!(
            helpers::call_winapi_export(&mut emu, library, "ResumeThread", &[handle]),
            1
        );
        assert!(!emu.threads[1].suspended);
        assert_eq!(
            helpers::call_winapi_export(&mut emu, library, "ResumeThread", &[handle]),
            0
        );
    }
}

#[test]
fn windows_thread_creation_failure_does_not_publish_a_thread() {
    for is_64 in [false, true] {
        let mut emu = if is_64 { emu64() } else { emu32() };
        emu.cfg.console_enabled = false;
        for (size, flags, output, error) in [
            (0, CREATE_SUSPENDED, 1, ERROR_NOACCESS),
            (0, 2, 0, ERROR_INVALID_PARAMETER),
            (u32::MAX as u64, 0, 0, ERROR_NOT_ENOUGH_MEMORY),
        ] {
            let result = helpers::call_winapi_export(
                &mut emu,
                "kernel32.dll",
                "CreateThread",
                &[0, size, 0x600000, 0, flags, output],
            );
            assert_eq!(result, 0);
            assert_eq!(emu.get_last_error(), error);
            assert_eq!(emu.threads.len(), 1);
        }
    }
}

#[test]
fn windows_thread_context_targets_the_handle_and_selected_registers() {
    for (is_64, library) in iproduct!([false, true], ["kernel32.dll", "kernelbase.dll"]) {
        let mut emu = if is_64 { emu64() } else { emu32() };
        emu.cfg.console_enabled = false;
        let address = 0x500000;
        emu.maps
            .create_map("context", address, 0x1000, Permission::READ_WRITE)
            .unwrap();
        let handle = helpers::call_winapi_export(
            &mut emu,
            library,
            "CreateThread",
            &[0, 0, 0x600000, 0, CREATE_SUSPENDED, 0],
        );
        let mut regs = *emu.threads[1].regs_x86();
        let original_sp = regs.rsp;
        regs.rbx = 0x123456;
        regs.rsp = 0;
        regs.rip = 0;
        let (flags_offset, integer, control) = if is_64 {
            Context64::new(&regs).save(address, &mut emu.maps);
            (Context64::FLAGS_OFFSET, CONTEXT_INTEGER, CONTEXT_CONTROL)
        } else {
            Context32::new(&regs).save(address as u32, &mut emu.maps);
            (
                Context32::FLAGS_OFFSET,
                WOW64_CONTEXT_INTEGER,
                WOW64_CONTEXT_CONTROL,
            )
        };
        emu.maps.write_dword(address + flags_offset, integer);
        emu.regs_mut().rbx = 0x777;
        assert_eq!(
            helpers::call_winapi_export(&mut emu, library, "SetThreadContext", &[handle, address]),
            1
        );
        assert_eq!(emu.regs().rbx, 0x777);
        assert_eq!(emu.threads[1].regs_x86().rbx, 0x123456);
        assert_eq!(emu.threads[1].regs_x86().rsp, original_sp);
        assert_eq!(emu.threads[1].regs_x86().rip, 0x600000);
        emu.maps
            .write_dword(address + flags_offset, integer | control);
        assert_eq!(
            helpers::call_winapi_export(&mut emu, library, "GetThreadContext", &[handle, address]),
            1
        );
        if is_64 {
            let mut saved = Context64::new(&regs);
            saved.load(address, &mut emu.maps);
            saved.sync(&mut regs);
        } else {
            let mut saved = Context32::new(&regs);
            saved.load(address as u32, &mut emu.maps);
            saved.sync(&mut regs);
        }
        assert_eq!(regs.rsp, original_sp);
        assert_eq!(regs.rbx, 0x123456);
        assert_eq!(
            helpers::call_winapi_export(&mut emu, library, "SetThreadContext", &[0, address]),
            0
        );
        assert_eq!(emu.get_last_error(), ERROR_INVALID_HANDLE);
    }
}

#[test]
fn windows_thread_return_finishes_only_the_child() {
    for is_64 in [false, true] {
        let mut emu = if is_64 { emu64() } else { emu32() };
        emu.enable_threading(true);
        emu.cfg.console_enabled = false;
        let code = 0x600000;
        emu.maps
            .create_map("thread_code", code, 0x1000, Permission::READ_WRITE_EXECUTE)
            .unwrap();
        // mov eax, 42; ret (the x86 callback also removes its argument).
        let bytes: &[u8] = if is_64 {
            &[0xb8, 42, 0, 0, 0, 0xc3]
        } else {
            &[0xb8, 42, 0, 0, 0, 0xc2, 4, 0]
        };
        emu.maps.write_bytes(code, bytes);
        helpers::call_winapi_export(
            &mut emu,
            "kernel32.dll",
            "CreateThread",
            &[0, 0, code, 0, 0, 0],
        );
        assert!(ThreadScheduler::execute_thread_instruction(&mut emu, 1));
        assert!(ThreadScheduler::execute_thread_instruction(&mut emu, 1));
        assert_eq!(emu.threads[1].exit_value, Some(42));
        assert_eq!(emu.threads[0].exit_value, None);
    }
}
