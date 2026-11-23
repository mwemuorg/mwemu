use itertools::iproduct;

use crate::api::windows::common::memory::{read_pointer, write_pointer};
use crate::context::{context32::Context32, context64::Context64};
use crate::emu::Emu;
use crate::maps::mem64::Permission;
use crate::serialization::Serialization;
use crate::tests::helpers;
use crate::threading::scheduler::ThreadScheduler;
use crate::windows::constants::errors::*;
use crate::windows::constants::exceptions::*;
use crate::windows::constants::process::*;
use crate::windows::structures::{TEB, TEB64};
use crate::{emu32, emu64};

#[test]
fn threading_disabled_by_default() {
    let emu = emu64();

    assert!(
        !emu.is_threading_enabled(),
        "Threading should be disabled by default"
    );
}

#[test]
fn enable_threading() {
    let mut emu = emu64();

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
    let mut emu = emu64();

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
    let mut emu = emu64();

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
    let mut emu = emu32();

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
    let mut emu = emu64();

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
    let mut emu = emu64();

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
    let mut emu = emu64();

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
        if !is_64 {
            const DR0_OFFSET: u64 = 4;
            const DR7_OFFSET: u64 = 24;
            emu.maps
                .write_dword(address + flags_offset, WOW64_CONTEXT_DEBUG_REGISTERS);
            emu.maps.write_dword(address + DR0_OFFSET, 0x600100);
            emu.maps.write_dword(address + DR7_OFFSET, 1);
            assert_eq!(
                helpers::call_winapi_export(
                    &mut emu,
                    library,
                    "SetThreadContext",
                    &[handle, address]
                ),
                1
            );
            assert_eq!(emu.threads[1].regs_x86().dr0, 0x600100);
            assert_eq!(emu.threads[1].regs_x86().dr7, 1);
            assert_eq!(emu.regs().dr7, 0);
            emu.maps.write_dword(address + DR0_OFFSET, 0);
            assert_eq!(
                helpers::call_winapi_export(
                    &mut emu,
                    library,
                    "GetThreadContext",
                    &[handle, address]
                ),
                1
            );
            assert_eq!(emu.maps.read_dword(address + DR0_OFFSET), Some(0x600100));
        }
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

#[test]
fn ntdll32_tls_gateway_keeps_slot_storage() {
    const TLS_VALUE: u64 = 0x1234_5678;
    let (mut emu, _) = helpers::prepare_winapi(false);
    let first = helpers::call_winapi_export(&mut emu, "ntdll.dll", "RtlTlsAlloc", &[]);
    let second = helpers::call_winapi_export(&mut emu, "ntdll.dll", "RtlTlsAlloc", &[]);
    assert_ne!(first, second);
    assert_ne!(
        helpers::call_winapi_export(
            &mut emu,
            "ntdll.dll",
            "RtlTlsSetValue",
            &[second, TLS_VALUE]
        ),
        0
    );
    assert_eq!(
        helpers::call_winapi_export(&mut emu, "ntdll.dll", "RtlTlsGetValue", &[second]),
        TLS_VALUE
    );
    assert_eq!(
        helpers::call_winapi_export(&mut emu, "ntdll.dll", "RtlTlsGetValue", &[first]),
        0
    );
    assert_ne!(
        helpers::call_winapi_export(&mut emu, "ntdll.dll", "RtlTlsFree", &[second]),
        0
    );
    assert_eq!(
        helpers::call_winapi_export(&mut emu, "ntdll.dll", "RtlTlsGetValue", &[second]),
        0
    );
    assert!(emu.api_policy.last_outcome.is_none());
}

#[test]
fn ntdll_slist_gateway_clears_only_the_guest_header() {
    const HEADER32_SIZE: u64 = 8;
    const HEADER64_SIZE: u64 = 16;
    const CANARY: u8 = 0xa5;
    for (is64, header_size) in [(false, HEADER32_SIZE), (true, HEADER64_SIZE)] {
        let (mut emu, data) = helpers::prepare_winapi(is64);
        let header = data + HEADER64_SIZE;
        assert!(
            emu.maps
                .fill_range(header - 1, CANARY, (header_size + 2) as usize)
        );
        helpers::call_winapi_export(&mut emu, "ntdll.dll", "RtlInitializeSListHead", &[header]);
        assert_eq!(emu.maps.read_byte(header - 1), Some(CANARY));
        assert_eq!(emu.maps.read_byte(header + header_size), Some(CANARY));
        for offset in 0..header_size {
            assert_eq!(emu.maps.read_byte(header + offset), Some(0));
        }
        assert!(emu.api_policy.last_outcome.is_none());
    }
}

#[test]
fn windows_thread_identity_and_suspend_counts_share_handles_and_survive_snapshots() {
    use crate::serialization::Serialization;
    for (is64, library) in iproduct!([false, true], ["kernel32.dll", "kernelbase.dll"]) {
        let (mut emu, _) = helpers::prepare_winapi(is64);
        let parent = emu.current_thread().id;
        assert_eq!(
            helpers::call_winapi_export(&mut emu, library, "GetCurrentThreadId", &[]),
            parent
        );
        assert_eq!(
            helpers::call_winapi_export(&mut emu, library, "GetThreadId", &[NT_CURRENT_THREAD]),
            parent
        );
        let thread = helpers::call_winapi_export(
            &mut emu,
            library,
            "CreateThread",
            &[0, 0, 0x600000, 0, CREATE_SUSPENDED, 0],
        );
        let child = emu.threads[1].id;
        assert_eq!(
            helpers::call_winapi_export(&mut emu, library, "GetThreadId", &[thread]),
            child
        );
        let opened = helpers::call_winapi_export(&mut emu, library, "OpenThread", &[0, 0, child]);
        assert_ne!(opened, 0);
        assert_ne!(opened, thread);
        emu.set_last_error(ERROR_ACCESS_DENIED);
        for (api, expected, suspended) in [
            ("SuspendThread", 1, true),
            ("SuspendThread", 2, true),
            ("ResumeThread", 3, true),
            ("ResumeThread", 2, true),
        ] {
            assert_eq!(
                helpers::call_winapi_export(&mut emu, library, api, &[opened]),
                expected
            );
            assert_eq!(emu.threads[1].suspended, suspended);
            assert_eq!(emu.get_last_error(), ERROR_ACCESS_DENIED);
        }
        emu = Serialization::deserialize(&Serialization::serialize(&emu));
        assert_eq!(
            helpers::call_winapi_export(&mut emu, library, "GetThreadId", &[opened]),
            child
        );
        assert_eq!(
            helpers::call_winapi_export(&mut emu, library, "ResumeThread", &[thread]),
            1
        );
        assert!(!emu.threads[1].suspended);
        assert_eq!(
            helpers::call_winapi_export(&mut emu, library, "ResumeThread", &[opened]),
            0
        );
        emu.threads[1].windows.suspend_count = MAXIMUM_SUSPEND_COUNT;
        emu.threads[1].suspended = true;
        assert_eq!(
            helpers::call_winapi_export(&mut emu, library, "SuspendThread", &[thread]),
            u32::MAX as u64
        );
        assert_eq!(emu.get_last_error(), ERROR_SIGNAL_REFUSED);
        assert_eq!(emu.threads[1].windows.suspend_count, MAXIMUM_SUSPEND_COUNT);
        assert_eq!(
            helpers::call_winapi_export(&mut emu, library, "CloseHandle", &[opened]),
            1
        );
        assert_eq!(
            helpers::call_winapi_export(&mut emu, library, "GetThreadId", &[opened]),
            0
        );
        assert_eq!(emu.get_last_error(), ERROR_INVALID_HANDLE);
        assert_eq!(
            helpers::call_winapi_export(&mut emu, library, "GetThreadId", &[thread]),
            child
        );
        assert_eq!(
            helpers::call_winapi_export(&mut emu, library, "OpenThread", &[0, 0, 0]),
            0
        );
        assert_eq!(emu.get_last_error(), ERROR_INVALID_PARAMETER);
    }
}

#[test]
fn windows_thread_error_state_follows_teb_memory_and_survives_snapshots() {
    use crate::serialization::{Serialization, native::NativeSnapshotV5};
    use iced_x86::{Code, Encoder, Instruction, MemoryOperand, Register};

    const CODE_BASE: u64 = 0x600000;
    const PRIMARY_TEB: u64 = 0x700000;
    const GUEST_ERROR: u32 = 0x1234;
    for is64 in [false, true] {
        let (mut emu, _) = helpers::prepare_winapi(is64);
        emu.maps
            .create_map("teb", PRIMARY_TEB, 0x2000, Permission::READ_WRITE)
            .unwrap();
        emu.maps
            .create_map(
                "teb_access_code",
                CODE_BASE,
                0x1000,
                Permission::READ_WRITE_EXECUTE,
            )
            .unwrap();
        let (last_error, tid, segment) = if is64 {
            (
                TEB64::LAST_ERROR_VALUE_OFFSET,
                TEB64::CLIENT_ID_THREAD_ID_OFFSET,
                Register::GS,
            )
        } else {
            (
                TEB::LAST_ERROR_VALUE_OFFSET,
                TEB::CLIENT_ID_THREAD_ID_OFFSET,
                Register::FS,
            )
        };
        let memory = |offset| {
            MemoryOperand::new(
                Register::None,
                Register::None,
                1,
                offset as i64,
                4,
                false,
                segment,
            )
        };
        let instructions = [
            Instruction::with2(Code::Mov_rm32_r32, memory(last_error), Register::EAX).unwrap(),
            Instruction::with2(Code::Mov_r32_rm32, Register::EAX, memory(last_error)).unwrap(),
            Instruction::with2(Code::Mov_r32_rm32, Register::ECX, memory(tid)).unwrap(),
        ];
        let mut encoder = Encoder::new(if is64 { 64 } else { 32 });
        let mut pc = CODE_BASE;
        for instruction in instructions {
            pc += encoder.encode(&instruction, pc).unwrap() as u64;
        }
        assert!(emu.maps.write_bytes(CODE_BASE, &encoder.take_buffer()));
        helpers::call_winapi_export(
            &mut emu,
            "kernel32.dll",
            "SetLastError",
            &[ERROR_ACCESS_DENIED],
        );
        assert_eq!(
            emu.maps.read_dword(PRIMARY_TEB + last_error),
            Some(ERROR_ACCESS_DENIED as u32)
        );
        helpers::call_winapi_export(
            &mut emu,
            "kernel32.dll",
            "CreateThread",
            &[0, 0, CODE_BASE, 0, 0, 0],
        );
        let child_teb = emu.threads[1].windows.teb;
        assert_ne!(child_teb, PRIMARY_TEB);
        assert_ne!(child_teb, 0);
        assert_eq!(emu.maps.read_dword(child_teb + last_error), Some(0));
        assert!(ThreadScheduler::switch_to_thread(&mut emu, 1));
        assert_eq!(
            helpers::call_winapi_export(&mut emu, "kernelbase.dll", "GetLastError", &[]),
            0
        );
        emu.regs_mut().rip = CODE_BASE;
        emu.regs_mut().rax = GUEST_ERROR as u64;
        let previous_seh = emu.seh();
        assert!(emu.step());
        assert_eq!(emu.get_last_error(), GUEST_ERROR as u64);
        assert_eq!(
            emu.seh(),
            previous_seh,
            "a TEB field store must not replace the SEH head"
        );
        emu.regs_mut().rax = 0;
        assert!(emu.step());
        assert_eq!(emu.regs().rax, GUEST_ERROR as u64);
        assert!(emu.step());
        assert_eq!(emu.regs().get_ecx(), emu.current_thread().id);
        assert!(
            emu.maps
                .write_dword(child_teb + last_error, ERROR_INVALID_HANDLE as u32)
        );
        assert_eq!(
            helpers::call_winapi_export(&mut emu, "kernel32.dll", "GetLastError", &[]),
            ERROR_INVALID_HANDLE
        );
        assert!(ThreadScheduler::switch_to_thread(&mut emu, 0));
        assert_eq!(emu.get_last_error(), ERROR_ACCESS_DENIED);
        let mut restored = Serialization::deserialize(&Serialization::serialize(&emu));
        assert_eq!(restored.get_last_error(), ERROR_ACCESS_DENIED);
        assert!(ThreadScheduler::switch_to_thread(&mut restored, 1));
        assert_eq!(restored.get_last_error(), ERROR_INVALID_HANDLE);
        restored.clear_last_error();
        assert_eq!(restored.maps.read_dword(child_teb + last_error), Some(0));
        // Legacy snapshots have only the active thread's error and a boolean suspension state.
        let mut legacy = b"MWEMU\0\x05\0".to_vec();
        legacy.extend(bitcode::serialize(&NativeSnapshotV5::from(&emu)).unwrap());
        let restored = Serialization::deserialize(&legacy);
        assert_eq!(restored.get_last_error(), ERROR_ACCESS_DENIED);
    }
}

#[test]
fn waits_observe_event_reset_mode_and_consume_semaphore_counts() {
    use crate::tests::helpers::call_winapi_export as call;
    use crate::windows::constants::process::{WAIT_FAILED, WAIT_OBJECT_0, WAIT_TIMEOUT};
    for is64 in [false, true] {
        let (mut emu, data) = helpers::prepare_winapi(is64);
        let width = if is64 { 8 } else { 4 };
        emu.maps.write_wide_string(data, "LocalEvent");
        let event = call(&mut emu, "kernelbase.dll", "CreateEventW", &[0, 0, 0, data]);
        let other = call(
            &mut emu,
            "kernel32.dll",
            "OpenEventW",
            &[0x0010_0002, 0, data],
        );
        assert_ne!(event, 0);
        assert_ne!(other, 0);
        assert_ne!(event, other);
        emu = Serialization::deserialize(&Serialization::serialize(&emu));
        let semaphore = call(&mut emu, "kernel32.dll", "CreateSemaphoreA", &[0, 1, 2, 0]);
        assert!(write_pointer(&mut emu, data, semaphore));
        assert!(write_pointer(&mut emu, data + width, event));
        assert_eq!(
            call(
                &mut emu,
                "kernelbase.dll",
                "WaitForMultipleObjects",
                &[2, data, 1, 0]
            ),
            WAIT_TIMEOUT
        );
        assert_eq!(
            call(&mut emu, "ntdll.dll", "ZwSetEvent", &[other, data + 64]),
            0
        );
        assert_eq!(emu.maps.read_dword(data + 64), Some(0));
        assert_eq!(
            call(
                &mut emu,
                "kernel32.dll",
                "WaitForMultipleObjects",
                &[2, data, 1, 0]
            ),
            WAIT_OBJECT_0
        );
        assert_eq!(
            call(
                &mut emu,
                "kernel32.dll",
                "WaitForMultipleObjects",
                &[2, data, 0, 0]
            ),
            WAIT_TIMEOUT
        );
        assert_eq!(
            call(
                &mut emu,
                "kernel32.dll",
                "ReleaseSemaphore",
                &[semaphore, 2, data + 64]
            ),
            1
        );
        assert_eq!(emu.maps.read_dword(data + 64), Some(0));
        assert_eq!(
            call(
                &mut emu,
                "kernel32.dll",
                "ReleaseSemaphore",
                &[semaphore, 1, data + 64]
            ),
            0
        );
        assert_eq!(emu.get_last_error(), ERROR_TOO_MANY_POSTS);
        assert_eq!(call(&mut emu, "ntdll.dll", "NtClose", &[other]), 0);
        assert_eq!(
            call(&mut emu, "kernel32.dll", "WaitForSingleObject", &[other, 0]),
            WAIT_FAILED
        );
        assert_eq!(call(&mut emu, "kernel32.dll", "SetEvent", &[event]), 1);
        emu.maps.write_qword(data + 128, 0);
        assert_eq!(
            call(
                &mut emu,
                "ntdll.dll",
                "NtWaitForSingleObject",
                &[event, 0, data + 128]
            ),
            WAIT_OBJECT_0
        );
        assert_eq!(
            call(
                &mut emu,
                "ntdll.dll",
                "NtWaitForSingleObject",
                &[event, 0, data + 128]
            ),
            WAIT_TIMEOUT
        );
    }
}

#[test]
fn mutex_recursion_and_relative_timer_waits_use_guest_state() {
    use crate::tests::helpers::call_winapi_export as call;
    use crate::windows::constants::process::{WAIT_OBJECT_0, WAIT_TIMEOUT};
    for is64 in [false, true] {
        let (mut emu, data) = helpers::prepare_winapi(is64);
        let mutex = call(&mut emu, "kernel32.dll", "CreateMutexW", &[0, 1, 0]);
        assert_eq!(
            call(
                &mut emu,
                "kernelbase.dll",
                "WaitForSingleObject",
                &[mutex, 0]
            ),
            WAIT_OBJECT_0
        );
        for _ in 0..2 {
            assert_eq!(call(&mut emu, "kernel32.dll", "ReleaseMutex", &[mutex]), 1);
        }
        assert_eq!(call(&mut emu, "kernel32.dll", "ReleaseMutex", &[mutex]), 0);
        let timer = call(&mut emu, "kernel32.dll", "CreateWaitableTimerW", &[0, 0, 0]);
        emu.maps.write_qword(data, (-50_000i64) as u64);
        assert_eq!(
            call(
                &mut emu,
                "kernelbase.dll",
                "SetWaitableTimer",
                &[timer, data, 0, 0, 0, 0]
            ),
            1
        );
        assert_eq!(
            call(&mut emu, "kernel32.dll", "WaitForSingleObject", &[timer, 0]),
            WAIT_TIMEOUT
        );
        assert_eq!(
            call(&mut emu, "kernel32.dll", "WaitForSingleObject", &[timer, 5]),
            WAIT_OBJECT_0
        );
        assert_eq!(
            call(&mut emu, "kernel32.dll", "WaitForSingleObject", &[timer, 0]),
            WAIT_TIMEOUT
        );
    }
}

#[test]
fn zero_tls_cell_clears_dynamic_slots_across_live_threads() {
    use helpers::call_winapi_export as call;

    for is64 in [false, true] {
        let (mut emu, data) = helpers::prepare_winapi(is64);
        let main_teb = data + 0x10000;
        let (inline, expansion, static_vector, width) = if is64 {
            (
                TEB64::TLS_SLOTS_OFFSET,
                TEB64::TLS_EXPANSION_SLOTS_OFFSET,
                TEB64::THREAD_LOCAL_STORAGE_POINTER_OFFSET,
                8,
            )
        } else {
            (
                TEB::TLS_SLOTS_OFFSET,
                TEB::TLS_EXPANSION_SLOTS_OFFSET,
                TEB::THREAD_LOCAL_STORAGE_POINTER_OFFSET,
                4,
            )
        };
        emu.maps
            .create_map(
                "teb",
                main_teb,
                TEB64::map_size() as u64,
                Permission::READ_WRITE,
            )
            .unwrap();
        write_pointer(&mut emu, main_teb + inline + width, 0x2222);
        write_pointer(&mut emu, main_teb + expansion, data + 512);
        let child = call(
            &mut emu,
            "kernel32.dll",
            "CreateThread",
            &[0, 0, data, 0, CREATE_SUSPENDED, 0],
        );
        assert_ne!(child, 0);
        let child_teb = emu.threads[1].windows.teb;
        assert_eq!(read_pointer(&emu, child_teb + inline + width), Some(0));
        assert_eq!(read_pointer(&emu, child_teb + expansion), Some(0));
        let main_id = emu.current_thread().id;
        let pseudo = if is64 {
            NT_CURRENT_THREAD
        } else {
            NT_CURRENT_THREAD as u32 as u64
        };
        let denied = call(&mut emu, "kernel32.dll", "OpenThread", &[0, 0, main_id]);
        let own = call(
            &mut emu,
            "kernel32.dll",
            "OpenThread",
            &[THREAD_SET_INFORMATION as u64, 0, main_id],
        );
        let set = |emu: &mut Emu, handle, p, size| {
            call(
                emu,
                "ntdll.dll",
                "NtSetInformationThread",
                &[handle, THREAD_ZERO_TLS_CELL as u64, p, size],
            )
        };
        let slots = [1, TLS_MINIMUM_AVAILABLE + 1];
        emu.windows_tls_slots.resize(slots[1] + 1, true);
        for (thread, teb) in [main_teb, child_teb].into_iter().enumerate() {
            let table = data + 512 + thread as u64 * 128;
            write_pointer(&mut emu, teb + expansion, table);
            write_pointer(&mut emu, teb + static_vector, table + 64);
            write_pointer(&mut emu, table + 64 + width, 0x3333);
            for address in [teb + inline, table] {
                write_pointer(&mut emu, address, 0x1111);
                write_pointer(&mut emu, address + width, 0x2222);
            }
            emu.current_thread_id = thread;
            for slot in slots {
                assert_eq!(
                    call(
                        &mut emu,
                        "kernel32.dll",
                        "TlsSetValue",
                        &[slot as u64, 0x2222]
                    ),
                    1
                );
            }
        }
        emu.current_thread_id = 0;
        emu.maps.write_dword(data, slots[0] as u32);
        for (handle, p, len, expected) in [
            (pseudo, data, 3, STATUS_INFO_LENGTH_MISMATCH),
            (pseudo, data, 8, STATUS_INFO_LENGTH_MISMATCH),
            (pseudo, 0, 4, STATUS_ACCESS_VIOLATION),
            (pseudo, data + 1, 4, STATUS_DATATYPE_MISALIGNMENT),
            (0, data, 4, STATUS_INVALID_HANDLE),
            (denied, data, 4, STATUS_ACCESS_DENIED),
            (child, data, 4, STATUS_INVALID_PARAMETER),
            (
                if is64 {
                    NT_CURRENT_PROCESS
                } else {
                    NT_CURRENT_PROCESS as u32 as u64
                },
                data,
                4,
                STATUS_OBJECT_TYPE_MISMATCH,
            ),
        ] {
            assert_eq!(set(&mut emu, handle, p, len), expected);
        }
        emu.maps.write_dword(data, u32::MAX);
        assert_eq!(set(&mut emu, pseudo, data, 4), STATUS_SUCCESS);
        assert_eq!(read_pointer(&emu, main_teb + inline + width), Some(0x2222));
        for (slot, handle) in [(slots[0], pseudo), (slots[1], own)] {
            emu.maps.write_dword(data, slot as u32);
            assert_eq!(set(&mut emu, handle, data, 4), STATUS_SUCCESS);
        }
        for (thread, teb) in [main_teb, child_teb].into_iter().enumerate() {
            emu.current_thread_id = thread;
            let table = read_pointer(&emu, teb + expansion).unwrap();
            assert_eq!(read_pointer(&emu, teb + static_vector), Some(table + 64));
            assert_eq!(read_pointer(&emu, table + 64 + width), Some(0x3333));
            for address in [teb + inline, table] {
                assert_eq!(read_pointer(&emu, address + width), Some(0));
                assert_eq!(read_pointer(&emu, address), Some(0x1111));
            }
            for slot in slots {
                assert_eq!(
                    call(&mut emu, "kernel32.dll", "TlsGetValue", &[slot as u64]),
                    0
                );
                assert!(emu.windows_tls_slots[slot]);
            }
        }
    }
}

#[test]
fn process_tls_and_vectored_handles_survive_thread_switch_and_snapshot() {
    use helpers::call_winapi_export as call;
    for is64 in [false, true] {
        let (mut emu, _) = helpers::prepare_winapi(is64);
        let mut next = emu.current_thread().clone();
        next.id += 1;
        emu.threads.push(next);
        let slot = call(&mut emu, "kernel32.dll", "TlsAlloc", &[]);
        let callback = 0x1234;
        let first = call(
            &mut emu,
            "ntdll.dll",
            "RtlAddVectoredExceptionHandler",
            &[0, callback],
        );
        let second = call(
            &mut emu,
            "kernel32.dll",
            "AddVectoredExceptionHandler",
            &[1, callback],
        );
        assert_ne!(first, 0);
        assert_ne!(first, second);
        assert_eq!(
            call(&mut emu, "kernelbase.dll", "TlsSetValue", &[slot, 0x5678]),
            1
        );
        emu.current_thread_id = 1;
        assert_eq!(call(&mut emu, "ntdll.dll", "RtlTlsGetValue", &[slot]), 0);
        assert_eq!(
            call(
                &mut emu,
                "kernel32.dll",
                "TlsSetValue",
                &[u64::from(u32::MAX), 1]
            ),
            0
        );
        emu = Serialization::deserialize(&Serialization::serialize(&emu));
        assert_eq!(emu.veh_list(), &[callback, callback]);
        assert_eq!(
            call(
                &mut emu,
                "kernelbase.dll",
                "RemoveVectoredExceptionHandler",
                &[first]
            ),
            1
        );
        assert_eq!(
            call(
                &mut emu,
                "ntdll.dll",
                "RtlRemoveVectoredExceptionHandler",
                &[first]
            ),
            0
        );
        assert_eq!(emu.veh_list(), &[callback]);
        emu.current_thread_id = 0;
        assert_eq!(
            call(&mut emu, "kernelbase.dll", "TlsGetValue", &[slot]),
            0x5678
        );
        assert_eq!(call(&mut emu, "kernel32.dll", "TlsFree", &[slot]), 1);
        assert_eq!(call(&mut emu, "ntdll.dll", "RtlTlsAlloc", &[]), slot);
        assert_eq!(call(&mut emu, "kernel32.dll", "TlsGetValue", &[slot]), 0);
        assert_eq!(
            call(
                &mut emu,
                "kernel32.dll",
                "RemoveVectoredExceptionHandler",
                &[second]
            ),
            1
        );
    }
}

#[test]
fn kernel_context_capture_saves_integer_registers_through_each_dll() {
    for is64 in [false, true] {
        let (mut emu, out) = helpers::prepare_winapi(is64);
        for dll in ["kernel32.dll", "ntdll.dll"] {
            emu.regs_mut().rbx = 0x4567;
            helpers::call_winapi_export(&mut emu, dll, "RtlCaptureContext", &[out]);
            let mut regs = *emu.regs();
            regs.rbx = 0;
            if is64 {
                let mut saved = Context64::new(&regs);
                saved.load(out, &mut emu.maps);
                saved.sync(&mut regs);
            } else {
                let mut saved = Context32::new(&regs);
                saved.load(out as u32, &mut emu.maps);
                saved.sync(&mut regs);
            }
            assert_eq!(regs.rbx, 0x4567);
        }
    }
}

#[test]
fn kernel_address_wait_compares_guest_bytes_and_uses_timeout_as_a_value() {
    use helpers::call_winapi_export as call;
    for is64 in [false, true] {
        let (mut emu, ptr) = helpers::prepare_winapi(is64);
        let compare = ptr + 8;
        emu.cfg.short_circuit_sleep = false;
        emu.maps.write_qword(ptr, 0x800);
        emu.maps.write_qword(compare, 0);
        for dll in ["kernel32.dll", "kernelbase.dll"] {
            assert_eq!(
                call(&mut emu, dll, "WaitOnAddress", &[ptr, compare, 2, 0]),
                1
            );
            let tick = emu.tick;
            assert_eq!(
                call(&mut emu, dll, "WaitOnAddress", &[ptr, compare, 1, 3]),
                0
            );
            assert_eq!(emu.get_last_error(), ERROR_TIMEOUT);
            assert_eq!(emu.tick, tick + 3);
            assert_eq!(
                call(&mut emu, dll, "WaitOnAddress", &[ptr, compare, 3, 0]),
                0
            );
            assert_eq!(emu.get_last_error(), ERROR_INVALID_PARAMETER);
            call(&mut emu, dll, "WakeByAddressSingle", &[ptr]);
            assert_eq!(emu.maps.read_qword(ptr), Some(0x800));
        }
    }
}

// Increment the DWORD addressed by PFLS_CALLBACK_FUNCTION's argument.
fn fls_counter_callback(emu: &mut Emu, is64: bool) -> u64 {
    const CODE: u64 = 0x2800_0000;
    emu.maps
        .create_map("fls_callback", CODE, 4096, Permission::READ_WRITE_EXECUTE)
        .unwrap();
    let body: &[u8] = if is64 {
        &[0xff, 0x01, 0xc3] // inc dword ptr [rcx]; ret
    } else {
        &[0x8b, 0x44, 0x24, 4, 0xff, 0x00, 0xc2, 4, 0] // mov eax,[esp+4]; inc dword ptr [eax]; ret 4
    };
    emu.maps.write_bytes(CODE, body);
    emu.regs_mut().rip = CODE + 64;
    CODE
}

#[test]
fn fls_free_calls_on_the_freeing_thread_and_drains_values_set_by_callbacks() {
    use helpers::call_winapi_export as call;
    for is64 in [false, true] {
        let (mut emu, data) = helpers::prepare_winapi(is64);
        let code = fls_counter_callback(&mut emu, is64);
        let child = data + 4;
        let replacement = data + 8;
        emu.cfg.enable_threading = true;
        emu.maps.write_bytes(code + 64, &[0xeb, 0xfe]); // main thread waits here
        emu.maps.write_byte(code + 96, 0x90); // the runnable child executes a NOP
        call(
            &mut emu,
            "kernel32.dll",
            "CreateThread",
            &[0, 0, code + 96, 0, 0, 0],
        );
        assert_eq!(emu.threads.len(), 2);
        let slot = call(&mut emu, "kernel32.dll", "FlsAlloc", &[code]);
        call(&mut emu, "kernelbase.dll", "FlsSetValue", &[slot, data]);
        emu.current_thread_id = 1;
        call(&mut emu, "kernel32.dll", "FlsSetValue", &[slot, child]);
        emu.current_thread_id = 0;
        emu.set_hook_on_pre_instruction(move |emu, pc, _, _| {
            if pc == code {
                assert_eq!(emu.current_thread_id, 0);
                let saved = *emu.regs();
                let argument = if is64 {
                    saved.rcx
                } else {
                    u64::from(emu.maps.read_dword(saved.rsp + 4).unwrap())
                };
                if argument == data {
                    assert_eq!(call(emu, "kernel32.dll", "FlsGetValue", &[slot]), 0);
                    assert_ne!(call(emu, "kernel32.dll", "FlsAlloc", &[0]), slot);
                    assert_eq!(
                        call(emu, "kernel32.dll", "FlsSetValue", &[slot, replacement]),
                        1
                    );
                }
                *emu.regs_mut() = saved;
            }
            true
        });
        assert_eq!(call(&mut emu, "kernelbase.dll", "FlsFree", &[slot]), 1);
        for address in [data, child, replacement] {
            assert_eq!(emu.maps.read_dword(address), Some(1));
        }
        assert_eq!(emu.current_thread_id, 0);
        assert!(emu.cfg.enable_threading);
        assert_eq!(call(&mut emu, "kernel32.dll", "FlsAlloc", &[0]), slot);
        emu.cfg.max_instructions = Some(emu.instruction_count + 256);
        assert_eq!(emu.run(Some(code + 97)).unwrap(), code + 97);
    }
}

#[test]
fn fls_callback_failure_preserves_the_guest_fault_and_never_returns_success() {
    for is64 in [false, true] {
        let (mut emu, data) = helpers::prepare_winapi(is64);
        let missing = fls_counter_callback(&mut emu, is64) + 0x2000;
        let slot = helpers::call_winapi_export(&mut emu, "kernel32.dll", "FlsAlloc", &[missing]);
        helpers::call_winapi_export(&mut emu, "kernel32.dll", "FlsSetValue", &[slot, data]);
        let free = |emu: &mut Emu| helpers::invoke_winapi_export(emu, "kernel32.dll", "FlsFree");
        if is64 {
            helpers::call_winapi64(&mut emu, free, &[slot]);
        } else {
            helpers::call_winapi32(&mut emu, free, &[slot as u32]);
        }
        assert_eq!(emu.pc(), missing);
        assert!(emu.fault_count > 0);
        assert!(emu.process_terminated);
        assert_eq!(emu.current_thread().exit_value, None);
        assert_eq!(emu.windows_fls_slots[slot as usize], Some(missing));
    }
}

#[test]
fn fls_exit_callbacks_follow_thread_and_process_termination_contracts() {
    use helpers::call_winapi_export as call;
    for (is64, api) in iproduct!(
        [false, true],
        [
            "ExitThread",
            "ExitProcess",
            "NtTerminateThread",
            "TerminateProcess"
        ]
    ) {
        let (mut emu, data) = helpers::prepare_winapi(is64);
        let code = fls_counter_callback(&mut emu, is64);
        let slot = call(&mut emu, "kernel32.dll", "FlsAlloc", &[code]);
        call(&mut emu, "kernel32.dll", "FlsSetValue", &[slot, data]);
        emu.cfg.enable_threading = true;
        let child_handle = call(
            &mut emu,
            "kernel32.dll",
            "CreateThread",
            &[0, 0, code + 64, 0, 0, 0],
        );
        emu.current_thread_id = 1;
        call(&mut emu, "kernel32.dll", "FlsSetValue", &[slot, data + 4]);
        emu.current_thread_id = 0;
        let graceful = matches!(api, "ExitThread" | "ExitProcess");
        emu.set_hook_on_pre_instruction(move |emu, pc, _, _| {
            if graceful && pc == code {
                let saved = *emu.regs();
                assert_eq!(call(emu, "kernel32.dll", "FlsGetValue", &[slot]), data);
                *emu.regs_mut() = saved;
            }
            true
        });
        match api {
            "NtTerminateThread" => {
                call(&mut emu, "ntdll.dll", api, &[child_handle, 23]);
            }
            "TerminateProcess" => {
                call(&mut emu, "kernel32.dll", api, &[NT_CURRENT_PROCESS, 23]);
            }
            _ => {
                call(&mut emu, "kernel32.dll", api, &[23]);
            }
        }
        assert_eq!(
            emu.maps.read_dword(data),
            Some(u32::from(graceful)),
            "{api}"
        );
        assert_eq!(emu.maps.read_dword(data + 4), Some(0), "{api}");
        if api == "NtTerminateThread" {
            assert_eq!(emu.threads[1].exit_value, Some(23));
            assert_eq!(emu.threads[0].exit_value, None);
            // Forced termination retains that fiber's value for a later process-wide FlsFree.
            assert_eq!(call(&mut emu, "kernel32.dll", "FlsFree", &[slot]), 1);
            assert_eq!(emu.maps.read_dword(data), Some(1));
            assert_eq!(emu.maps.read_dword(data + 4), Some(1));
            continue;
        }
        assert_eq!(emu.threads[0].exit_value, Some(23), "{api}");
        assert_eq!(
            emu.threads[1].exit_value.is_some(),
            api.ends_with("Process"),
            "{api}"
        );
    }
}

#[test]
fn fiber_slots_reuse_indices_and_value2_preserves_last_error() {
    use crate::tests::helpers::{call_winapi_export as call, prepare_winapi};
    for is64 in [false, true] {
        let (mut emu, _) = prepare_winapi(is64);
        let kernel = "kernel32.dll";
        let base = "kernelbase.dll";
        let slot = call(&mut emu, kernel, "FlsAlloc", &[0]);
        assert!(slot > 0 && slot < 4080);
        assert_eq!(call(&mut emu, base, "FlsSetValue", &[slot, 123]), 1);
        emu.set_last_error(73);
        assert_eq!(call(&mut emu, kernel, "FlsGetValue2", &[slot]), 123);
        assert_eq!(emu.get_last_error(), 73);
        assert_eq!(call(&mut emu, base, "FlsGetValue2", &[slot + 1]), 0);
        assert_eq!(emu.get_last_error(), 73);
        assert_eq!(call(&mut emu, base, "FlsFree", &[slot]), 1);
        assert_eq!(call(&mut emu, kernel, "FlsAlloc", &[0]), slot);
        assert_eq!(call(&mut emu, base, "FlsGetValue", &[slot]), 0);
        let tls = call(&mut emu, kernel, "TlsAlloc", &[]);
        assert_eq!(call(&mut emu, base, "TlsSetValue", &[tls, 456]), 1);
        emu.set_last_error(73);
        assert_eq!(call(&mut emu, kernel, "TlsGetValue2", &[tls]), 456);
        assert_eq!(emu.get_last_error(), 73);
    }
}

#[test]
fn srw_locks_track_shared_owners_and_exclude_writers() {
    for (wide, library) in iproduct!([false, true], ["kernel32.dll", "kernelbase.dll"]) {
        let (mut emu, lock) = helpers::prepare_winapi(wide);
        let mut invoke = |name: &str| helpers::call_winapi_export(&mut emu, library, name, &[lock]);
        invoke("InitializeSRWLock");
        assert_eq!(invoke("TryAcquireSRWLockShared"), 1);
        assert_eq!(invoke("TryAcquireSRWLockShared"), 1);
        assert_eq!(invoke("TryAcquireSRWLockExclusive"), 0);
        invoke("ReleaseSRWLockShared");
        assert_eq!(invoke("TryAcquireSRWLockExclusive"), 0);
        invoke("ReleaseSRWLockShared");
        assert_eq!(invoke("TryAcquireSRWLockExclusive"), 1);
        assert_eq!(invoke("TryAcquireSRWLockShared"), 0);
        invoke("ReleaseSRWLockExclusive");
        invoke("AcquireSRWLockShared");
        invoke("ReleaseSRWLockShared");
        invoke("AcquireSRWLockExclusive");
        invoke("ReleaseSRWLockExclusive");
        assert_eq!(emu.maps.read_qword(lock), Some(0));
    }
}

#[test]
fn interlocked_exchange_and_add_return_the_previous_long() {
    for (wide, library) in iproduct!([false, true], ["kernel32.dll", "kernelbase.dll"]) {
        let (mut emu, address) = helpers::prepare_winapi(wide);
        emu.maps.write_qword(address, u64::MAX);
        assert_eq!(
            helpers::call_winapi_export(&mut emu, library, "InterlockedExchangeAdd", &[address, 2]),
            u64::from(u32::MAX)
        );
        assert_eq!(
            helpers::call_winapi_export(
                &mut emu,
                library,
                "InterlockedCompareExchange",
                &[address, 8, 0]
            ),
            1
        );
        assert_eq!(
            helpers::call_winapi_export(
                &mut emu,
                library,
                "InterlockedCompareExchange",
                &[address, 8, 1]
            ),
            1
        );
        assert_eq!(
            helpers::call_winapi_export(&mut emu, library, "InterlockedExchange", &[address, 6]),
            8
        );
        assert_eq!(emu.maps.read_qword(address), Some(0xffff_ffff_0000_0006));
    }
}

#[test]
fn timer2_arms_only_a_modifiable_extended_timer_and_obeys_its_deadline() {
    use crate::windows::constants::errors::{
        ERROR_ACCESS_DENIED, STATUS_ACCESS_DENIED, STATUS_ACCESS_VIOLATION,
        STATUS_INVALID_PARAMETER_3, STATUS_OBJECT_TYPE_MISMATCH, STATUS_SUCCESS,
    };
    use crate::windows::constants::process::{WAIT_OBJECT_0, WAIT_TIMEOUT};
    use crate::windows::constants::security::SYNCHRONIZE;
    use crate::windows::constants::system::{TIMER_MODIFY_STATE, TIMER2_ATTRIBUTE_NO_WAKE};
    use helpers::call_winapi_export as call;

    for is64 in [false, true] {
        let (mut emu, data) = helpers::prepare_winapi(is64);
        let due = data + 16;
        let parameters = data + 32;
        emu.maps.write_qword(due, (-50_000i64) as u64);
        emu.maps.write_qword(parameters + 8, 750_000_000);
        let legacy = call(&mut emu, "kernel32.dll", "CreateWaitableTimerW", &[0, 0, 0]);
        assert_eq!(
            call(
                &mut emu,
                "ntdll.dll",
                "NtSetTimer2",
                &[legacy, due, 0, parameters]
            ),
            STATUS_OBJECT_TYPE_MISMATCH
        );
        for (access, expected) in [
            (SYNCHRONIZE, STATUS_ACCESS_DENIED),
            (SYNCHRONIZE | TIMER_MODIFY_STATE, STATUS_SUCCESS),
        ] {
            assert_eq!(
                call(
                    &mut emu,
                    "ntdll.dll",
                    "NtCreateTimer2",
                    &[data, 0, 0, TIMER2_ATTRIBUTE_NO_WAKE as u64, access as u64]
                ),
                STATUS_SUCCESS
            );
            let handle = read_pointer(&emu, data).unwrap();
            assert_eq!(
                call(
                    &mut emu,
                    "ntdll.dll",
                    "ZwSetTimer2",
                    &[handle, due, 0, parameters]
                ),
                expected
            );
            if expected == STATUS_ACCESS_DENIED {
                assert_eq!(
                    call(
                        &mut emu,
                        "kernel32.dll",
                        "SetWaitableTimer",
                        &[handle, due, 0, 0, 0, 0]
                    ),
                    0
                );
                assert_eq!(emu.get_last_error(), ERROR_ACCESS_DENIED);
            }
            if expected == STATUS_SUCCESS {
                emu.maps.write_dword(parameters, 1);
                assert_eq!(
                    call(
                        &mut emu,
                        "ntdll.dll",
                        "NtSetTimer2",
                        &[handle, due, 0, parameters]
                    ),
                    STATUS_INVALID_PARAMETER_3
                );
                assert_eq!(
                    call(&mut emu, "ntdll.dll", "NtSetTimer2", &[handle, due, 0, 1]),
                    STATUS_ACCESS_VIOLATION
                );
                assert_eq!(
                    call(
                        &mut emu,
                        "kernel32.dll",
                        "WaitForSingleObject",
                        &[handle, 4]
                    ),
                    WAIT_TIMEOUT
                );
                assert_eq!(
                    call(
                        &mut emu,
                        "kernel32.dll",
                        "WaitForSingleObject",
                        &[handle, 1]
                    ),
                    WAIT_OBJECT_0
                );
                assert_eq!(
                    call(
                        &mut emu,
                        "kernel32.dll",
                        "WaitForSingleObject",
                        &[handle, 0]
                    ),
                    WAIT_TIMEOUT
                );
            }
            assert_eq!(
                call(&mut emu, "ntdll.dll", "NtClose", &[handle]),
                STATUS_SUCCESS
            );
        }
    }
}
