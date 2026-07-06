mod helpers;

/// Resolve a path in the sample bundle, or **skip** the test (returns early,
/// passing silently) when the bundle isn't present. CI runs without the bundle,
/// so binary-dependent tests no-op there while the self-contained suite runs;
/// `make tests` fetches the bundle locally so everything runs. Usage:
/// `emu.load_code(&sample!("exe64win_msgbox.bin"));`
macro_rules! sample {
    ($rel:expr) => {{
        let p = crate::tests::helpers::test_data_path($rel);
        if !std::path::Path::new(&p).exists() {
            eprintln!(
                "[skip] {}: sample '{}' not present (run `make samples`)",
                module_path!(),
                $rel
            );
            return;
        }
        p
    }};
}

mod isa;
mod loaders;
mod os;
mod shellcode;
mod unit;
