use super::{TLM_IOC_WRITE, boot_driver, call_through, create_channel, write_req};
use crate::kernel::guard::FindingKind;
use crate::kernel::windows::structs::{IRP_MJ_CLOSE, IRP_MJ_CREATE, IRP_MJ_DEVICE_CONTROL};
use crate::kernel::windows::{alloc_user_buffer, write_unicode_header};

#[test]
fn driver_loads_and_driver_entry_installs_dispatch() {
    let emu = boot_driver();

    let kernel = emu.kernel.as_ref().expect("kernel env");
    assert_eq!(kernel.module.name, "windows_uaf_driver");
    assert!(
        kernel.module.unresolved.is_empty(),
        "unresolved kernel imports: {:?}",
        kernel.module.unresolved
    );
    assert!(kernel.module.exit.is_some(), "DriverUnload not captured");

    let devices = emu.kernel_devices();
    assert_eq!(devices.len(), 1, "one device expected, got {:?}", devices);
    assert_eq!(devices[0].name, "\\Device\\TlmWin");
    assert_eq!(
        emu.kernel_symlinks(),
        vec![(
            "\\DosDevices\\TlmWin".to_string(),
            "\\Device\\TlmWin".to_string()
        )]
    );

    let table = emu.kernel_dispatch_table();
    let majors: Vec<u8> = table.iter().map(|(m, _, _)| *m).collect();
    assert_eq!(
        majors,
        vec![IRP_MJ_CREATE, IRP_MJ_CLOSE, IRP_MJ_DEVICE_CONTROL]
    );
    assert!(
        emu.module_symbol("IRP_MJ_DEVICE_CONTROL").is_some(),
        "the dispatch routine should be reachable by name"
    );

    // The data import NtBuildNumber resolved to storage the driver could read.
    assert!(
        kernel
            .log
            .iter()
            .any(|l| l.contains("telemetry driver loaded, build 19045")),
        "expected the driver's own log line, got {:?}",
        kernel.log
    );
    assert!(
        emu.kernel_findings().is_empty(),
        "a clean load must not report anything"
    );
    assert!(
        emu.kernel_privops().is_empty(),
        "DriverEntry performs no privileged operation"
    );
}

#[test]
fn create_and_write_channel_is_clean() {
    let mut emu = boot_driver();
    emu.kernel_create(0).expect("IRP_MJ_CREATE");

    let id = create_channel(&mut emu, "sensor0", 256);
    assert_eq!(id, 1);
    let r = emu
        .kernel_ioctl(0, TLM_IOC_WRITE, &write_req(id, b"telemetry-sample"), 0)
        .expect("write ioctl");
    assert_eq!(r.status, 0);
    assert_eq!(r.information, 16, "the write should report the byte count");

    // An unknown control code takes the driver's default branch.
    let r = emu
        .kernel_ioctl(0, 0x80002ffc, &[], 0)
        .expect("unknown ioctl");
    assert_eq!(
        r.status, 0xC0000010,
        "STATUS_INVALID_DEVICE_REQUEST expected"
    );

    assert!(
        emu.kernel_findings().is_empty(),
        "legitimate use must stay silent: {:?}",
        emu.kernel_findings()
    );
}

#[test]
fn driver_unload_releases_everything() {
    let mut emu = boot_driver();
    create_channel(&mut emu, "a", 64);
    create_channel(&mut emu, "b", 64);
    emu.run_module_exit().expect("DriverUnload");
    assert!(
        emu.kernel_devices().is_empty(),
        "IoDeleteDevice should have removed the device"
    );
    emu.kernel_check_leaks();
    let leaks: Vec<_> = emu
        .kernel_findings()
        .iter()
        .filter(|f| f.kind == FindingKind::Leak)
        .collect();
    assert!(
        leaks.is_empty(),
        "unload should free every channel: {:?}",
        leaks
    );
}

/// A bugcheck, a fast-fail or a raised status ends the call with an error,
/// not with whatever was in rax, and the driver can be driven again
/// afterwards: a fuzzer must not mistake a crash for a successful request.
#[test]
fn bugcheck_is_an_error_and_the_driver_stays_usable() {
    let mut emu = boot_driver();
    let bugcheck = call_through(&mut emu, "KeBugCheckEx");
    let err = emu
        .kernel_call(bugcheck, &[0x7e, 1, 2, 3, 4])
        .expect_err("a bugcheck must not look like a return value");
    assert!(err.message.contains("KeBugCheckEx"), "{}", err.message);
    assert!(
        emu.kernel
            .as_ref()
            .expect("kernel env")
            .log
            .iter()
            .any(|l| l.starts_with("ABORT:")),
        "the abort is in the kernel log"
    );

    let id = create_channel(&mut emu, "after-crash", 64);
    assert_eq!(id, 1, "requests keep working after an aborted call");
}

/// `MmGetSystemRoutineAddress` returns NULL for a name the kernel does not
/// export, so a driver probing for an optional API takes its fallback path.
#[test]
fn system_routine_lookup_is_null_for_unknown_names() {
    let mut emu = boot_driver();
    let us = alloc_user_buffer(&mut emu, 0x100, "name");
    let buf = us + 0x20;
    emu.maps.write_wide_string(buf, "ExAllocatePool2");
    write_unicode_header(&mut emu, us, buf, 15 * 2, 0x40);
    let lookup = call_through(&mut emu, "MmGetSystemRoutineAddress");
    let known = emu.kernel_call(lookup, &[us]).expect("lookup");
    assert_ne!(known, 0, "a modelled export resolves");

    emu.maps.write_wide_string(buf, "ZwNoSuchRoutine");
    write_unicode_header(&mut emu, us, buf, 15 * 2, 0x40);
    let unknown = emu.kernel_call(lookup, &[us]).expect("lookup");
    assert_eq!(unknown, 0, "an unknown name is NULL");
}
