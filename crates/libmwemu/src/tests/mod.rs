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

/// Windows maps folder for 32 or 64 bits, or **skip** the test when its
/// DLLs are not provisioned. Tests never download: `make symbols` fetches the
/// DLLs once beforehand. Usage: `emu.cfg.maps_folder = win_maps!(64);`
macro_rules! win_maps {
    (32) => {
        win_maps!(@folder crate::tests::helpers::win32_maps_folder())
    };
    (64) => {
        win_maps!(@folder crate::tests::helpers::win64_maps_folder())
    };
    (@folder $folder:expr) => {{
        let f = $folder;
        if !crate::tests::helpers::win_maps_ready(&f) {
            eprintln!(
                "[skip] {}: Windows DLLs missing in {} (run `make symbols`)",
                module_path!(),
                f
            );
            return;
        }
        f
    }};
}

mod isa;
mod kernel;
mod loaders;
mod os;
mod robustness;
mod shellcode;
mod unit;
