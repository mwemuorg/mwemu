//! PE TLS callback execution on real samples (sample-bundle gated).
//!
//! These exercise the full chain that makes a TLS-using PE actually run:
//!   * TLS directory parsing + callback rebasing (`get_tls_callbacks`),
//!   * running each callback before the entry point (Win64 ABI, best-effort),
//!   * and — for mingw — the api-set import routing + ntdll→kernel32 gateway
//!     delegation the CRT init relies on.
//!
//! Self-contained coverage (no bundle) lives in `loaders::hello_world`
//! (`hello_win_x64_runs_tls_callbacks`); these add the end-to-end proof on the
//! real mingw/msgbox binaries and skip when the bundle is absent.

use crate::tests::helpers;
use crate::*;

/// mingw's x64 PE declares two TLS callbacks. Loading must detect + rebase them,
/// and driving execution past CRT init (which calls api-set CRT imports through
/// the callbacks' groundwork) must not hit the old `deref qword on 0x0` failure
/// that used to stop this binary at pos ~254.
#[test]
fn mingw64_executes_tls_callbacks() {
    helpers::setup();

    let mut emu = emu64();
    emu.cfg.maps_folder = win_maps!(64);

    let sample = sample!("exe64win_mingw.bin");
    emu.load_code(&sample);

    // Detected + rebased into mapped, executable addresses.
    assert_eq!(
        emu.tls_callbacks.len(),
        2,
        "mingw64 declares 2 TLS callbacks; got {:?}",
        emu.tls_callbacks
    );
    for &cb in &emu.tls_callbacks {
        assert!(
            emu.maps.get_addr_name(cb).is_some(),
            "TLS callback 0x{:x} should be a mapped address",
            cb
        );
    }

    // Run well past the CRT bootstrap. Before the TLS + api-set-routing fixes,
    // mingw died dereferencing an unbound `__p___argv` IAT slot around pos 254;
    // reaching 300 proves the callbacks ran and the CRT init got wired up.
    emu.run_to(300).expect("mingw64 should run past CRT init");
    assert!(emu.pos >= 300, "expected to reach pos 300, got {}", emu.pos);
}

/// Implicit TLS: _tls_index must be written and TEB.ThreadLocalStoragePointer
/// must point to a valid pointer array whose first entry is the TLS data region.
#[test]
fn mingw64_implicit_tls_is_initialized() {
    helpers::setup();

    let mut emu = emu64();
    emu.cfg.maps_folder = win_maps!(64);

    let sample = sample!("exe64win_mingw.bin");
    emu.load_code(&sample);

    // _tls_index must be 0 (written by the loader)
    let tls_index_va = 0x14000707c_u64;
    let tls_index = emu.maps.read_dword(tls_index_va).unwrap_or(0xFFFF);
    assert_eq!(tls_index, 0, "_tls_index must be 0 for the main module");

    // TEB+0x58 (ThreadLocalStoragePointer) must be non-zero
    let teb_base = emu.maps.get_mem("teb").get_base();
    let tls_ptrs = emu.maps.read_qword(teb_base + 0x58).unwrap_or(0);
    assert_ne!(tls_ptrs, 0, "TEB.ThreadLocalStoragePointer must be set");

    // The pointer array's first entry must point to mapped TLS data
    let tls_data = emu.maps.read_qword(tls_ptrs).unwrap_or(0);
    assert_ne!(tls_data, 0, "TLS pointer array[0] must point to TLS data");
    assert!(
        emu.maps.get_addr_name(tls_data).is_some(),
        "TLS data at 0x{:x} must be in a mapped region",
        tls_data
    );
}

#[test]
fn mingw32_implicit_tls_is_initialized() {
    helpers::setup();

    let mut emu = emu32();
    emu.cfg.maps_folder = win_maps!(32);

    let sample = sample!("exe32win_mingw.bin");
    emu.load_code(&sample);

    // _tls_index must be 0
    let tls_index_va = 0x405038_u64;
    let tls_index = emu.maps.read_dword(tls_index_va).unwrap_or(0xFFFF);
    assert_eq!(tls_index, 0, "_tls_index must be 0 for the main module");

    // TEB+0x2C (ThreadLocalStoragePointer) must be non-zero
    let teb_base = emu.maps.get_mem("teb").get_base();
    let tls_ptrs = emu.maps.read_dword(teb_base + 0x2C).unwrap_or(0);
    assert_ne!(tls_ptrs, 0, "TEB.ThreadLocalStoragePointer must be set");

    // The pointer array's first entry must point to mapped TLS data
    let tls_data = emu.maps.read_dword(tls_ptrs as u64).unwrap_or(0);
    assert_ne!(tls_data, 0, "TLS pointer array[0] must point to TLS data");
    assert!(
        emu.maps.get_addr_name(tls_data as u64).is_some(),
        "TLS data at 0x{:x} must be in a mapped region",
        tls_data
    );
}

/// A PE without a TLS directory must expose zero callbacks. msgbox has no `.tls`;
/// before the `get_tls_callbacks` guard it misparsed offset 0 into bogus
/// callback addresses (e.g. 0x300905a4d) that then faulted on execution.
#[test]
fn msgbox_has_no_tls_callbacks() {
    helpers::setup();

    let mut emu = emu64();
    emu.cfg.maps_folder = win_maps!(64);

    let sample = sample!("exe64win_msgbox.bin");
    emu.load_code(&sample);

    assert!(
        emu.tls_callbacks.is_empty(),
        "msgbox has no TLS directory; callbacks must be empty (no garbage reads); got {:?}",
        emu.tls_callbacks
    );
}
