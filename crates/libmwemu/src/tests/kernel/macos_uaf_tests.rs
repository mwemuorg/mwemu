//! End-to-end kext emulation: load a Mach-O `MH_KEXT_BUNDLE`, run its kmod
//! `start`, drive its ioctl surface, and check that the deliberate
//! use-after-free in `drivers/macos/tlm` is reported — the macOS counterpart
//! of `linux_uaf_tests`.
//!
//! The kext is built from source by `make macos-driver`; when the artefact is
//! absent these tests skip, like every other sample-dependent test here.

use crate::emu::Emu;
use crate::emu_aarch64;
use crate::maps::mem64::Permission;
use crate::tests::helpers;

const TLM_IOC_CREATE: u64 = 0x1001;
const TLM_IOC_WRITE: u64 = 0x1002;
const TLM_IOC_DESTROY: u64 = 0x1003;

/// Scratch page standing in for the caller's request/payload buffer.
fn scratch_page(emu: &mut Emu) -> u64 {
    emu.maps
        .create_map("tlm.user", 0x1_0000_0000, 0x1000, Permission::READ_WRITE)
        .expect("cannot create scratch page")
        .get_base()
}

/// `struct tlm_create_req { char name[24]; u32 buf_len; u32 encoding; u32 id_out; }`
fn write_create_req(emu: &mut Emu, at: u64, name: &str, buf_len: u32, encoding: u32) {
    emu.maps.write_bytes(at, &[0u8; 36]);
    emu.maps.write_string(at, name);
    emu.maps.write_dword(at + 24, buf_len);
    emu.maps.write_dword(at + 28, encoding);
    emu.maps.write_dword(at + 32, 0);
}

/// `struct tlm_write_req { u32 id; u32 len; u64 data; }`
fn write_write_req(emu: &mut Emu, at: u64, id: u32, len: u32, data: u64) {
    emu.maps.write_dword(at, id);
    emu.maps.write_dword(at + 4, len);
    emu.maps.write_qword(at + 8, data);
}

/// Load the kext and run its `start`, or skip when the artefact is missing.
fn boot_kext() -> Option<Emu> {
    helpers::setup();
    let path = helpers::test_data_path("macos_uaf_driver.kext");
    if !std::path::Path::new(&path).exists() {
        eprintln!("[skip] macos_uaf_driver.kext not built (run `make macos-driver`)");
        return None;
    }

    let mut emu = emu_aarch64();
    emu.cfg.verbose = 1;
    emu.load_kext_macho64(&path)
        .expect("the kext should link against the emulated kernel");
    let ret = emu.run_module_init().expect("kext start should run");
    assert_eq!(ret, 0, "kext start returned an error");
    Some(emu)
}

#[test]
fn kext_links_and_starts() {
    let Some(emu) = boot_kext() else { return };

    let kernel = emu.kernel.as_ref().expect("kernel env");
    assert_eq!(kernel.module.name, "com.mwemu.tlm");
    assert!(kernel.module.init.is_some(), "kmod start not found");
    assert!(kernel.module.exit.is_some(), "kmod stop not found");
    assert!(
        emu.module_symbol("_tlm_ioctl").is_some(),
        "the ioctl handler should be reachable by name"
    );
    assert!(
        kernel
            .log
            .iter()
            .any(|l| l.contains("telemetry driver loaded")),
        "expected the driver's own log line, got {:?}",
        kernel.log
    );
    assert!(
        emu.kernel_findings().is_empty(),
        "a clean load must not report anything: {:?}",
        emu.kernel_findings()
    );
}

#[test]
fn create_and_write_channel_is_clean() {
    let Some(mut emu) = boot_kext() else { return };
    let page = scratch_page(&mut emu);

    write_create_req(&mut emu, page, "sensor0", 256, 0);
    let ret = emu
        .call_module_symbol("_tlm_ioctl", &[0, TLM_IOC_CREATE, page])
        .expect("create ioctl should run");
    assert_eq!(ret, 0, "create ioctl failed");
    let id = emu.maps.read_dword(page + 32).expect("id_out") as u64;
    assert_eq!(id, 1);

    let payload = page + 0x100;
    emu.maps.write_bytes(payload, b"telemetry-sample");
    write_write_req(&mut emu, page, id as u32, 16, payload);
    let ret = emu
        .call_module_symbol("_tlm_ioctl", &[0, TLM_IOC_WRITE, page])
        .expect("write ioctl should run");
    assert_eq!(ret, 16, "write ioctl should report the byte count");

    assert!(
        emu.kernel_findings().is_empty(),
        "legitimate use must stay silent: {:?}",
        emu.kernel_findings()
    );
}

#[test]
fn stale_hot_channel_cache_is_a_use_after_free() {
    let Some(mut emu) = boot_kext() else { return };
    let page = scratch_page(&mut emu);

    // 1. create a channel
    write_create_req(&mut emu, page, "sensor0", 256, 0);
    assert_eq!(
        emu.call_module_symbol("_tlm_ioctl", &[0, TLM_IOC_CREATE, page])
            .expect("create ioctl"),
        0
    );
    let id = emu.maps.read_dword(page + 32).expect("id_out") as u64;

    // 2. write once, populating the driver's hot-channel cache
    let payload = page + 0x100;
    emu.maps.write_bytes(payload, b"telemetry-sample");
    write_write_req(&mut emu, page, id as u32, 16, payload);
    emu.call_module_symbol("_tlm_ioctl", &[0, TLM_IOC_WRITE, page])
        .expect("first write ioctl");
    assert!(
        emu.kernel_findings().is_empty(),
        "first write must be clean"
    );

    // 3. destroy the channel — the cache is never invalidated here
    emu.maps.write_dword(page, id as u32);
    assert_eq!(
        emu.call_module_symbol("_tlm_ioctl", &[0, TLM_IOC_DESTROY, page])
            .expect("destroy ioctl"),
        0
    );

    // 4. write to the same id again: the hot path runs against the freed object
    write_write_req(&mut emu, page, id as u32, 16, payload);
    let _ = emu.call_module_symbol("_tlm_ioctl", &[0, TLM_IOC_WRITE, page]);

    let findings = emu.kernel_findings();
    assert!(
        !findings.is_empty(),
        "the stale cache write should have been reported"
    );
    assert!(
        emu.kernel_found_uaf(),
        "expected a use-after-free, got {:?}",
        findings.iter().map(|f| f.kind).collect::<Vec<_>>()
    );

    let uaf = findings
        .iter()
        .find(|f| f.kind.is_use_after_free())
        .expect("a use-after-free finding");
    assert_eq!(uaf.origin.cache, "kalloc");
    assert!(uaf.origin.alloc_api.starts_with("IOMalloc"));
    assert_eq!(uaf.origin.free_api, "IOFree");
    assert!(uaf.rip >= emu.kernel.as_ref().unwrap().module.base);

    for f in findings {
        println!("{}", f.report());
    }
}
