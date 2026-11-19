use crate::api::abi::ArgumentFault;
use crate::api::unix::ApiStopReason as UnixStopReason;
use crate::api::windows::policy::{ApiDisposition, ApiStopReason as WindowsStopReason};
use crate::emu::Emu;
use crate::kernel::windows::crt::{ArgSource, format_args};
use crate::kernel::{KernelOs, linux};
use crate::maps::mem64::Permission;
use crate::maps::virtual_memory::PAGE_SIZE;
use crate::serialization::Serialization;
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
