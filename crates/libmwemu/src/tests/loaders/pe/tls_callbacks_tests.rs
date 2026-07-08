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
    emu.cfg.maps_folder = helpers::win64_maps_folder();

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

/// A PE without a TLS directory must expose zero callbacks. msgbox has no `.tls`;
/// before the `get_tls_callbacks` guard it misparsed offset 0 into bogus
/// callback addresses (e.g. 0x300905a4d) that then faulted on execution.
#[test]
fn msgbox_has_no_tls_callbacks() {
    helpers::setup();

    let mut emu = emu64();
    emu.cfg.maps_folder = helpers::win64_maps_folder();

    let sample = sample!("exe64win_msgbox.bin");
    emu.load_code(&sample);

    assert!(
        emu.tls_callbacks.is_empty(),
        "msgbox has no TLS directory; callbacks must be empty (no garbage reads); got {:?}",
        emu.tls_callbacks
    );
}
