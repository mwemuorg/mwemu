use std::io::Write as _;
use std::path::PathBuf;
use std::sync::Once;

static INIT: Once = Once::new();

pub fn setup() {
    INIT.call_once(|| {
        env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("error"))
            .format(|buf, record| writeln!(buf, "{}", record.args()))
            .init();
    });
}

/// Errors raised when a test body exceeds its wall-clock budget.
#[derive(Debug)]
pub struct TimeoutError {
    pub elapsed: std::time::Duration,
    pub budget: std::time::Duration,
}

impl std::fmt::Display for TimeoutError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "test exceeded wall-clock budget: {:?} >= {:?}",
            self.elapsed, self.budget
        )
    }
}

impl std::error::Error for TimeoutError {}

/// Run `body` on a worker thread, returning `Err(TimeoutError)` if it does not
/// finish within `budget`. Cooperative only — the body is responsible for
/// reporting its own progress if it would otherwise lock up. The point is to
/// give cargo's 60-s per-test default an out: a runaway `step()`/`run()` loop
/// is killed at the budget boundary instead of stalling the whole suite.
///
/// On budget exhaustion the worker thread is detached (the `JoinHandle` is
/// dropped). That is acceptable for a test that has hit its budget — the
/// process exits shortly after cargo reports the test result. Panics inside
/// `body` propagate normally so test failures still surface as cargo's
/// `FAILED` output.
pub fn run_with_timeout<F>(
    budget: std::time::Duration,
    body: F,
) -> Result<(), Box<dyn std::error::Error>>
where
    F: FnOnce() + Send + 'static,
{
    let (tx, rx) = std::sync::mpsc::channel::<()>();
    let _handle = std::thread::Builder::new()
        .name("test-body".into())
        .spawn(move || {
            body();
            let _ = tx.send(());
        })?;
    let started = std::time::Instant::now();
    match rx.recv_timeout(budget) {
        Ok(()) => Ok(()),
        Err(std::sync::mpsc::RecvTimeoutError::Timeout) => Err(Box::new(TimeoutError {
            elapsed: started.elapsed(),
            budget,
        })),
        Err(std::sync::mpsc::RecvTimeoutError::Disconnected) => Err(Box::new(TimeoutError {
            elapsed: started.elapsed(),
            budget,
        })),
    }
}

pub const TEST_BUDGET: std::time::Duration = std::time::Duration::from_secs(8);


/// Workspace root. `CARGO_MANIFEST_DIR` is `.../crates/libmwemu`, so go up two
/// levels: the canonical `test/` and `maps/` data live at the repo root (shared
/// with the CLI), not duplicated per crate.
fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
}

/// `rel` is a filename inside the repo top-level `test/` directory (e.g. `exe64win_msgbox.bin`).
/// Resolves relative to the workspace root so tests work regardless of the CWD.
pub fn test_data_path(rel: &str) -> String {
    repo_root()
        .join("test")
        .join(rel)
        .to_string_lossy()
        .into_owned()
}

/// Maps folder for 32-bit Windows samples (`maps/windows/x86/`).
pub fn win32_maps_folder() -> String {
    let mut s = repo_root()
        .join("maps/windows/x86")
        .to_string_lossy()
        .into_owned();
    if !s.ends_with('/') {
        s.push('/');
    }
    s
}

/// Populate `emu`'s maps folder with a genuine Windows system32 fetched from
/// Microsoft's symbol server (the `--winver` mechanism), so the deep `--ssdt`
/// loader tests run on Linux/macOS without a Windows VM or an ISO.
///
/// Returns `false` when the maps can't be obtained — no network, or the non-PE
/// NLS code-page tables (which aren't on the symbol server) couldn't be seeded
/// from an existing `--iso` cache. Callers should treat `false` as "skip" so
/// the suite stays green on offline machines rather than failing spuriously.
pub fn set_winver_maps(emu: &mut crate::emu::Emu, version: &str) -> bool {
    if let Err(e) = emu.set_maps_from_winver(version) {
        eprintln!("skipping: --winver {} unavailable ({})", version, e);
        return false;
    }
    // The loader needs the NLS tables; --winver seeds them from an iso cache if
    // present. Without them DLL-name lookups produce zeros and the load fails,
    // so skip rather than report a misleading failure.
    let nls = std::path::Path::new(&emu.cfg.maps_folder).join("locale.nls");
    if !nls.is_file() {
        eprintln!(
            "skipping: --winver {} has no NLS tables (seed them once from --iso)",
            version
        );
        return false;
    }
    true
}

/// Maps folder for 64-bit Windows samples (`maps/windows/x86_64/`).
pub fn win64_maps_folder() -> String {
    let mut s = repo_root()
        .join("maps/windows/x86_64")
        .to_string_lossy()
        .into_owned();
    if !s.ends_with('/') {
        s.push('/');
    }
    s
}

pub fn critical_values(bits: u32) -> Vec<u64> {
    let max = match bits {
        8 => u8::MAX as u64,
        16 => u16::MAX as u64,
        32 => u32::MAX as u64,
        64 => u64::MAX,
        _ => panic!("Unsupported size"),
    };

    let sign_bit = 1u64 << (bits - 1);

    vec![
        0,
        1,
        max,
        sign_bit,
        sign_bit - 1,
        sign_bit + 1,
        0x55,                                 // 01010101
        0xAA,                                 // 10101010
        0xFFFFFFFFFFFFFFFFu64 >> (64 - bits), // all 1s for the width
    ]
}

pub fn shift_counts(bits: u32) -> Vec<u64> {
    vec![
        0,
        1,
        bits as u64 - 1,
        bits as u64,
        bits as u64 + 1,
        63,
        64,
        127,
    ]
}

use crate::emu::Emu;
use crate::maps::mem64::Permission;

/// Make sure `rsp`/`esp` points inside a writable stack region, mapping one on
/// demand. Used by `call_winapi32`/`call_winapi64` so a bare `emu32()`/`emu64()`
/// can invoke an API hook without the caller hand-rolling a stack.
fn ensure_stack(emu: &mut Emu) {
    let sp = if emu.cfg.arch.is_64bits() {
        emu.regs().rsp
    } else {
        emu.regs().get_esp()
    };
    if emu.maps.is_mapped(sp) && emu.maps.is_mapped(sp.wrapping_sub(0x1000)) {
        return;
    }
    let sz = 0x40000u64;
    let base = emu.maps.alloc(sz).expect("cannot reserve test stack");
    emu.maps
        .create_map("teststack", base, sz, Permission::READ_WRITE)
        .expect("cannot create test stack");
    let sp = base + sz / 2;
    if emu.cfg.arch.is_64bits() {
        emu.regs_mut().rsp = sp;
    } else {
        emu.regs_mut().set_esp(sp);
    }
}

/// Invoke a 64-bit WinAPI hook honoring the Windows x64 calling convention,
/// exactly as the engine presents it to a hook: the `call`'s return address has
/// already been popped, so `rsp` points at the base of the 32-byte shadow space
/// and the 5th+ stack arguments live at `rsp+0x20`, `rsp+0x28`, ... The first
/// four arguments go in `rcx`, `rdx`, `r8`, `r9`. Returns the hook's `rax`.
pub fn call_winapi64(emu: &mut Emu, func: fn(&mut Emu), args: &[u64]) -> u64 {
    ensure_stack(emu);

    let reg_setters: [fn(&mut Emu, u64); 4] = [
        |e, v| e.regs_mut().rcx = v,
        |e, v| e.regs_mut().rdx = v,
        |e, v| e.regs_mut().r8 = v,
        |e, v| e.regs_mut().r9 = v,
    ];
    for (i, &a) in args.iter().take(4).enumerate() {
        reg_setters[i](emu, a);
    }
    // Stack arguments (5th onward) at rsp+0x20, rsp+0x28, ...
    let rsp = emu.regs().rsp;
    for (i, &a) in args.iter().enumerate().skip(4) {
        let slot = rsp + 0x20 + ((i - 4) as u64) * 8;
        emu.maps.write_qword(slot, a);
    }

    func(emu);
    emu.regs().rax
}

/// Register a module in the export-index registry from a synthetic parsed
/// export directory, exactly as `Emu::load_pe32`/`load_pe64` do after parsing
/// a real PE. Lets `GetProcAddress`/resolver tests run without system DLLs.
pub fn register_export_module(
    emu: &mut Emu,
    module: &str,
    base: u64,
    parsed: &rs_header::pe::export_index::ExportIndexData,
) {
    use crate::api::windows::export_index::{ModuleExportIndex, normalize_module_name};
    let index = ModuleExportIndex::from_parsed(
        module.to_string(),
        normalize_module_name(module),
        base,
        parsed,
    );
    emu.export_indexes.register(index);
}

/// Register a synthetic module in the emulator's export-index registry:
/// `fake.dll` at `base`, export ordinal base 5, with a single function at
/// `base + 0x1500` named `CreateFileA` (ordinal 5).
pub fn register_fake_export_module(emu: &mut Emu, base: u64) {
    use rs_header::pe::export_index::{ExportIndexData, ExportTarget, NamedExport};

    let parsed = ExportIndexData {
        export_base: 5,
        number_of_functions: 1,
        ordinal_targets: vec![Some(ExportTarget::Direct { rva: 0x1500 })],
        named_exports: vec![NamedExport {
            name: "CreateFileA".to_string(),
            ordinal_index: 0,
        }],
    };
    register_export_module(emu, "fake.dll", base, &parsed);
}

/// Invoke a 32-bit WinAPI hook honoring the `stdcall` convention as the engine
/// presents it: the return address has already been popped, so arguments sit at
/// `[esp+0]`, `[esp+4]`, ... (the hook pops them itself). Arguments are pushed
/// right-to-left. Returns the hook's `eax`.
pub fn call_winapi32(emu: &mut Emu, func: fn(&mut Emu), args: &[u32]) -> u32 {
    ensure_stack(emu);
    for &a in args.iter().rev() {
        emu.stack_push32(a);
    }
    func(emu);
    emu.regs().get_eax() as u32
}
