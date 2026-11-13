//! End-to-end Windows kernel-mode emulation: load a `.sys`, run DriverEntry,
//! send IOCTLs, and check that both the deliberate use-after-free and the
//! BYOVD primitives in `drivers/windows/tlm` are reported.
//!
//! The driver is built by `make windows-driver` (mingw-w64); when the
//! artefact is not present these tests skip, like every other
//! sample-dependent test here.

use crate::emu::Emu;
use crate::emu64;
use crate::kernel::guard::FindingKind;
use crate::kernel::windows::structs::{IRP_MJ_CLOSE, IRP_MJ_CREATE, IRP_MJ_DEVICE_CONTROL};
use crate::tests::helpers;

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
