//! Generate `include/mwemu.h` from the `extern "C"` surface in `src/lib.rs`.
//!
//! cbindgen only rewrites the header when its contents change, so committing the
//! generated `include/mwemu.h` and regenerating on build keeps the two in sync
//! without churn. C consumers can use the committed header without a Rust build.

fn main() {
    let crate_dir = std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR");
    let out = std::path::PathBuf::from(&crate_dir)
        .join("include")
        .join("mwemu.h");

    match cbindgen::generate(&crate_dir) {
        Ok(bindings) => {
            bindings.write_to_file(&out);
        }
        Err(e) => {
            // Don't fail the whole build if header generation hiccups; the
            // committed header remains valid. Surface it as a warning.
            println!("cargo:warning=cbindgen failed to generate mwemu.h: {}", e);
        }
    }

    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rerun-if-changed=cbindgen.toml");
}
