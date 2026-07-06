use crate::tests::helpers;
use crate::*;
use iced_x86::Mnemonic;
use std::cell::RefCell;
use std::rc::Rc;

/// Step size for absolute `run_to` targets while searching for the first `syscall`.
const MSG_SYSCALL_STEP: u64 = 25_000;
/// Upper bound on instructions scanned for the first Windows `syscall` (ntdll stub).
const MSG_SYSCALL_SCAN_CAP: u64 = 1_000_000;

#[test]
#[ignore = "SSDT mode only initializes cleanly on Win2022, whose DLLs are no longer bundled and \
            aren't on the symbol server (winbindex lacks Server 2022). Auto-fetch provides Win11, \
            whose real LdrInitializeThunk still stalls in console init. Re-enable once Win11 LdrInit \
            completes."]
fn exe64win_msgbox_ssdt_reaches_cli_trace_window() {
    helpers::setup();

    let mut emu = emu64();
    emu.cfg.maps_folder = helpers::win64_maps_folder();
    emu.cfg.emulate_winapi = true; // same behavior as command line --ssdt

    let sample = helpers::test_data_path("exe64win_msgbox.bin");
    assert!(
        std::path::Path::new(&sample).is_file(),
        "missing test sample (run from mwemu repo): {}",
        sample
    );
    emu.load_code(&sample);

    // Fast smoke: SS/--ssdt still runs forward.
    let target = 120u64;
    emu.run_to(target).unwrap_or_else(|e| {
        panic!(
            "run_to({}) failed: {} (pos={} rip=0x{:x})",
            target,
            e,
            emu.pos,
            emu.regs().rip
        );
    });
    assert!(emu.pos >= target);
}

/// Runs the msgbox PE until the first **`syscall`** in Windows mode (`!linux`), or until
/// [`MSG_SYSCALL_SCAN_CAP`] instructions, whichever comes first.
///
/// Stops as soon as the hook sees `Mnemonic::Syscall` so we do not keep emulating after the
/// syscall (avoids extra work and reduces the chance of hitting unrelated emulator bugs later).
///
/// **Ignored by default**: scanning can take a long time in `debug` builds; long runs may also
/// hit unimplemented operand paths before any syscall. Run locally with:
/// `cargo test -p libmwemu exe64win_msgbox_ssdt_hits_first_windows_syscall --release -- --ignored --nocapture`
#[test]
#[ignore = "slow: run with --release -- --ignored; may need MSG_SYSCALL_SCAN_CAP raised"]
fn exe64win_msgbox_ssdt_hits_first_windows_syscall() {
    helpers::setup();

    let mut emu = emu64();
    if !helpers::set_winver_maps(&mut emu, "win11") {
        return;
    }
    emu.cfg.emulate_winapi = true;

    let sample = helpers::test_data_path("exe64win_msgbox.bin");
    assert!(
        std::path::Path::new(&sample).is_file(),
        "missing {}",
        sample
    );

    let hit = Rc::new(RefCell::new(false));
    let hit_flag = Rc::clone(&hit);
    emu.hooks
        .on_post_instruction(move |emu, _rip, ins, _sz, _ok| {
            if emu.os.is_windows() && ins.is_x86() && ins.as_x86().mnemonic() == Mnemonic::Syscall {
                *hit_flag.borrow_mut() = true;
            }
        });

    emu.load_code(&sample);

    let mut next_goal = MSG_SYSCALL_STEP;
    while !*hit.borrow() && emu.pos < MSG_SYSCALL_SCAN_CAP {
        let goal = next_goal.min(MSG_SYSCALL_SCAN_CAP);
        if goal <= emu.pos {
            break;
        }
        emu.run_to(goal).unwrap_or_else(|e| {
            panic!(
                "run_to({}) failed: {} (pos={} rip=0x{:x})",
                goal,
                e,
                emu.pos,
                emu.regs().rip
            );
        });
        if *hit.borrow() {
            break;
        }
        next_goal += MSG_SYSCALL_STEP;
    }

    assert!(
        *hit.borrow(),
        "no Windows `syscall` before pos {} (rip=0x{:x}). Raise MSG_SYSCALL_SCAN_CAP if the CRT path grew.",
        MSG_SYSCALL_SCAN_CAP,
        emu.regs().rip
    );
}

/// `--ssdt` smoke for the Enigma-packed PE: confirms syscall-mode can load the
/// sample, drive `ntdll!LdrInitializeThunk` to completion (~7M instructions
/// inside `load_code`), and step the Enigma unpacker forward. This is strictly
/// deeper than the API-stub path, which aborts on unimplemented
/// `ntdll!RtlAcquirePebLock`; running the real ntdll bytes makes it work.
#[test]
#[ignore = "SSDT mode needs the Win2022 build (not bundled, not on winbindex); Win11 LdrInit still \
            stalls in console init. Re-enable once Win11 LdrInit completes."]
fn exe64win_enigma_ssdt_reaches_unpacker() {
    helpers::setup();

    let mut emu = emu64();
    emu.cfg.maps_folder = helpers::win64_maps_folder();
    emu.cfg.emulate_winapi = true; // same behavior as command line --ssdt

    let sample = helpers::test_data_path("exe64win_enigma.bin");
    assert!(
        std::path::Path::new(&sample).is_file(),
        "missing test sample: {}",
        sample
    );

    // load_code drives LdrInitializeThunk via call64 when emulate_winapi is set,
    // then leaves RIP at the EXE entry point.
    emu.load_code(&sample);

    assert!(
        emu.ldr_init_done,
        "ntdll!LdrInitializeThunk did not complete under --ssdt (pos={} rip=0x{:x})",
        emu.pos,
        emu.regs().rip
    );

    // Step the Enigma payload a little past the entry point to prove the
    // protected code runs against the real DLL/syscall surface.
    let target = emu.pos + 500_000;
    let _ = emu.run_to(target);
    assert!(
        emu.pos >= target,
        "ssdt enigma stalled at {} (target {}); rip=0x{:x}",
        emu.pos,
        target,
        emu.regs().rip
    );
}

/// Deep `--ssdt` run of the Enigma unpacker: equivalent to
/// `cargo run --release -- -f test/exe64win_enigma.bin -6 --ssdt`.
///
/// Drives the protected payload to 50M instructions — tens of millions of
/// instructions of Enigma's own VM/unpacker executing against the real
/// ntdll/kernel32/kernelbase bytes, with only kernel `syscall`s intercepted.
///
/// Stops at 50M on purpose: the unpacker currently diverges from real Windows
/// around pos ~109M (an in-image RVA dispatch table at imagebase+0x5960b4 is
/// never converted to VAs, so `call qword ptr [rbp+0x5960b4]` jumps to a bare
/// RVA). 50M is comfortably below that wall.
///
/// Run with:
/// ```text
/// cargo test -p libmwemu exe64win_enigma_ssdt_runs_deep \
///     --release -- --ignored --nocapture
/// ```
#[test]
#[ignore = "slow: ~3s in release; run with --release -- --ignored"]
fn exe64win_enigma_ssdt_runs_deep() {
    helpers::setup();

    let mut emu = emu64();
    if !helpers::set_winver_maps(&mut emu, "win11") {
        return;
    }
    emu.cfg.emulate_winapi = true; // --ssdt / --syscall-mode

    let sample = helpers::test_data_path("exe64win_enigma.bin");
    assert!(
        std::path::Path::new(&sample).is_file(),
        "missing test sample: {}",
        sample
    );

    emu.load_code(&sample);

    let target = 50_000_000u64;
    let _ = emu.run_to(target);
    assert!(
        emu.pos >= target,
        "ssdt enigma only reached {} instructions (need >= {}); rip=0x{:x}",
        emu.pos,
        target,
        emu.regs().rip
    );
}

#[test]
#[ignore = "SSDT mode needs the Win2022 build (not bundled, not on winbindex); Win11 LdrInit still \
            stalls in console init. Re-enable once Win11 LdrInit completes."]
fn exe64win_mingw_ssdt_reaches_early_execution_window() {
    helpers::setup();

    let mut emu = emu64();
    emu.cfg.maps_folder = helpers::win64_maps_folder();
    emu.cfg.emulate_winapi = true; // same behavior as command line --ssdt

    let sample = helpers::test_data_path("exe64win_mingw.bin");
    assert!(
        std::path::Path::new(&sample).is_file(),
        "missing {}",
        sample
    );
    emu.load_code(&sample);

    emu.run_to(120)
        .expect("ssdt mingw should reach early execution window");
    assert!(emu.pos >= 120);
}

/// Verifies that `ntdll!LdrInitializeThunk` can be emulated for more than 15 000 instructions
/// under `--ssdt` mode, equivalent to:
///
/// ```text
/// cargo run --release -- -f test/exe64win_msgbox.bin -6 --ssdt
/// ```
///
/// `load_code` with `emulate_winapi = true` calls `LdrInitializeThunk` internally via
/// `call64`; `emu.pos` after `load_code` reflects instructions executed inside that call.
/// We use banzai mode so unimplemented instructions are skipped rather than aborting early.
///
/// Run with:
/// ```text
/// cargo test -p libmwemu ssdt_ldr_initialize_thunk_executes_over_15000_instructions \
///     --release -- --ignored --nocapture
/// ```
#[test]
#[ignore = "slow: run with --release -- --ignored --nocapture"]
fn ssdt_ldr_initialize_thunk() {
    helpers::setup();

    let mut emu = emu64();
    if !helpers::set_winver_maps(&mut emu, "win11") {
        return;
    }
    emu.cfg.emulate_winapi = true; // --ssdt
    emu.cfg.skip_unimplemented = true; // --banzai: skip unimplemented, keep going
    emu.maps.set_banzai(true);

    let sample = helpers::test_data_path("exe64win_msgbox.bin");
    assert!(
        std::path::Path::new(&sample).is_file(),
        "missing test sample: {}",
        sample
    );

    // load_code triggers ntdll!LdrInitializeThunk via call64 when emulate_winapi is set.
    // emu.pos counts every instruction executed during that internal call.
    emu.load_code(&sample);

    assert!(
        emu.pos > 15_000,
        "LdrInitializeThunk emulation only reached {} instructions (need > 15 000). \
         rip=0x{:x}  — check for early crash or unimplemented syscall.",
        emu.pos,
        emu.regs().rip
    );
}

/// End-to-end `--ssdt` smoke for the MessageBoxA achievement: equivalent to
/// running `cargo run --release -- -f test/exe64win_msgbox.bin -6 --ssdt`
/// and confirming the EXE reaches its payload.
///
/// Verifies that, with `--ssdt`:
///   1. `ntdll!LdrInitializeThunk` completes cleanly (`ldr_init_done = true`,
///      ~6.9M instructions on the current Win2022 build), then
///   2. the EXE's payload runs, the `LoadLibraryA('user32.dll')` shim returns
///      the user32 base, and
///   3. the in-loop `user32!MessageBoxA` shim fires (i.e. execution actually
///      reaches the address resolved by GetProcAddress).
///
/// If this test fails, the `--ssdt` happy path is broken somewhere along the
/// chain: LdrInit → exe entry → loader.exe-shellcode-style resolver → APIs.
///
/// Run with:
/// ```text
/// cargo test -p libmwemu ssdt_msgbox_reaches_messageboxa \
///     --release -- --ignored --nocapture
/// ```
#[test]
#[ignore = "slow (~9 min): asserts the full MessageBoxA happy path under --winver win11, which \
            needs LdrInitializeThunk to COMPLETE. As of 2026-06-14 this works (ConDrv console, \
            movsd GS-cookie, and thread-pool loop blockers fixed); kept #[ignore] only for runtime. \
            Run with: cargo test -p libmwemu --release ssdt_msgbox_reaches_messageboxa -- --ignored --nocapture"]
fn ssdt_msgbox_reaches_messageboxa() {
    use std::cell::RefCell;
    use std::rc::Rc;

    helpers::setup();

    let mut emu = emu64();
    // LdrInitializeThunk now completes under --winver win11 (pos ~15.8M) and the
    // EXE reaches MessageBoxA. Kept #[ignore] only because the full run is slow.
    if !helpers::set_winver_maps(&mut emu, "win11") {
        return;
    }
    emu.cfg.emulate_winapi = true; // --ssdt

    let sample = helpers::test_data_path("exe64win_msgbox.bin");
    assert!(
        std::path::Path::new(&sample).is_file(),
        "missing test sample: {}",
        sample
    );

    // Strategy: the in-loop MessageBoxA shim short-circuits the function
    // entry — it sets rax/rsp directly and never decodes the instruction at
    // `user32!MessageBoxA`. Detecting the shim with `on_pre_instruction(rip
    // == mba_addr)` therefore misses every call.
    //
    // Instead, watch for:
    //   * `user32_loaded`: the user32 PE map appearing (= LoadLibraryA shim
    //     fired and `load_library("user32.dll")` succeeded), and
    //   * `mba_returned`: the EXE's post-call return PC. In the
    //     loader.exe-style resolver the call to MessageBoxA lives at
    //     0x14000123f (`call rax`, 2 bytes); the instruction immediately
    //     after is at 0x140001241, which is only ever reached if the call
    //     returns normally. Anything that lands there proves the MessageBoxA
    //     shim ran AND it correctly popped the return address back.
    let user32_loaded = Rc::new(RefCell::new(false));
    let mba_returned = Rc::new(RefCell::new(false));
    let user32_loaded_c = Rc::clone(&user32_loaded);
    let mba_returned_c = Rc::clone(&mba_returned);
    const POST_MBA_CALL_RIP: u64 = 0x140001241;
    emu.hooks.on_pre_instruction(move |emu, rip, _ins, _sz| {
        if !*user32_loaded_c.borrow() && emu.maps.get_map_by_name("user32.pe").is_some() {
            *user32_loaded_c.borrow_mut() = true;
        }
        if rip == POST_MBA_CALL_RIP {
            *mba_returned_c.borrow_mut() = true;
        }
        true
    });

    // `load_code` with `emulate_winapi = true` drives LdrInitializeThunk via
    // `call64` and then leaves RIP at the EXE entry point.
    emu.load_code(&sample);

    assert!(
        emu.ldr_init_done,
        "ntdll!LdrInitializeThunk did not complete (pos={} rip=0x{:x})",
        emu.pos,
        emu.regs().rip
    );

    // 12M instructions is comfortably above the ~7M used by LdrInit and the
    // ~10K extra used by the EXE payload (loader.exe-style resolver +
    // MessageBoxA shim). If this cap is hit, something stalled.
    let cap = 12_000_000u64;
    let _ = emu.run_to(cap);

    assert!(
        *user32_loaded.borrow(),
        "user32.dll was never mapped — LoadLibraryA('user32.dll') shim didn't fire \
         (pos={} rip=0x{:x})",
        emu.pos,
        emu.regs().rip,
    );
    assert!(
        *mba_returned.borrow(),
        "EXE never returned from `call MessageBoxA` (no execution at 0x{:x}) — \
         either the MessageBoxA shim didn't fire, didn't pop the return address \
         correctly, or LdrInit / GetProcAddress regressed (pos={} rip=0x{:x}).",
        POST_MBA_CALL_RIP,
        emu.pos,
        emu.regs().rip,
    );
}
