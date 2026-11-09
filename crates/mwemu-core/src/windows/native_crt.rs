//! Legacy `msvcrt.dll` runs its real code: calls into its mapped code sections
//! are executed natively instead of being served by an API handler.

use crate::emu;

const NATIVE_SECTIONS: [&str; 2] = ["msvcrt.text", "msvcrtfothk"];

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

pub fn is_native_section(section_name: &str) -> bool {
    NATIVE_SECTIONS.contains(&section_name)
}

pub fn log_native_call(emu: &mut emu::Emu, addr: u64) {
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
