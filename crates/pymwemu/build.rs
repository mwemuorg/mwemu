fn main() {
    // pyo3-stub-gen references `PyEncodingWarning`, which pyo3 only defines
    // for Python >= 3.10, so an older interpreter fails with a cryptic error
    // deep inside that dependency. macOS ships 3.9 as `python3`, and that is
    // what pyo3 picks when PYO3_PYTHON is unset: fail here with the fix instead.
    let cfg = pyo3_build_config::get();
    if (cfg.version.major, cfg.version.minor) < (3, 10) {
        let exe = cfg
            .executable
            .clone()
            .unwrap_or_else(|| "python3".to_string());
        panic!(
            "pymwemu needs Python >= 3.10 to build (pyo3-stub-gen), but pyo3 found Python {} \
             at {}. Point it at a newer interpreter, e.g. \
             `PYO3_PYTHON=$(brew --prefix)/bin/python3.14 cargo build -p pymwemu` \
             (brew install python), or build inside a venv created from it \
             (`make -C crates/pymwemu setup-env`).",
            cfg.version, exe
        );
    }
    pyo3_build_config::add_extension_module_link_args();
}
