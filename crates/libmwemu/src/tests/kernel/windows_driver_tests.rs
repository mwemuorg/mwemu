//! Windows driver loading, API routing, and IRP delivery.
//!
//! Synthetic regressions run offline. The tests using `windows_uaf_driver.sys`
//! require `make windows-driver` and skip when that fixture is absent.

use std::fs;

use goblin::pe::header::{SIZEOF_COFF_HEADER, SIZEOF_PE_MAGIC};
use goblin::pe::optional_header::{SIZEOF_STANDARD_FIELDS_64, SIZEOF_WINDOWS_FIELDS_64};
use goblin::pe::section_table::{
    IMAGE_SCN_CNT_CODE, IMAGE_SCN_MEM_EXECUTE, IMAGE_SCN_MEM_READ, IMAGE_SCN_MEM_WRITE,
};
use goblin::pe::subsystem::IMAGE_SUBSYSTEM_NATIVE;
use rs_header::pe::shared::{
    IMAGE_DIRECTORY_ENTRY_BASERELOC, IMAGE_DIRECTORY_ENTRY_IMPORT,
    IMAGE_DIRECTORY_ENTRY_LOAD_CONFIG, IMAGE_NUMBEROF_DIRECTORY_ENTRIES, IMAGE_REL_BASED_DIR64,
};

use crate::api::windows::policy::{ApiDisposition, ApiMode, ApiRoute, ApiStopReason};
use crate::emu::Emu;
use crate::emu64;
use crate::kernel::KernelOs;
use crate::kernel::guard::FindingKind;
use crate::kernel::windows::structs::{
    DEVICE_OBJECT_SIZE, DEVOBJ_STACK_SIZE, DRVOBJ_MAJOR_FUNCTION, IRP_MJ_CLOSE, IRP_MJ_CREATE,
    IRP_MJ_DEVICE_CONTROL,
};
use crate::kernel::windows::{
    self, STATUS_NOT_IMPLEMENTED, STATUS_SUCCESS, STATUS_UNSUCCESSFUL, alloc_user_buffer,
    write_unicode_header,
};
use crate::maps::mem64::Permission;
use crate::serialization::Serialization;
use crate::tests::helpers::{self, LOADER64};

const TLM_MAGIC: u32 = 0x544c4d30;
const TLM_IOC_CREATE: u32 = 0x80002004;
const TLM_IOC_WRITE: u32 = 0x80002008;
const TLM_IOC_DESTROY: u32 = 0x8000200c;
const TLM_IOC_MAPPHYS: u32 = 0x80002010;
const TLM_IOC_RDMSR: u32 = 0x80002014;
const TLM_IOC_WRMSR: u32 = 0x80002018;

/// `struct tlm_create_req { char name[24]; u32 buf_len; u32 encoding; u32 id_out; }`
fn create_req(name: &str, buf_len: u32) -> Vec<u8> {
    let mut v = vec![0u8; 36];
    v[..name.len()].copy_from_slice(name.as_bytes());
    v[24..28].copy_from_slice(&buf_len.to_le_bytes());
    v
}

/// `struct tlm_write_req { u32 id; u32 len; u8 data[]; }`
fn write_req(id: u32, data: &[u8]) -> Vec<u8> {
    let mut v = Vec::new();
    v.extend_from_slice(&id.to_le_bytes());
    v.extend_from_slice(&(data.len() as u32).to_le_bytes());
    v.extend_from_slice(data);
    v
}

/// Load the driver and run DriverEntry, or skip when the artefact is missing.
fn boot_driver() -> Option<Emu> {
    helpers::setup();
    let path = helpers::test_data_path("windows_uaf_driver.sys");
    if !std::path::Path::new(&path).exists() {
        eprintln!("[skip] windows_uaf_driver.sys not built (run `make windows-driver`)");
        return None;
    }
    let mut emu = emu64();
    emu.cfg.verbose = 1;
    emu.load_kernel_module(&path)
        .expect("the driver should link against the emulated kernel");
    let ret = emu
        .run_module_init()
        .expect("DriverEntry should run to completion");
    assert_eq!(ret, 0, "DriverEntry returned 0x{:x}", ret);
    Some(emu)
}

fn create_channel(emu: &mut Emu, name: &str, buf_len: u32) -> u32 {
    let r = emu
        .kernel_ioctl(0, TLM_IOC_CREATE, &create_req(name, buf_len), 36)
        .expect("create ioctl should be delivered");
    assert_eq!(r.status, 0, "create ioctl failed: 0x{:x}", r.status);
    assert!(r.completed, "the driver must complete the IRP");
    u32::from_le_bytes([r.output[32], r.output[33], r.output[34], r.output[35]])
}

#[test]
fn driver_loads_and_driver_entry_installs_dispatch() {
    let Some(emu) = boot_driver() else { return };

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
    let Some(mut emu) = boot_driver() else { return };
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
fn stale_hot_channel_cache_is_a_use_after_free() {
    let Some(mut emu) = boot_driver() else { return };
    let id = create_channel(&mut emu, "sensor0", 256);

    // Populate the hot-channel cache.
    emu.kernel_ioctl(0, TLM_IOC_WRITE, &write_req(id, b"telemetry-sample"), 0)
        .expect("first write");
    assert!(
        emu.kernel_findings().is_empty(),
        "first write must be clean"
    );

    // Destroy the channel: the cache is never invalidated here.
    let r = emu
        .kernel_ioctl(0, TLM_IOC_DESTROY, &id.to_le_bytes(), 0)
        .expect("destroy ioctl");
    assert_eq!(r.status, 0);

    // Write to the same id again: the hot path uses the freed object.
    let _ = emu.kernel_ioctl(0, TLM_IOC_WRITE, &write_req(id, b"telemetry-sample"), 0);

    let findings = emu.kernel_findings();
    assert!(
        emu.kernel_found_uaf(),
        "expected a use-after-free, got {:?}",
        findings.iter().map(|f| f.kind).collect::<Vec<_>>()
    );
    let uaf = findings
        .iter()
        .find(|f| f.kind.is_use_after_free())
        .expect("a use-after-free finding");
    assert_eq!(
        uaf.origin.cache, "pool-TlmC",
        "the pool tag names the object"
    );
    assert_eq!(uaf.origin.alloc_api, "ExAllocatePoolWithTag");
    assert_eq!(uaf.origin.free_api, "ExFreePoolWithTag");
    let module_base = emu.kernel.as_ref().map(|k| k.module.base).unwrap_or(0);
    assert!(
        uaf.rip >= module_base,
        "the fault must be attributed to driver code"
    );
    for f in findings {
        println!("{}", f.report());
    }
}

#[test]
fn double_free_is_reported() {
    let Some(mut emu) = boot_driver() else { return };
    let id = create_channel(&mut emu, "sensor0", 64);
    emu.kernel_ioctl(0, TLM_IOC_DESTROY, &id.to_le_bytes(), 0)
        .expect("destroy ioctl");

    let chunk = emu
        .kernel
        .as_ref()
        .expect("kernel env")
        .heap
        .chunks()
        .iter()
        .find(|c| c.cache == "pool-TlmC")
        .map(|c| c.addr)
        .expect("the channel object should be in the ledger");
    emu.kernel_free(chunk, "ExFreePoolWithTag");
    assert!(
        emu.kernel_findings()
            .iter()
            .any(|f| f.kind == FindingKind::DoubleFree),
        "expected a double-free finding"
    );
}

#[test]
fn physical_memory_and_msr_primitives_are_logged() {
    let Some(mut emu) = boot_driver() else { return };

    // struct tlm_phys_req { u32 magic; u32 size; u64 phys; u64 value_out; }
    let mut req = Vec::new();
    req.extend_from_slice(&TLM_MAGIC.to_le_bytes());
    req.extend_from_slice(&0x1000u32.to_le_bytes());
    req.extend_from_slice(&0xfee0_0000u64.to_le_bytes());
    req.extend_from_slice(&0u64.to_le_bytes());
    let r = emu
        .kernel_ioctl(0, TLM_IOC_MAPPHYS, &req, req.len())
        .expect("map ioctl");
    assert_eq!(r.status, 0, "map ioctl failed: 0x{:x}", r.status);
    let kinds: Vec<&str> = r.privops.iter().map(|p| p.kind).collect();
    assert_eq!(
        kinds,
        vec!["map_io_space", "unmap_io_space"],
        "{:?}",
        r.privops
    );
    assert_eq!(
        r.privops[0].addr, 0xfee0_0000,
        "the physical address the caller chose"
    );
    assert_eq!(r.privops[0].size, 0x1000);

    // The magic check is honoured: a wrong magic reaches nothing.
    req[0] = 0;
    let r = emu
        .kernel_ioctl(0, TLM_IOC_MAPPHYS, &req, req.len())
        .expect("map ioctl");
    assert_eq!(r.status, 0xC000000D, "STATUS_INVALID_PARAMETER expected");
    assert!(r.privops.is_empty());

    // struct tlm_msr_req { u32 magic; u32 msr; u64 value; }
    emu.kernel_set_msr(0xc000_0082, 0xffff_f800_1234_5678);
    let mut req = Vec::new();
    req.extend_from_slice(&TLM_MAGIC.to_le_bytes());
    req.extend_from_slice(&0xc000_0082u32.to_le_bytes());
    req.extend_from_slice(&0u64.to_le_bytes());
    let r = emu
        .kernel_ioctl(0, TLM_IOC_RDMSR, &req, req.len())
        .expect("rdmsr ioctl");
    assert_eq!(r.status, 0);
    assert_eq!(r.privops.len(), 1);
    assert_eq!(r.privops[0].kind, "rdmsr");
    assert_eq!(r.privops[0].addr, 0xc000_0082);
    let value = u64::from_le_bytes(r.output[8..16].try_into().expect("8 bytes"));
    assert_eq!(
        value, 0xffff_f800_1234_5678,
        "the preset MSR value is returned to the caller"
    );

    req[8..16].copy_from_slice(&0x4141_4141_4141_4141u64.to_le_bytes());
    let r = emu
        .kernel_ioctl(0, TLM_IOC_WRMSR, &req, req.len())
        .expect("wrmsr ioctl");
    assert_eq!(r.status, 0);
    assert_eq!(r.privops[0].kind, "wrmsr");
    assert_eq!(r.privops[0].value, 0x4141_4141_4141_4141);
    assert_eq!(
        emu.kernel
            .as_ref()
            .and_then(|k| k.msrs.get(&0xc000_0082).copied()),
        Some(0x4141_4141_4141_4141),
        "the write is visible to the next read"
    );
    assert!(
        emu.kernel_findings().is_empty(),
        "the primitives are not memory bugs"
    );
}

#[test]
fn ioctl_scan_finds_the_primitives() {
    let Some(mut emu) = boot_driver() else { return };

    let codes = emu.kernel_ioctl_candidates();
    for c in [
        TLM_IOC_CREATE,
        TLM_IOC_WRITE,
        TLM_IOC_DESTROY,
        TLM_IOC_MAPPHYS,
        TLM_IOC_RDMSR,
        TLM_IOC_WRMSR,
    ] {
        assert!(
            codes.contains(&c),
            "0x{:x} missing from candidates {:x?}",
            c,
            codes
        );
    }

    // With a 0x41-filled input the magic check rejects the privileged
    // IOCTLs and the size check rejects create, so the scan reports them as
    // rejected rather than reached: the scan tells *which* code has a
    // constraint, the analyst (or the fuzzer) satisfies it.
    let probes = emu.kernel_scan_ioctls(0, 0x100);
    for c in [
        TLM_IOC_MAPPHYS,
        TLM_IOC_RDMSR,
        TLM_IOC_WRMSR,
        TLM_IOC_CREATE,
    ] {
        let p = probes.iter().find(|p| p.code == c).expect("probe");
        assert_eq!(p.result.io_status, 0xC000000D, "{}", p.report());
        assert!(p.result.privops.is_empty());
        assert!(p.result.error.is_none(), "{}", p.report());
    }
    // A destroy of an id that does not exist is the driver's own verdict.
    let destroy = probes
        .iter()
        .find(|p| p.code == TLM_IOC_DESTROY)
        .expect("destroy probe");
    assert_eq!(destroy.result.io_status, 0xC0000225, "{}", destroy.report());
    // No probe ran out of its instruction budget or faulted.
    assert!(probes.iter().all(|p| p.result.error.is_none()));
    for p in &probes {
        println!("{}", p.report());
    }
}

#[test]
fn driver_unload_releases_everything() {
    let Some(mut emu) = boot_driver() else { return };
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

/// A kernel stub is only intercepted when control *branches* into it, as a
/// driver's `call [__imp_X]` does; setting PC on it directly would run its
/// `ret`. This builds `mov rax, stub; jmp rax` in an executable page: the
/// jump is intercepted and the stack (shadow space, stacked arguments,
/// return address) is exactly what `kernel_call` laid out.
fn call_through(emu: &mut Emu, api: &str) -> u64 {
    let stub = emu.kernel_stub_for(api).expect("kernel stub");
    let page = emu
        .maps
        .create_map(
            &format!("tramp.{}", api),
            0x2000_0000
                + (emu.maps.get_map_by_name("tramp.KeBugCheckEx").is_some() as u64) * 0x1000,
            0x1000,
            Permission::READ_WRITE_EXECUTE,
        )
        .expect("trampoline page")
        .get_base();
    let mut code = vec![0x48, 0xb8];
    code.extend_from_slice(&stub.to_le_bytes());
    code.extend_from_slice(&[0xff, 0xe0]);
    emu.maps.write_bytes(page, &code);
    page
}

/// A bugcheck, a fast-fail or a raised status ends the call with an error,
/// not with whatever was in rax, and the driver can be driven again
/// afterwards: a fuzzer must not mistake a crash for a successful request.
#[test]
fn bugcheck_is_an_error_and_the_driver_stays_usable() {
    let Some(mut emu) = boot_driver() else { return };
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
    let Some(mut emu) = boot_driver() else { return };
    let us = crate::kernel::windows::alloc_user_buffer(&mut emu, 0x100, "name");
    let buf = us + 0x20;
    emu.maps.write_wide_string(buf, "ExAllocatePool2");
    crate::kernel::windows::write_unicode_header(&mut emu, us, buf, 15 * 2, 0x40);
    let lookup = call_through(&mut emu, "MmGetSystemRoutineAddress");
    let known = emu.kernel_call(lookup, &[us]).expect("lookup");
    assert_ne!(known, 0, "a modelled export resolves");

    emu.maps.write_wide_string(buf, "ZwNoSuchRoutine");
    crate::kernel::windows::write_unicode_header(&mut emu, us, buf, 15 * 2, 0x40);
    let unknown = emu.kernel_call(lookup, &[us]).expect("lookup");
    assert_eq!(unknown, 0, "an unknown name is NULL");
}

const POLICY_CODE: u64 = 0x400000;

fn policy_driver(symbol: &str) -> Emu {
    let mut emu = emu64();
    emu.cfg.console_enabled = false;
    emu.running_script = true;
    emu.kernel_init(KernelOs::Windows);
    let stub = emu.kernel_resolve_import(symbol).expect("kernel import");
    emu.maps
        .create_map(
            "driver.code",
            POLICY_CODE,
            0x1000,
            Permission::READ_WRITE_EXECUTE,
        )
        .unwrap();
    // sub rsp, 0x28; mov rax, stub; call rax; add rsp, 0x28; ret.
    let mut code = vec![0x48, 0x83, 0xec, 0x28, 0x48, 0xb8];
    code.extend_from_slice(&stub.to_le_bytes());
    code.extend_from_slice(&[0xff, 0xd0, 0x48, 0x83, 0xc4, 0x28, 0xc3]);
    assert!(emu.maps.write_bytes(POLICY_CODE, &code));
    emu
}

#[test]
fn missing_windows_kernel_import_obeys_policy_through_host_call() {
    for mode in [
        ApiMode::Strict,
        ApiMode::SimulateFail,
        ApiMode::SimulateSuccess,
    ] {
        let mut emu = policy_driver("MissingKernelService");
        emu.set_api_mode(mode);
        let result = emu.kernel_call(POLICY_CODE, &[]);
        assert_eq!(
            result.is_err(),
            mode == ApiMode::Strict,
            "{mode:?}: {result:?}"
        );
        assert_eq!(emu.api_policy.stopped(), mode == ApiMode::Strict);
        assert_eq!(
            emu.kernel.as_ref().unwrap().unimplemented,
            ["MissingKernelService"]
        );
        if mode != ApiMode::Strict {
            assert_eq!(result.unwrap(), 0);
        }
    }
}

#[test]
fn kernel_import_models_preserve_results_and_unknown_nt_names() {
    for name in ["NtQueryInformationAtom", "ZwQueryInformationAtom"] {
        for (mode, expected) in [
            (ApiMode::SimulateFail, STATUS_NOT_IMPLEMENTED),
            (ApiMode::SimulateSuccess, STATUS_SUCCESS),
        ] {
            let mut emu = policy_driver(name);
            emu.set_api_mode(mode);
            emu.set_last_error(0x1234);
            let stack = emu.sp();
            assert_eq!(emu.kernel_call(POLICY_CODE, &[]).unwrap(), expected);
            assert_eq!(emu.sp(), stack);
            assert_eq!(emu.get_last_error(), 0x1234);
            let outcome = emu.api_policy.last_outcome.as_ref().unwrap();
            assert_eq!(outcome.call.route, ApiRoute::Kernel);
            assert_eq!(outcome.call.name, name);
            assert!(emu.api_policy.entry.is_none());
        }
    }
    let mut emu = policy_driver("PsGetCurrentProcessId");
    assert_eq!(emu.kernel_call(POLICY_CODE, &[]).unwrap(), 4);
    assert!(emu.api_policy.last_outcome.is_none());
    assert!(emu.kernel.as_ref().unwrap().unimplemented.is_empty());
}

#[test]
fn kernel_frame_fault_retains_entry_and_pending_stop() {
    let mut emu = policy_driver("MissingKernelService");
    emu.set_api_mode(ApiMode::SimulateSuccess);
    let stub = emu.kernel_resolve_import("MissingKernelService").unwrap();
    let stack = 0x600000;
    emu.set_sp(stack);
    emu.set_pc(POLICY_CODE);
    emu.regs_mut().rax = 0x1234;
    assert!(emu.kernel_set_pc(stub));
    assert_eq!(emu.pc(), POLICY_CODE);
    assert_eq!(emu.sp(), stack);
    assert_eq!(emu.regs().rax, 0x1234);
    let outcome = emu.api_policy.last_outcome.clone().unwrap();
    assert_eq!(outcome.call.route, ApiRoute::Kernel);
    assert_eq!(outcome.call.address, stub);
    assert_eq!(
        outcome.disposition,
        ApiDisposition::Stopped(ApiStopReason::InvalidFrame)
    );
    assert!(emu.kernel_call(POLICY_CODE, &[]).is_err());
    assert_eq!(emu.pc(), POLICY_CODE);
    assert_eq!(emu.sp(), stack);
    assert_eq!(emu.api_policy.last_outcome.as_ref(), Some(&outcome));
    let restored = Serialization::deserialize(&Serialization::serialize(&emu));
    assert_eq!(restored.api_policy.last_outcome.as_ref(), Some(&outcome));
}

#[test]
fn kernel_policy_stop_reaches_driver_entry_and_irp_result() {
    let mut emu = policy_driver("MissingKernelService");
    {
        let kernel = emu.kernel.as_mut().unwrap();
        kernel.module.base = POLICY_CODE;
        kernel.module.size = 0x1000;
        kernel.module.init = Some(POLICY_CODE);
        kernel.module.name = "policy_driver".into();
    }
    assert!(emu.run_driver_entry().is_err());
    assert!(emu.api_policy.stopped());
    let driver = emu.kernel_driver_object();
    assert_ne!(driver, 0);
    assert!(emu.maps.write_qword(
        driver + DRVOBJ_MAJOR_FUNCTION + IRP_MJ_DEVICE_CONTROL as u64 * size_of::<u64>() as u64,
        POLICY_CODE
    ));
    let device = emu
        .kernel_data_for("policy.device", DEVICE_OBJECT_SIZE)
        .unwrap();
    assert!(emu.maps.write_byte(device + DEVOBJ_STACK_SIZE, 1));
    emu.api_policy.clear_outcome();
    let result = emu.kernel_ioctl(device, 0, &[], 0).unwrap();
    assert!(!result.succeeded());
    assert!(result.error.as_deref().unwrap().contains("StrictPolicy"));
    assert!(!result.completed);
    assert_eq!(result.status, STATUS_UNSUCCESSFUL as u32);
    assert_eq!(
        emu.api_policy.last_outcome.as_ref().unwrap().call.name,
        "MissingKernelService"
    );
}

#[test]
fn other_kernel_platforms_keep_their_generic_fallback() {
    for os in [KernelOs::Linux, KernelOs::MacOS] {
        let mut emu = emu64();
        emu.kernel_init(os);
        let stub = emu.kernel_resolve_import("missing_kernel_service").unwrap();
        let stack = emu.sp();
        assert!(emu.maps.write_qword(stack, POLICY_CODE));
        emu.regs_mut().rax = 0x1234;
        assert!(emu.kernel_set_pc(stub));
        assert_eq!(emu.regs().rax, 0);
        assert_eq!(emu.pc(), POLICY_CODE);
        assert_eq!(emu.sp(), stack + size_of::<u64>() as u64);
        assert!(emu.api_policy.last_outcome.is_none());
    }
}

#[test]
fn kernel_policy_bookkeeping_preserves_callback_transfers() {
    let mut emu = policy_driver("KeIpiGenericCall");
    let callback = POLICY_CODE + 0x100;
    // mov rax, rcx; ret.
    assert!(emu.maps.write_bytes(callback, &[0x48, 0x89, 0xc8, 0xc3]));
    let context = 0x12345678;
    assert_eq!(
        emu.kernel_call(POLICY_CODE, &[callback, context]).unwrap(),
        context
    );
    assert!(emu.api_policy.last_outcome.is_none());
    assert!(emu.api_policy.entry.is_none());
}

const TEST_IMAGE_BASE: u64 = 0x140000000;
const TEST_SECTION_RVA: usize = 0x1000;
const TEST_SECTION_RAW: usize = 0x400;
const TEST_COOKIE_RVA: usize = 0x1200;
const TEST_LOAD_CONFIG_RVA: usize = 0x1300;
const LOAD_CONFIG_COOKIE_OFFSET: usize = 0x58;
const DEFAULT_COOKIE64: u64 = 0x00002b992ddfa232;

fn driver_fixture(imports: &[&str]) -> Vec<u8> {
    const IMPORT_RVA: usize = 0x1400;
    const LOOKUP_RVA: usize = 0x1480;
    const IAT_RVA: usize = 0x14c0;
    const DLL_NAME_RVA: usize = 0x1500;
    const IMPORT_NAMES_RVA: usize = 0x1520;
    const RELOC_RVA: usize = 0x1800;
    let mut raw = vec![0; TEST_SECTION_RAW + 0x1000];
    raw[..TEST_SECTION_RAW].copy_from_slice(&LOADER64[..TEST_SECTION_RAW]);
    let nt = u32::from_le_bytes(raw[0x3c..0x40].try_into().unwrap()) as usize;
    let opt = nt + SIZEOF_PE_MAGIC + SIZEOF_COFF_HEADER;
    let optional_size = u16::from_le_bytes(raw[nt + 20..nt + 22].try_into().unwrap()) as usize;
    raw[nt + 6..nt + 8].copy_from_slice(&1u16.to_le_bytes());
    raw[opt + 16..opt + 20].copy_from_slice(&(TEST_SECTION_RVA as u32).to_le_bytes());
    raw[opt + 24..opt + 32].copy_from_slice(&TEST_IMAGE_BASE.to_le_bytes());
    raw[opt + 32..opt + 36].copy_from_slice(&0x1000u32.to_le_bytes());
    raw[opt + 36..opt + 40].copy_from_slice(&0x200u32.to_le_bytes());
    raw[opt + 56..opt + 60].copy_from_slice(&0x2000u32.to_le_bytes());
    raw[opt + 60..opt + 64].copy_from_slice(&(TEST_SECTION_RAW as u32).to_le_bytes());
    raw[opt + 68..opt + 70].copy_from_slice(&IMAGE_SUBSYSTEM_NATIVE.to_le_bytes());
    let directories = opt + SIZEOF_STANDARD_FIELDS_64 + SIZEOF_WINDOWS_FIELDS_64;
    raw[directories..directories + IMAGE_NUMBEROF_DIRECTORY_ENTRIES * 8].fill(0);
    let section = opt + optional_size;
    raw[section..TEST_SECTION_RAW].fill(0);
    raw[section..section + 8].copy_from_slice(b".driver\0");
    for (offset, value) in [
        (8, 0x1000),
        (12, TEST_SECTION_RVA as u32),
        (16, 0x1000),
        (20, TEST_SECTION_RAW as u32),
        (
            36,
            IMAGE_SCN_CNT_CODE | IMAGE_SCN_MEM_READ | IMAGE_SCN_MEM_WRITE | IMAGE_SCN_MEM_EXECUTE,
        ),
    ] {
        raw[section + offset..section + offset + 4].copy_from_slice(&value.to_le_bytes());
    }
    let file_offset = |rva: usize| TEST_SECTION_RAW + rva - TEST_SECTION_RVA;
    // mov rax, [rip + cookie]; shr rax, 48; ret. A valid x64 cookie returns zero.
    let mut code = vec![0x48, 0x8b, 0x05];
    code.extend_from_slice(&((TEST_COOKIE_RVA - TEST_SECTION_RVA - 7) as u32).to_le_bytes());
    code.extend_from_slice(&[0x48, 0xc1, 0xe8, 48, 0xc3]);
    raw[TEST_SECTION_RAW..TEST_SECTION_RAW + code.len()].copy_from_slice(&code);
    let cookie = file_offset(TEST_COOKIE_RVA);
    raw[cookie..cookie + 8].copy_from_slice(&DEFAULT_COOKIE64.to_le_bytes());
    let config = file_offset(TEST_LOAD_CONFIG_RVA);
    let config_size = LOAD_CONFIG_COOKIE_OFFSET + size_of::<u64>();
    raw[config..config + 4].copy_from_slice(&(config_size as u32).to_le_bytes());
    let cookie_pointer = config + LOAD_CONFIG_COOKIE_OFFSET;
    raw[cookie_pointer..cookie_pointer + 8]
        .copy_from_slice(&(TEST_IMAGE_BASE + TEST_COOKIE_RVA as u64).to_le_bytes());
    let relocation = file_offset(RELOC_RVA);
    raw[relocation..relocation + 4].copy_from_slice(&(TEST_SECTION_RVA as u32).to_le_bytes());
    raw[relocation + 4..relocation + 8].copy_from_slice(&12u32.to_le_bytes());
    let entry = (IMAGE_REL_BASED_DIR64 << 12)
        | ((TEST_LOAD_CONFIG_RVA + LOAD_CONFIG_COOKIE_OFFSET - TEST_SECTION_RVA) as u16);
    raw[relocation + 8..relocation + 10].copy_from_slice(&entry.to_le_bytes());
    for (index, rva, size) in [
        (
            IMAGE_DIRECTORY_ENTRY_LOAD_CONFIG,
            TEST_LOAD_CONFIG_RVA,
            config_size,
        ),
        (IMAGE_DIRECTORY_ENTRY_BASERELOC, RELOC_RVA, 12),
        (IMAGE_DIRECTORY_ENTRY_IMPORT, IMPORT_RVA, 40),
    ] {
        let offset = directories + index * 8;
        raw[offset..offset + 4].copy_from_slice(&(rva as u32).to_le_bytes());
        raw[offset + 4..offset + 8].copy_from_slice(&(size as u32).to_le_bytes());
    }
    if !imports.is_empty() {
        let descriptor = file_offset(IMPORT_RVA);
        for (offset, rva) in [(0, LOOKUP_RVA), (12, DLL_NAME_RVA), (16, IAT_RVA)] {
            raw[descriptor + offset..descriptor + offset + 4]
                .copy_from_slice(&(rva as u32).to_le_bytes());
        }
        let dll = b"ntoskrnl.exe\0";
        let dll_offset = file_offset(DLL_NAME_RVA);
        raw[dll_offset..dll_offset + dll.len()].copy_from_slice(dll);
        let mut name_rva = IMPORT_NAMES_RVA;
        for (index, name) in imports.iter().enumerate() {
            let name_offset = file_offset(name_rva) + 2;
            raw[name_offset..name_offset + name.len()].copy_from_slice(name.as_bytes());
            for table in [LOOKUP_RVA, IAT_RVA] {
                let slot = file_offset(table) + index * 8;
                raw[slot..slot + 8].copy_from_slice(&(name_rva as u64).to_le_bytes());
            }
            name_rva += name.len() + 3;
        }
    }
    raw
}

fn load_driver_fixture(imports: &[&str]) -> Emu {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("kernel-regression.sys");
    fs::write(&path, driver_fixture(imports)).unwrap();
    let mut emu = emu64();
    emu.cfg.console_enabled = false;
    emu.running_script = true;
    emu.load_sys_driver(path.to_str().unwrap()).unwrap();
    emu
}

#[test]
fn driver_loader_seeds_cookie_accepted_by_x64_check() {
    let mut emu = load_driver_fixture(&[]);
    let base = emu.kernel.as_ref().unwrap().module.base;
    assert_ne!(
        base, TEST_IMAGE_BASE,
        "exercise the relocated Load Config pointer"
    );
    let cookie = emu.read_security_cookie().unwrap();
    assert_ne!(cookie, 0);
    assert_ne!(cookie, DEFAULT_COOKIE64);
    assert_eq!(emu.run_driver_entry().unwrap(), STATUS_SUCCESS);
    assert_eq!(cookie >> 48, 0, "x64 cookie high word must be zero");
}

#[test]
fn driver_loader_recognizes_existing_import_handlers() {
    let emu = load_driver_fixture(&[
        "IoSetStartIoAttributes",
        "IoGetRequestorSessionId",
        "MissingKernelService",
    ]);
    assert_eq!(
        emu.kernel.as_ref().unwrap().module.unresolved,
        ["MissingKernelService"]
    );
}

#[test]
fn kernel_lookup_resolves_dispatch_aliases_and_rejects_unknown_names() {
    let mut emu = policy_driver("MmGetSystemRoutineAddress");
    let us = alloc_user_buffer(&mut emu, 0x200, "lookup");
    let buffer = us + 0x20;
    for name in [
        "InterlockedPushEntrySList",
        "IoSetStartIoAttributes",
        "IoGetRequestorSessionId",
        "MmLockPagableImageSection",
        "RtlCopyMemoryNonTemporal",
        "KeInitializeDeviceQueue",
        "DbgPrintReturnControlC",
        "PsGetCurrentThreadProcess",
    ] {
        emu.maps.write_wide_string(buffer, name);
        write_unicode_header(&mut emu, us, buffer, (name.len() * 2) as u64, 0x100);
        let expected = emu.kernel_resolve_import(name).unwrap();
        assert_eq!(
            emu.kernel_call(POLICY_CODE, &[us]).unwrap(),
            expected,
            "{name}"
        );
        assert!(windows::is_implemented(name), "{name}");
    }
    for name in [
        "IoCsqRemoveNextIrp",
        "MissingKernelService",
        "iogetrequestorsessionid",
    ] {
        emu.maps.write_wide_string(buffer, name);
        write_unicode_header(&mut emu, us, buffer, (name.len() * 2) as u64, 0x100);
        assert_eq!(emu.kernel_call(POLICY_CODE, &[us]).unwrap(), 0, "{name}");
    }
    assert_eq!(emu.api_policy.simulation_count, 0);
}
