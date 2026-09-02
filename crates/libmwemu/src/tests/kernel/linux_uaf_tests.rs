//! End-to-end kernel-mode emulation: load a `.ko`, drive its ioctl surface,
//! and check that the deliberate use-after-free in `drivers/linux/tlm` is
//! reported.
//!
//! The driver is built from source by `make driver`; when the artefact is not
//! present these tests skip, like every other sample-dependent test here.

use crate::emu::Emu;
use crate::emu64;
use crate::kernel::guard::FindingKind;
use crate::maps::mem64::Permission;
use crate::tests::helpers;

const TLM_IOC_CREATE: u64 = 0x1001;
const TLM_IOC_WRITE: u64 = 0x1002;
const TLM_IOC_DESTROY: u64 = 0x1003;

/// Scratch page standing in for the caller's user-space buffer.
fn user_page(emu: &mut Emu) -> u64 {
    emu.maps
        .create_map("tlm.user", 0x1000_0000, 0x1000, Permission::READ_WRITE)
        .expect("cannot create user scratch page")
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

/// Load the driver and run its init, or skip when the artefact is missing.
fn boot_driver() -> Option<Emu> {
    helpers::setup();
    let path = helpers::test_data_path("linux_uaf_driver.ko");
    if !std::path::Path::new(&path).exists() {
        eprintln!("[skip] linux_uaf_driver.ko not built (run `make driver`)");
        return None;
    }

    let mut emu = emu64();
    emu.cfg.verbose = 1;
    emu.load_kernel_module(&path)
        .expect("the driver should link against the emulated kernel");
    let ret = emu
        .run_module_init()
        .expect("module init should run to completion");
    assert_eq!(ret, 0, "module init returned an error");
    Some(emu)
}

#[test]
fn kernel_module_links_and_initializes() {
    let Some(emu) = boot_driver() else { return };

    let kernel = emu.kernel.as_ref().expect("kernel env");
    assert_eq!(kernel.module.name, "tlm");
    assert!(kernel.module.init.is_some(), "init_module not found");
    assert!(kernel.module.exit.is_some(), "cleanup_module not found");
    assert!(
        kernel.module.unresolved.is_empty(),
        "unresolved kernel imports: {:?}",
        kernel.module.unresolved
    );
    assert!(
        emu.module_symbol("tlm_ioctl").is_some(),
        "the ioctl handler should be reachable by name"
    );

    // init registers a misc device and creates its slab cache.
    assert!(
        kernel
            .log
            .iter()
            .any(|l| l.contains("telemetry driver loaded")),
        "expected the driver's own log line, got {:?}",
        kernel.log
    );
    assert!(
        kernel.caches.values().any(|c| c.name == "tlm_channel"),
        "the driver's kmem_cache should be registered"
    );
    assert!(
        emu.kernel_findings().is_empty(),
        "a clean load must not report anything: {:?}",
        emu.kernel_findings()
    );
}

#[test]
fn create_and_write_channel_is_clean() {
    let Some(mut emu) = boot_driver() else { return };
    let page = user_page(&mut emu);

    write_create_req(&mut emu, page, "sensor0", 256, 0);
    let ret = emu
        .call_module_symbol("tlm_ioctl", &[0, TLM_IOC_CREATE, page])
        .expect("create ioctl should run");
    assert_eq!(ret, 0, "create ioctl failed");
    let id = emu.maps.read_dword(page + 32).expect("id_out") as u64;
    assert_eq!(id, 1);

    // Payload for the write lives in the same scratch page.
    let payload = page + 0x100;
    emu.maps.write_bytes(payload, b"telemetry-sample");
    write_write_req(&mut emu, page, id as u32, 16, payload);
    let ret = emu
        .call_module_symbol("tlm_ioctl", &[0, TLM_IOC_WRITE, page])
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
    let Some(mut emu) = boot_driver() else { return };
    let page = user_page(&mut emu);

    // 1. create a channel
    write_create_req(&mut emu, page, "sensor0", 256, 0);
    assert_eq!(
        emu.call_module_symbol("tlm_ioctl", &[0, TLM_IOC_CREATE, page])
            .expect("create ioctl"),
        0
    );
    let id = emu.maps.read_dword(page + 32).expect("id_out");

    // 2. write to it once, which populates the driver's hot-channel cache
    let payload = page + 0x100;
    emu.maps.write_bytes(payload, b"telemetry-sample");
    write_write_req(&mut emu, page, id, 16, payload);
    emu.call_module_symbol("tlm_ioctl", &[0, TLM_IOC_WRITE, page])
        .expect("first write ioctl");
    assert!(
        emu.kernel_findings().is_empty(),
        "first write must be clean"
    );

    // 3. destroy the channel — the cache is never invalidated here
    emu.maps.write_dword(page, id);
    assert_eq!(
        emu.call_module_symbol("tlm_ioctl", &[0, TLM_IOC_DESTROY, page])
            .expect("destroy ioctl"),
        0
    );

    // 4. write to the same id again: the hot path uses the freed object
    write_write_req(&mut emu, page, id, 16, payload);
    let _ = emu.call_module_symbol("tlm_ioctl", &[0, TLM_IOC_WRITE, page]);

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

    // The report must name the object the driver actually allocated, and both
    // the allocation and the free site, or it is not actionable.
    let uaf = findings
        .iter()
        .find(|f| f.kind.is_use_after_free())
        .expect("a use-after-free finding");
    assert_eq!(uaf.origin.cache, "tlm_channel");
    assert!(uaf.origin.alloc_api.starts_with("kmem_cache_alloc"));
    assert_eq!(uaf.origin.free_api, "kmem_cache_free");
    assert!(uaf.rip >= emu.kernel.as_ref().unwrap().module.base);

    for f in findings {
        println!("{}", f.report());
    }
}

#[test]
fn double_free_is_reported() {
    let Some(mut emu) = boot_driver() else { return };
    let page = user_page(&mut emu);

    write_create_req(&mut emu, page, "sensor0", 64, 0);
    emu.call_module_symbol("tlm_ioctl", &[0, TLM_IOC_CREATE, page])
        .expect("create ioctl");
    let id = emu.maps.read_dword(page + 32).expect("id_out");

    emu.maps.write_dword(page, id);
    emu.call_module_symbol("tlm_ioctl", &[0, TLM_IOC_DESTROY, page])
        .expect("destroy ioctl");

    // Free the object a second time behind the driver's back: the ledger must
    // recognise the pointer as already quarantined.
    let chunk = emu
        .kernel
        .as_ref()
        .unwrap()
        .heap
        .chunks()
        .iter()
        .find(|c| c.cache == "tlm_channel")
        .map(|c| c.addr)
        .expect("the channel object should be in the ledger");
    emu.kernel_free(chunk, "kmem_cache_free");

    assert!(
        emu.kernel_findings()
            .iter()
            .any(|f| f.kind == FindingKind::DoubleFree),
        "expected a double-free finding, got {:?}",
        emu.kernel_findings()
            .iter()
            .map(|f| f.kind)
            .collect::<Vec<_>>()
    );
}

/// A freshly loaded `.ko` must be a self-consistent image: every placed section
/// and relocated symbol has to land inside the module's own address range. A
/// mis-placed section or an unapplied relocation would push a symbol (or the
/// init/exit entry) outside `[base, base + size)`, which is exactly the class of
/// loader bug that silently breaks driver emulation.
#[test]
fn kernel_module_layout_and_symbols_are_consistent() {
    let Some(emu) = boot_driver() else { return };
    let m = &emu.kernel.as_ref().expect("kernel env").module;

    assert!(m.size > 0, "module image has zero size");
    let range = m.base..(m.base + m.size);

    let init = m.init.expect("init_module resolved");
    let exit = m.exit.expect("cleanup_module resolved");
    assert!(
        range.contains(&init),
        "init 0x{:x} outside image {:x?}",
        init,
        range
    );
    assert!(
        range.contains(&exit),
        "exit 0x{:x} outside image {:x?}",
        exit,
        range
    );

    // Every defined function symbol must resolve inside the placed image.
    for s in m.symbols.iter().filter(|s| s.is_func && s.addr != 0) {
        assert!(
            range.contains(&s.addr),
            "symbol {} at 0x{:x} lies outside the module image {:x?}",
            s.name,
            s.addr,
            range
        );
    }

    // A named lookup round-trips to an address the module actually covers.
    let ioctl = emu
        .module_symbol("tlm_ioctl")
        .expect("the ioctl handler resolves by name");
    assert!(
        range.contains(&ioctl),
        "tlm_ioctl 0x{:x} outside the module image {:x?}",
        ioctl,
        range
    );
}
