//! Legacy `msvcrt.dll` native-execution helpers.
//!
//! The x64 `msvcrt.text` and `msvcrtfothk` sections produced by the PE loader
//! carry the actual CRT machine code. After this change, calls into either of
//! those mapped sections leave RIP at the target and execute the real bytes
//! directly — no Rust-side emulation remains. Two helpers stay here so the
//! instruction-pointer dispatcher can route native calls and emit the
//! verbose-gated named execution log that downstream tooling and mwemu-mcp
//! consume:
//!
//! * [`is_native_section`] classifies the loader-produced map name.
//! * [`log_native_call`] resolves the exact export via the existing export
//!   index and emits a red `log_red!` line for every name on the current list.
//!
//! Adding a new function name to the log list is a single-line change in
//! [`log_native_call`]; no dispatcher arm or stub emulation is required.
//! Functions not on the list still execute natively — the list governs logging
//! only.
//!
//! Scope is x64 only; x86 `winapi32::msvcrt` and AArch64 `set_pc_aarch64`
//! remain unchanged. AArch64 PEs that happen to land in a map named
//! `msvcrt.text` would hit `winapi64::gateway`'s `unreachable!` like any other
//! unknown section, consistent with the rest of the dispatcher.

use crate::emu;

/// Section names produced by the PE loader that carry x64 `msvcrt.dll`
/// executable code and therefore must be executed natively rather than
/// emulated.
const NATIVE_SECTIONS: [&str; 2] = ["msvcrt.text", "msvcrtfothk"];

/// Exported function names that currently receive a verbose-gated execution
/// log on native entry. Extend by appending the name; no other change is
/// required.
const LOGGED_FUNCTIONS: [&str; 8] = [
    "__set_app_type",
    "malloc",
    "realloc",
    "_errno",
    "_lock",
    "__dllonexit",
    "_msize",
    "_initterm",
];

/// Returns `true` iff `section_name` carries the loader-mapped x64
/// `msvcrt.dll` executable bytes. The comparison is exact on purpose:
/// `msvcrt.rdata`/`msvcrt.data` are data and must not be executed.
pub(crate) fn is_native_section(section_name: &str) -> bool {
    NATIVE_SECTIONS.contains(&section_name)
}

/// Emit a red `log_red!` line for the native call at `addr` when the resolver
/// recognises the name as one of the current logged functions. Native code
/// owns behavior; this helper preserves only named execution observability.
pub(crate) fn log_native_call(emu: &mut emu::Emu, addr: u64) {
    // Resolve against the registered export index only. Native msvcrt
    // execution routes *every* internal CRT call through here (see
    // `set_rip_with_check`), and internal (non-exported) functions are not in
    // the index. Falling back to the PEB export-table walk would scan every
    // exported function of every loaded DLL per call — a multi-second stall
    // before a single instruction is emulated. Index misses simply have no
    // log name and are skipped.
    let Some(fn_name) = emu.export_indexes.resolve_address_module("msvcrt", addr) else {
        return;
    };
    if !LOGGED_FUNCTIONS.contains(&fn_name) {
        return;
    }
    log_red!(emu, "executing msvcrt!{}", fn_name);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn native_sections_match_exact_loader_names() {
        assert!(is_native_section("msvcrt.text"));
        assert!(is_native_section("msvcrtfothk"));
        assert!(!is_native_section("msvcrt.rdata"));
        assert!(!is_native_section("msvcrt.data"));
        assert!(!is_native_section(""));
        assert!(!is_native_section("MSVCRT.TEXT"));
        assert!(!is_native_section("msvcrt.text "));
        assert!(!is_native_section("kernel32.text"));
    }
}
