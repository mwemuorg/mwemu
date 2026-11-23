use crate::api::abi::ArgumentFault;
use crate::api::unix::ApiStopReason as UnixStopReason;
use crate::api::windows::policy::{ApiDisposition, ApiStopReason as WindowsStopReason};
use crate::emu::Emu;
use crate::kernel::guard::FindingKind;
use crate::kernel::heap::Region;
use crate::kernel::windows::crt::{ArgSource, format_args};
use crate::kernel::{KernelOs, linux, macos};
use crate::maps::mem64::Permission;
use crate::maps::virtual_memory::PAGE_SIZE;
use crate::serialization::Serialization;
use crate::syscall::linux::constants::errors::E2BIG;
use crate::windows::constants::crt::{_TRUNCATE, ERANGE, STRUNCATE};
use crate::{ApiMode, emu_aarch64, emu64};

const CODE: u64 = 0x400000;
const DATA: u64 = 0x500000;
const STACK: u64 = 0x600000;
const RESULT: u64 = 0x1234;
const ALLOCATION_SIZE: u64 = 16;
const RETURN_ADDRESS: u64 = CODE + 16;
const X64_SHADOW_SPACE: u64 = 4 * size_of::<u64>() as u64;

fn emulator(os: KernelOs) -> Emu {
    let mut emu = emu64();
    emu.cfg.console_enabled = false;
    emu.running_script = true;
    emu.kernel_init(os);
    for (name, base) in [("kernel.test.code", CODE), ("kernel.test.data", DATA)] {
        emu.maps
            .create_map(name, base, PAGE_SIZE, Permission::READ_WRITE_EXECUTE)
            .unwrap();
    }
    emu.regs_mut().rip = CODE;
    emu.regs_mut().rax = RESULT;
    emu
}

fn return_slot(emu: &mut Emu) {
    emu.maps
        .create_map(
            "kernel.test.frame",
            STACK,
            size_of::<u64>() as u64,
            Permission::READ_WRITE,
        )
        .unwrap();
    assert!(emu.maps.write_qword(STACK, RETURN_ADDRESS));
    emu.regs_mut().rsp = STACK;
}

#[test]
fn linux_heap_guard_distinguishes_live_access_and_freed_allocations() {
    let mut emu = emulator(KernelOs::Linux);
    let ptr = emu.kernel_alloc(Region::Slab, ALLOCATION_SIZE, "test", "kmalloc", true);
    assert_ne!(ptr, 0);
    emu.kernel_guard_access(CODE, ptr, 1, false);
    assert!(emu.kernel_findings().is_empty());

    assert!(emu.kernel_free(ptr, "kfree"));
    emu.kernel_guard_access(CODE, ptr, 1, false);
    emu.kernel_guard_access(CODE, ptr, 1, true);
    assert!(!emu.kernel_free(ptr, "kfree"));
    let kinds: Vec<_> = emu.kernel_findings().iter().map(|f| f.kind).collect();
    assert_eq!(
        kinds,
        [
            FindingKind::UseAfterFree,
            FindingKind::WriteAfterFree,
            FindingKind::DoubleFree,
        ]
    );
}

#[test]
fn unix_kernel_rejects_unreadable_return_before_handler_effects() {
    for (os, symbol) in [
        (KernelOs::Linux, "__kmalloc"),
        (KernelOs::MacOS, "IOMalloc"),
    ] {
        let mut emu = emulator(os);
        let stub = emu.kernel_resolve_import(symbol).unwrap();
        emu.regs_mut().rsp = STACK;
        emu.regs_mut().rdi = ALLOCATION_SIZE;
        assert!(emu.kernel_set_pc(stub));
        assert_eq!(
            emu.kernel_alloc_count(),
            0,
            "{os:?} handler ran after a frame error"
        );
        assert_eq!((emu.pc(), emu.sp(), emu.regs().rax), (CODE, STACK, RESULT));
        let fault = emu
            .unix_api
            .fault
            .as_ref()
            .expect("retained return-frame fault");
        assert_eq!(fault.reason, UnixStopReason::InvalidReturnAddress);
        assert_eq!(fault.call.name, symbol);
        assert!(emu.run(None).is_err());
    }
}

#[test]
fn kernel_argument_fault_stops_before_allocation_or_output() {
    for (os, symbol, mode) in [
        (KernelOs::Windows, "IoCreateDevice", ApiMode::Strict),
        (KernelOs::Windows, "IoCreateDevice", ApiMode::SimulateFail),
        (
            KernelOs::Windows,
            "IoCreateDevice",
            ApiMode::SimulateSuccess,
        ),
        (KernelOs::Linux, "usb_control_msg", ApiMode::Strict),
    ] {
        let mut emu = emulator(os);
        emu.set_api_mode(mode);
        return_slot(&mut emu);
        let stub = emu.kernel_resolve_import(symbol).unwrap();
        emu.regs_mut().rdx = ALLOCATION_SIZE;
        emu.regs_mut().r8 = DATA;
        assert!(emu.maps.write_qword(DATA, RESULT));
        assert!(emu.kernel_set_pc(stub));
        assert!(emu.api_stopped(), "{os:?} ignored a missing argument");
        assert_eq!(emu.kernel_alloc_count(), 0);
        assert!(emu.kernel.as_ref().unwrap().unimplemented.is_empty());
        assert_eq!(emu.api_policy.simulation_count, 0);
        assert_eq!(emu.maps.read_qword(DATA), Some(RESULT));
        assert_eq!((emu.pc(), emu.sp(), emu.regs().rax), (CODE, STACK, RESULT));
        match os {
            KernelOs::Windows => assert!(
                matches!(emu.api_policy.last_outcome.as_ref().map(|outcome| &outcome.disposition), Some(ApiDisposition::Stopped(WindowsStopReason::InvalidArgument(error))) if error.fault == ArgumentFault::Unreadable)
            ),
            _ => assert!(
                matches!(emu.unix_api.fault.as_ref().map(|fault| fault.reason), Some(UnixStopReason::InvalidArgument(error)) if error.fault == ArgumentFault::Unreadable)
            ),
        }
        assert!(emu.run(None).is_err());
    }
}

#[test]
fn linux_surface_includes_connected_handlers() {
    for symbol in ["unregister_chrdev", "usb_register_driver", "free_netdev"] {
        assert!(linux::is_implemented(symbol), "missing {symbol}");
        assert!(
            linux::SURFACE
                .iter()
                .any(|(_, names)| names.contains(&symbol))
        );
    }
}

#[test]
fn kernel_zero_arguments_and_aarch64_link_returns_remain_valid() {
    for (os, symbol) in [
        (KernelOs::Linux, "__kmalloc"),
        (KernelOs::MacOS, "IOMalloc"),
    ] {
        let mut emu = emulator(os);
        return_slot(&mut emu);
        let stub = emu.kernel_resolve_import(symbol).unwrap();
        emu.regs_mut().rdi = 0;
        assert!(emu.kernel_set_pc(stub));
        assert!(!emu.api_stopped());
        if os == KernelOs::Linux {
            assert_eq!(emu.regs().rax, 0);
        } else {
            assert!(emu.maps.read_byte(emu.regs().rax).is_some());
        }
        assert_eq!(
            (emu.pc(), emu.sp()),
            (RETURN_ADDRESS, STACK + size_of::<u64>() as u64)
        );

        let mut arm = emu_aarch64();
        arm.cfg.console_enabled = false;
        arm.running_script = true;
        arm.kernel_init(os);
        let stub = arm.kernel_resolve_import(symbol).unwrap();
        arm.set_sp(STACK);
        arm.regs_aarch64_mut().x[0] = ALLOCATION_SIZE;
        arm.regs_aarch64_mut().x[30] = RETURN_ADDRESS;
        assert!(arm.kernel_set_pc(stub));
        assert!(!arm.api_stopped());
        assert_eq!((arm.pc(), arm.sp()), (RETURN_ADDRESS, STACK));
        assert_eq!(arm.kernel_alloc_count(), 1);
    }
}

#[test]
fn linux_surface_preserves_exact_aliases_and_prefix_models() {
    let mut emu = emulator(KernelOs::Linux);
    for symbol in [
        "unregister_chrdev",
        "usb_register_driver",
        "free_netdev",
        "netif_future_operation",
        "__netif_future_operation",
        "ethtool_future_operation",
    ] {
        assert!(linux::is_implemented(symbol), "{symbol}");
        assert!(linux::gateway(symbol, &mut emu), "{symbol}");
    }
    for symbol in [
        "UnknownKernelFunction",
        "netiffuture_operation",
        "ethtoolfuture_operation",
        "jiffies",
    ] {
        assert!(!linux::is_implemented(symbol), "{symbol}");
        assert!(!linux::gateway(symbol, &mut emu), "{symbol}");
    }
    assert!(linux::data_symbol_size("jiffies").is_some());
}

#[test]
fn kernel_argument_errors_reach_execution_and_snapshots() {
    for execution in ["step", "run", "threaded", "host_call"] {
        let mut emu = emulator(KernelOs::Linux);
        return_slot(&mut emu);
        let stub = emu.kernel_resolve_import("usb_control_msg").unwrap();
        emu.cfg.enable_threading = execution == "threaded";
        emu.regs_mut().rbx = RESULT;
        emu.regs_mut().r11 = stub;
        emu.set_sp(STACK + size_of::<u64>() as u64);
        // call r11; xor ebx, ebx; ret.
        assert!(
            emu.maps
                .write_bytes(CODE, &[0x41, 0xff, 0xd3, 0x31, 0xdb, 0xc3])
        );
        let expected_pc = if execution == "host_call" {
            // The driver replaces its stack, then reaches the same import call.
            let mut code = vec![0x48, 0xbc]; // mov rsp, frame_end
            code.extend_from_slice(&(STACK + size_of::<u64>() as u64).to_le_bytes());
            code.extend_from_slice(&[0x41, 0xff, 0xd3, 0x31, 0xdb, 0xc3]);
            assert!(emu.maps.write_bytes(CODE, &code));
            assert!(emu.kernel_call(CODE, &[]).is_err());
            CODE + 10
        } else {
            if execution == "step" {
                assert!(!emu.step());
            } else {
                assert!(emu.run(Some(CODE + 5)).is_err());
            }
            CODE
        };
        let fault = emu.unix_api.fault.clone().expect("kernel argument fault");
        assert_eq!(fault.call.name, "usb_control_msg");
        assert_eq!(
            (fault.call.pc, fault.call.stack_pointer),
            (expected_pc, STACK)
        );
        assert_eq!((emu.pc(), emu.sp()), (expected_pc, STACK));
        assert_eq!(emu.regs().rbx, RESULT, "{execution}");
        let mut restored = Serialization::deserialize(&Serialization::serialize(&emu));
        assert_eq!(restored.unix_api.fault.as_ref(), Some(&fault));
        assert!(restored.run(None).is_err());
    }
}

#[test]
fn failed_key_argument_does_not_publish_a_handle() {
    let mut emu = emulator(KernelOs::Windows);
    return_slot(&mut emu);
    let stub = emu.kernel_resolve_import("ZwCreateKey").unwrap();
    emu.regs_mut().rcx = DATA;
    emu.regs_mut().r8 = 0;
    assert!(emu.maps.write_qword(DATA, RESULT));
    assert!(emu.kernel_set_pc(stub));
    assert!(emu.api_stopped());
    assert!(emu.kernel.as_ref().unwrap().win.handles.is_empty());
    assert_eq!(emu.maps.read_qword(DATA), Some(RESULT));
    assert_eq!(emu.regs().rax, RESULT);
}

#[test]
fn kernel_formatters_stop_on_unreadable_arguments_without_outputs() {
    for (os, symbol, format) in [
        (KernelOs::Windows, "DbgPrint", "%u %u %u %u"),
        (KernelOs::Linux, "printk", "%u %u %u %u %u %u"),
    ] {
        let mut emu = emulator(os);
        return_slot(&mut emu);
        emu.maps.write_string(DATA, format);
        if os == KernelOs::Windows {
            emu.regs_mut().rcx = DATA;
        } else {
            emu.regs_mut().rdi = DATA;
        }
        let stub = emu.kernel_resolve_import(symbol).unwrap();
        assert!(emu.kernel_set_pc(stub));
        assert!(emu.api_stopped());
        assert!(emu.kernel.as_ref().unwrap().log.is_empty());
        assert_eq!(emu.regs().rax, RESULT);
    }
    let mut emu = emulator(KernelOs::Windows);
    assert!(emu.maps.write_dword(DATA, RESULT as u32));
    emu.regs_mut().rcx = DATA;
    emu.set_sp(STACK);
    let mut args = ArgSource::Call(0);
    assert!(format_args(&mut emu, "%n%u%u%u%u", &mut args, false).is_err());
    assert_eq!(emu.maps.read_dword(DATA), Some(RESULT as u32));
    let mut args = ArgSource::VaList {
        base: u64::MAX,
        index: 0,
    };
    assert!(format_args(&mut emu, "%u", &mut args, false).is_err());
    let mut args = ArgSource::VaList {
        base: STACK,
        index: 0,
    };
    assert_eq!(
        format_args(&mut emu, "literal", &mut args, false).unwrap(),
        "literal"
    );
}

#[test]
fn aarch64_kernel_formatters_use_the_platform_variadic_layout() {
    for (os, symbol, format, expected) in [
        (
            KernelOs::Linux,
            "printk",
            "%u %u %u %u %u %u %u %u",
            "1 2 3 4 5 6 7 42",
        ),
        (KernelOs::MacOS, "printf", "%u", "42"),
    ] {
        let mut emu = emu_aarch64();
        emu.cfg.console_enabled = false;
        emu.kernel_init(os);
        emu.maps
            .create_map("format.data", DATA, PAGE_SIZE, Permission::READ_WRITE)
            .unwrap();
        emu.maps.write_string(DATA, format);
        let stack = DATA + PAGE_SIZE - size_of::<u64>() as u64;
        assert!(emu.maps.write_qword(stack, 42));
        emu.set_sp(stack);
        emu.regs_aarch64_mut().x[0] = DATA;
        for index in 1..8 {
            emu.regs_aarch64_mut().x[index] = index as u64;
        }
        emu.regs_aarch64_mut().x[30] = RETURN_ADDRESS;
        let stub = emu.kernel_resolve_import(symbol).unwrap();
        assert!(emu.kernel_set_pc(stub));
        assert!(!emu.api_stopped());
        assert_eq!(emu.kernel.as_ref().unwrap().log, [expected]);
        assert_eq!(emu.sp(), stack);
    }
}

#[test]
fn valid_format_count_writes_remain_visible_to_later_conversions() {
    let mut emu = emulator(KernelOs::Windows);
    emu.maps.write_string(DATA, "hello");
    emu.regs_mut().rcx = DATA;
    emu.regs_mut().rdx = DATA;
    let mut args = ArgSource::Call(0);
    assert_eq!(
        format_args(&mut emu, "abc%n%s", &mut args, false).unwrap(),
        "abc\u{3}"
    );
    assert_eq!(emu.maps.read_dword(DATA), Some(3));
}

#[test]
fn kernel_u32_stack_arguments_do_not_read_padding() {
    for (symbol, stack_arguments) in [
        ("HalGetBusData", vec![size_of::<u32>() as u32]),
        ("HalGetBusDataByOffset", vec![0, size_of::<u32>() as u32]),
    ] {
        let mut emu = emulator(KernelOs::Windows);
        let first_argument = STACK + size_of::<u64>() as u64 + X64_SHADOW_SPACE;
        let last_argument =
            first_argument + (stack_arguments.len() - 1) as u64 * size_of::<u64>() as u64;
        let frame_end = last_argument + size_of::<u32>() as u64;
        emu.maps
            .create_map(
                "kernel.test.frame",
                STACK,
                frame_end - STACK,
                Permission::READ_WRITE,
            )
            .unwrap();
        assert!(emu.maps.write_qword(STACK, RETURN_ADDRESS));
        for (index, value) in stack_arguments.into_iter().enumerate() {
            assert!(emu.maps.write_dword(
                first_argument + index as u64 * size_of::<u64>() as u64,
                value
            ));
        }
        assert!(emu.maps.read_qword(last_argument).is_none());
        emu.set_sp(STACK);
        emu.regs_mut().r9 = DATA;
        let stub = emu.kernel_resolve_import(symbol).unwrap();
        assert!(emu.kernel_set_pc(stub));
        assert!(!emu.api_stopped(), "{symbol}");
        assert_eq!(emu.regs().rax, size_of::<u32>() as u64);
        assert_eq!(
            (emu.pc(), emu.sp()),
            (RETURN_ADDRESS, STACK + size_of::<u64>() as u64)
        );
    }
}

#[test]
fn kernel_bounded_strings_preserve_bytes_and_copy_contracts() {
    const SOURCE: u64 = DATA;
    const DESTINATION: u64 = DATA + 0x100;
    const CANARY: u8 = 0xcc;
    for os in [KernelOs::Linux, KernelOs::MacOS, KernelOs::Windows] {
        let mut emu = emulator(os);
        return_slot(&mut emu);
        for (name, source, capacity, output, expected) in [
            (
                "strncpy",
                b"ABCD\0".as_slice(),
                3,
                b"ABC\xcc".as_slice(),
                DESTINATION,
            ),
            (
                "strncpy",
                b"A\0".as_slice(),
                3,
                b"A\0\0\xcc".as_slice(),
                DESTINATION,
            ),
            (
                "strncpy",
                b"\xc3\xa9\0".as_slice(),
                1,
                b"\xc3\xcc".as_slice(),
                DESTINATION,
            ),
            (
                "strncpy",
                b"A\0".as_slice(),
                0,
                b"\xcc".as_slice(),
                DESTINATION,
            ),
        ] {
            assert!(emu.maps.write_bytes(SOURCE, source));
            assert!(emu.maps.fill_range(DESTINATION, CANARY, 16));
            let stub = emu.kernel_resolve_import(name).unwrap();
            emu.regs_mut().rsp = STACK;
            if os == KernelOs::Windows {
                let regs = emu.regs_mut();
                [regs.rcx, regs.rdx, regs.r8] = [DESTINATION, SOURCE, capacity];
            } else {
                let regs = emu.regs_mut();
                [regs.rdi, regs.rsi, regs.rdx] = [DESTINATION, SOURCE, capacity];
            }
            assert!(emu.kernel_set_pc(stub));
            assert_eq!(emu.regs().rax, expected, "{os:?}: {name}");
            assert_eq!(
                emu.maps.read_bytes(DESTINATION, output.len()).as_ref(),
                output,
                "{os:?}: {name}, capacity {capacity}"
            );
        }
    }
}

#[test]
fn unix_kernel_counted_copies_and_append_keep_distinct_results() {
    const SOURCE: u64 = DATA;
    const DESTINATION: u64 = DATA + 0x100;
    for os in [KernelOs::Linux, KernelOs::MacOS] {
        let mut emu = emulator(os);
        return_slot(&mut emu);
        for (name, source, capacity, output, expected) in [
            (
                "strlcpy",
                b"ABCD\0".as_slice(),
                3,
                b"AB\0\xcc".as_slice(),
                4,
            ),
            ("strlcpy", b"ABCD\0".as_slice(), 0, b"\xcc".as_slice(), 4),
            (
                "strscpy",
                b"ABCD\0".as_slice(),
                3,
                b"AB\0\xcc".as_slice(),
                E2BIG,
            ),
            (
                "sized_strscpy",
                b"ABCD\0".as_slice(),
                3,
                b"AB\0\xcc".as_slice(),
                E2BIG,
            ),
            (
                "strscpy_pad",
                b"A\0".as_slice(),
                4,
                b"A\0\0\0\xcc".as_slice(),
                1,
            ),
            ("strscpy", b"A\0".as_slice(), 0, b"\xcc".as_slice(), E2BIG),
        ] {
            assert!(emu.maps.write_bytes(SOURCE, source));
            assert!(emu.maps.fill_range(DESTINATION, 0xcc, 16));
            let stub = emu.kernel_resolve_import(name).unwrap();
            let regs = emu.regs_mut();
            regs.rsp = STACK;
            [regs.rdi, regs.rsi, regs.rdx] = [DESTINATION, SOURCE, capacity];
            assert!(emu.kernel_set_pc(stub));
            if os == KernelOs::MacOS && matches!(name, "strscpy" | "strscpy_pad" | "sized_strscpy")
            {
                assert!(
                    emu.kernel
                        .as_ref()
                        .unwrap()
                        .unimplemented
                        .iter()
                        .any(|s| s == name)
                );
                assert_eq!(emu.regs().rax, 0);
                assert_eq!(emu.maps.read_bytes(DESTINATION, 16).as_ref(), &[0xcc; 16]);
                continue;
            }
            assert_eq!(emu.regs().rax, expected, "{os:?}: {name}");
            assert_eq!(
                emu.maps.read_bytes(DESTINATION, output.len()).as_ref(),
                output,
                "{os:?}: {name}, capacity {capacity}"
            );
        }
        assert!(emu.maps.write_bytes(SOURCE, b"BCDE\0"));
        assert!(emu.maps.write_bytes(DESTINATION, b"A\0\xcc\xcc\xcc"));
        let stub = emu.kernel_resolve_import("strncat").unwrap();
        let regs = emu.regs_mut();
        regs.rsp = STACK;
        [regs.rdi, regs.rsi, regs.rdx] = [DESTINATION, SOURCE, 2];
        assert!(emu.kernel_set_pc(stub));
        assert_eq!(emu.regs().rax, DESTINATION);
        assert_eq!(emu.maps.read_bytes(DESTINATION, 5).as_ref(), b"ABC\0\xcc");
    }
}

#[test]
fn windows_kernel_memory_comparison_keeps_faults_distinct() {
    let mut emu = emulator(KernelOs::Windows);
    return_slot(&mut emu);
    let other = DATA + 32;
    assert!(emu.maps.write_bytes(DATA, b"ab\0x"));
    assert!(emu.maps.write_bytes(other, b"ab\0y"));
    for (symbol, left, right, length, expected) in [
        ("RtlCompareMemory", DATA, other, 4, 3),
        ("RtlCompareMemory", DATA, DATA, 4, 4),
        ("RtlCompareMemory", 0, 0, 0, 0),
        ("RtlEqualMemory", DATA, other, 4, 0),
        ("RtlEqualMemory", DATA, DATA, 4, 1),
        ("RtlEqualMemory", 0, 0, 0, 1),
    ] {
        let stub = emu.kernel_resolve_import(symbol).unwrap();
        let regs = emu.regs_mut();
        regs.rsp = STACK;
        [regs.rcx, regs.rdx, regs.r8] = [left, right, length];
        assert!(emu.kernel_set_pc(stub));
        assert_eq!(emu.regs().rax, expected, "{symbol}");
    }
    for symbol in ["RtlCompareMemory", "RtlEqualMemory"] {
        let mut emu = emulator(KernelOs::Windows);
        return_slot(&mut emu);
        let stub = emu.kernel_resolve_import(symbol).unwrap();
        let faults = emu.fault_count;
        let regs = emu.regs_mut();
        [regs.rcx, regs.rdx, regs.r8] = [0, 0, 4];
        assert!(emu.kernel_set_pc(stub));
        assert_eq!(
            emu.fault_count,
            faults + 1,
            "{symbol}: unreadable bytes are not equal"
        );
    }
}

#[test]
fn windows_kernel_strings_use_byte_or_utf16_limits_and_secure_capacities() {
    let mut emu = emulator(KernelOs::Windows);
    return_slot(&mut emu);
    let destination = DATA + 256;
    for wide in [false, true] {
        let encode = |text: &str| {
            if wide {
                text.encode_utf16()
                    .flat_map(u16::to_le_bytes)
                    .collect::<Vec<_>>()
            } else {
                text.as_bytes().to_vec()
            }
        };
        assert!(emu.maps.write_bytes(DATA, &encode("ABCD\0")));
        for (capacity, count, expected, output) in [
            (3, 4, ERANGE as u64, "\0"),
            (3, _TRUNCATE, STRUNCATE as u64, "AB\0"),
            (3, 2, 0, "AB\0"),
            (5, 4, 0, "ABCD\0"),
        ] {
            assert!(emu.maps.fill_range(destination, 0xcc, 32));
            let name = if wide { "wcsncpy_s" } else { "strncpy_s" };
            let stub = emu.kernel_resolve_import(name).unwrap();
            let regs = emu.regs_mut();
            regs.rsp = STACK;
            [regs.rcx, regs.rdx, regs.r8, regs.r9] = [destination, capacity, DATA, count];
            assert!(emu.kernel_set_pc(stub));
            assert_eq!(
                emu.regs().rax,
                expected,
                "{name}, capacity={capacity}, count={count}"
            );
            let bytes = encode(output);
            assert_eq!(
                emu.maps.read_bytes(destination, bytes.len()).as_ref(),
                bytes
            );
            assert_eq!(
                emu.maps.read_byte(destination + bytes.len() as u64),
                Some(0xcc)
            );
        }
    }
    for (name, left, right, limit) in [
        (
            "strncmp",
            b"\xc3\xa9\0".as_slice(),
            b"\xc3\xaa\0".as_slice(),
            1,
        ),
        (
            "wcsncmp",
            &[0x3d, 0xd8, 0x00, 0xde, 0, 0],
            &[0x3d, 0xd8, 0x01, 0xde, 0, 0],
            1,
        ),
    ] {
        assert!(emu.maps.write_bytes(DATA, left));
        assert!(emu.maps.write_bytes(destination, right));
        let stub = emu.kernel_resolve_import(name).unwrap();
        let regs = emu.regs_mut();
        regs.rsp = STACK;
        [regs.rcx, regs.rdx, regs.r8] = [DATA, destination, limit];
        assert!(emu.kernel_set_pc(stub));
        assert_eq!(emu.regs().rax, 0, "{name} compares raw units");
    }
    assert!(emu.maps.write_bytes(DATA, &[0x3d, 0xd8, 0x00, 0xde, 0, 0]));
    assert!(emu.maps.fill_range(destination, 0xcc, 16));
    let stub = emu.kernel_resolve_import("wcsncpy").unwrap();
    let regs = emu.regs_mut();
    regs.rsp = STACK;
    [regs.rcx, regs.rdx, regs.r8] = [destination, DATA, 1];
    assert!(emu.kernel_set_pc(stub));
    assert_eq!(
        emu.maps.read_bytes(destination, 4).as_ref(),
        &[0x3d, 0xd8, 0xcc, 0xcc]
    );
}

fn unix_kernel_call(emu: &mut Emu, name: &str, args: [u64; 3]) -> u64 {
    let stub = emu.kernel_resolve_import(name).unwrap();
    let regs = emu.regs_mut();
    regs.rsp = STACK;
    [regs.rdi, regs.rsi, regs.rdx] = args;
    assert!(emu.kernel_set_pc(stub));
    emu.regs().rax
}

#[test]
fn linux_user_copies_return_the_residual_and_preserve_the_guard_byte() {
    use crate::syscall::linux::constants::errors::EFAULT;
    let mut emu = emulator(KernelOs::Linux);
    return_slot(&mut emu);
    let tail = DATA + PAGE_SIZE - 2;
    assert!(emu.maps.write_bytes(tail, b"XY"));
    for (symbol, bytes) in [
        ("copy_from_user", b"XY\0\0!"),
        ("__copy_from_user", b"XY!!!"),
        ("copy_to_user", b"XY!!!"),
    ] {
        assert!(emu.maps.fill_range(DATA, b'!', 5));
        assert_eq!(unix_kernel_call(&mut emu, symbol, [DATA, tail, 4]), 2);
        assert_eq!(emu.maps.read_bytes(DATA, 5).as_ref(), bytes, "{symbol}");
    }
    assert_eq!(
        unix_kernel_call(&mut emu, "strncpy_from_user", [DATA, tail, 2]),
        2
    );
    assert_eq!(
        unix_kernel_call(&mut emu, "strncpy_from_user", [DATA, tail, 3]),
        EFAULT
    );
    assert_eq!(
        unix_kernel_call(&mut emu, "memdup_user", [tail, 3, 0]),
        EFAULT
    );
    let duplicate = unix_kernel_call(&mut emu, "memdup_user_nul", [tail, 2, 0]);
    assert_eq!(emu.maps.read_bytes(duplicate, 3).as_ref(), b"XY\0");
    assert_eq!(unix_kernel_call(&mut emu, "clear_user", [tail, 3, 0]), 1);
    assert_eq!(emu.maps.read_bytes(tail, 2).as_ref(), b"\0\0");
}

#[test]
fn unix_kernel_byte_operations_stop_on_bad_memory_and_use_byte_offsets() {
    for os in [KernelOs::Linux, KernelOs::MacOS] {
        let mut emu = emulator(os);
        return_slot(&mut emu);
        assert!(emu.maps.write_bytes(DATA, b"\xffQ\xff\0"));
        assert_eq!(unix_kernel_call(&mut emu, "strchr", [DATA, 0, 0]), DATA + 3);
        assert_eq!(
            unix_kernel_call(&mut emu, "strrchr", [DATA, 0xff, 0]),
            DATA + 2
        );
        assert_eq!(
            unix_kernel_call(&mut emu, "strstr", [DATA, DATA + 1, 0]),
            DATA + 1
        );
        assert_eq!(unix_kernel_call(&mut emu, "strnlen", [DATA, 1, 0]), 1);
        let tail = DATA + PAGE_SIZE - 1;
        assert!(emu.maps.write_byte(tail, 1));
        assert_eq!(unix_kernel_call(&mut emu, "memcpy", [DATA, tail, 2]), 1);
        assert!(emu.api_stopped(), "{os:?}");
        assert_eq!(emu.maps.read_byte(DATA), Some(0xff));
        assert_eq!(emu.sp(), STACK);
    }
}

#[test]
fn kernel_formatted_output_observes_capacity_and_reports_the_correct_length() {
    let mut emu = emulator(KernelOs::Linux);
    return_slot(&mut emu);
    let fmt = DATA + 64;
    assert!(emu.maps.write_bytes(fmt, b"A%cCD\0"));
    for (name, size, result, output) in [
        ("snprintf", 0, 4, b"!!!!!".as_slice()),
        ("scnprintf", 0, 0, b"!!!!!"),
        ("snprintf", 3, 4, b"A\xff\0!!"),
        ("scnprintf", 3, 2, b"A\xff\0!!"),
        ("snprintf", 5, 4, b"A\xffCD\0"),
    ] {
        assert!(emu.maps.fill_range(DATA, b'!', 5));
        emu.regs_mut().rcx = 0xff;
        let dst = if size == 0 { 0 } else { DATA };
        assert_eq!(unix_kernel_call(&mut emu, name, [dst, size, fmt]), result);
        assert_eq!(emu.maps.read_bytes(DATA, 5).as_ref(), output);
    }
}

#[test]
fn kernel_numeric_strings_honor_radix_width_and_error_outputs() {
    use crate::syscall::linux::constants::errors::{EINVAL, ERANGE};
    let mut emu = emulator(KernelOs::Linux);
    return_slot(&mut emu);
    let out = DATA + 128;
    for (name, input, base, status, expected) in [
        (
            "kstrtoint",
            b"-0x20\n\0".as_slice(),
            0,
            0,
            0xcccc_cccc_ffff_ffe0,
        ),
        ("kstrtou64", b"18446744073709551615\0", 10, 0, u64::MAX),
        (
            "kstrtouint",
            b"4294967296\0",
            10,
            ERANGE,
            0xcccc_cccc_cccc_cccc,
        ),
        ("kstrtouint", b"-1\0", 10, EINVAL, 0xcccc_cccc_cccc_cccc),
        ("kstrtol", b"1234 \0", 10, EINVAL, 0xcccc_cccc_cccc_cccc),
        ("kstrtoint", b"10\0", 99, EINVAL, 0xcccc_cccc_cccc_cccc),
    ] {
        assert!(emu.maps.write_bytes(DATA, input));
        assert!(emu.maps.fill_range(out, 0xcc, 8));
        assert_eq!(unix_kernel_call(&mut emu, name, [DATA, base, out]), status);
        assert_eq!(emu.maps.read_qword(out), Some(expected), "{name}");
    }
    assert!(emu.maps.write_bytes(DATA, b"-030!\0"));
    assert_eq!(
        unix_kernel_call(&mut emu, "simple_strtol", [DATA, out, 0]),
        (-24i64) as u64
    );
    assert_eq!(emu.maps.read_qword(out), Some(DATA + 4));
}

#[test]
fn secure_driver_copies_check_capacity_and_handle_the_destination_by_contract() {
    let mut emu = emulator(KernelOs::Windows);
    return_slot(&mut emu);
    let src = DATA + 128;
    assert!(emu.maps.write_bytes(src, b"abcd"));
    for (name, cap, len, expected, output) in [
        ("memcpy_s", 2, 3, ERANGE, b"\0\0!!"),
        ("memmove_s", 2, 3, ERANGE, b"!!!!"),
        ("memcpy_s", 4, 3, 0, b"abc!"),
        ("memmove_s", 4, 3, 0, b"abc!"),
    ] {
        assert!(emu.maps.write_bytes(DATA, b"!!!!"));
        let stub = emu.kernel_resolve_import(name).unwrap();
        let regs = emu.regs_mut();
        regs.rsp = STACK;
        [regs.rcx, regs.rdx, regs.r8, regs.r9] = [DATA, cap, src, len];
        assert!(emu.kernel_set_pc(stub));
        assert_eq!(emu.regs().rax, expected as u64, "{name}");
        assert_eq!(emu.maps.read_bytes(DATA, 4).as_ref(), output, "{name}");
    }
}

#[test]
fn darwin_kernel_discovery_matches_allocation_and_format_contracts() {
    for (_, names) in macos::SURFACE {
        for name in *names {
            assert!(macos::is_implemented(name), "{name}");
        }
    }
    let mut emu = emulator(KernelOs::MacOS);
    return_slot(&mut emu);
    let ptr = unix_kernel_call(&mut emu, "IOMallocData", [ALLOCATION_SIZE, 0, 0]);
    assert!(emu.maps.is_mapped(ptr));
    for name in [
        "kalloc_type_impl_external",
        "kfree_type_impl_external",
        "kstrtoint",
        "_os_log_internal",
    ] {
        assert!(!macos::is_implemented(name));
        unix_kernel_call(&mut emu, name, [ptr, ptr, 0]);
        assert!(
            emu.kernel
                .as_ref()
                .unwrap()
                .unimplemented
                .iter()
                .any(|n| n == name)
        );
    }
    assert!(
        !emu.kernel
            .as_ref()
            .unwrap()
            .heap
            .chunk_at(ptr)
            .unwrap()
            .is_freed()
    );
    assert_eq!(emu.kernel_alloc_count(), 1);

    let aligned = unix_kernel_call(&mut emu, "IOMallocAligned", [ALLOCATION_SIZE, 16, 0]);
    assert_ne!(aligned, 0);
    assert_eq!(aligned % 16, 0);
    assert_eq!(
        unix_kernel_call(&mut emu, "IOMallocAligned", [ALLOCATION_SIZE, PAGE_SIZE, 0]),
        0
    );
    assert!(emu.maps.write_bytes(DATA, b"value=%u\0"));
    unix_kernel_call(&mut emu, "IOLog", [DATA, ALLOCATION_SIZE, 0]);
    assert_eq!(
        emu.kernel.as_ref().unwrap().log.last().map(String::as_str),
        Some("value=16")
    );
}

#[test]
fn linux_usb_dma_publishes_an_address_only_after_allocation() {
    let mut emu = emulator(KernelOs::Linux);
    return_slot(&mut emu);
    let stub = emu.kernel_resolve_import("usb_alloc_coherent").unwrap();
    for (index, inject_failure) in [false, true].into_iter().enumerate() {
        emu.kernel.as_mut().unwrap().fail_alloc_index = inject_failure.then_some(index as u64);
        assert!(emu.maps.write_qword(DATA, RESULT));
        emu.regs_mut().rsp = STACK;
        emu.regs_mut().rsi = ALLOCATION_SIZE;
        emu.regs_mut().rcx = DATA;
        assert!(emu.kernel_set_pc(stub));
        let cpu_address = emu.regs().rax;
        if inject_failure {
            assert_eq!(cpu_address, 0);
            assert_eq!(emu.maps.read_qword(DATA), Some(RESULT));
        } else {
            assert!(emu.maps.is_mapped(cpu_address));
            assert_eq!(emu.maps.read_qword(DATA), Some(cpu_address));
        }
    }
    let count = emu.kernel_alloc_count();
    emu.regs_mut().rsp = STACK;
    emu.regs_mut().rcx = 0;
    assert!(emu.kernel_set_pc(stub));
    assert!(emu.unix_api.memory_fault.is_some());
    assert_eq!(emu.kernel_alloc_count(), count);
}

#[test]
fn linux_skb_data_operations_follow_the_unknown_import_route() {
    let mut emu = emulator(KernelOs::Linux);
    return_slot(&mut emu);
    for symbol in ["skb_put", "skb_copy_bits", "skb_trim"] {
        assert!(!linux::is_implemented(symbol));
        assert!(linux::is_implemented("consume_skb"));
        let stub = emu.kernel_resolve_import(symbol).unwrap();
        emu.regs_mut().rsp = STACK;
        emu.regs_mut().rdi = DATA;
        assert!(emu.kernel_set_pc(stub));
        assert_eq!(emu.regs().rax, 0);
        assert!(
            emu.kernel
                .as_ref()
                .unwrap()
                .unimplemented
                .iter()
                .any(|name| name == symbol)
        );
    }
}
