//! Module-name rules of the Windows loader: API-set contracts, library-name
//! normalisation and the main image's guest path.

use crate::emu::Emu;

pub fn is_api_set_contract(module: &str) -> bool {
    let module = module.trim().to_ascii_lowercase();
    module.starts_with("api-ms-win-") || module.starts_with("ext-ms-")
}

/// DLLs implementing an API set contract, the host first. ntdll's ApiSet
/// schema (a per-build binary blob) is the authoritative mapping, but the
/// prefixes are stable across builds.
pub fn api_set_hosts(contract: &str) -> &'static [&'static str] {
    let contract = contract.trim().to_ascii_lowercase();
    let prefix = |p: &str| contract.starts_with(p);
    if prefix("api-ms-win-crt-") {
        &["ucrtbase.dll", "msvcp_win.dll", "kernelbase.dll"]
    } else if prefix("api-ms-win-core-rtlsupport-") || prefix("api-ms-win-core-apiquery-") {
        // RtlCompareMemory, RtlUnwind, RtlCaptureContext, ApiSetQueryApiSetPresence
        &["ntdll.dll", "kernelbase.dll"]
    } else if prefix("api-ms-win-shcore-") {
        &["shcore.dll", "kernelbase.dll"]
    } else if prefix("api-ms-win-eventing-") {
        // ETW user-mode helpers live in ntdll; advapi32 for legacy callers
        &["ntdll.dll", "kernelbase.dll", "advapi32.dll"]
    } else if prefix("api-ms-win-security-") {
        &["sechost.dll", "advapi32.dll", "kernelbase.dll"]
    } else if prefix("api-ms-win-service-") {
        &["sechost.dll", "kernelbase.dll"]
    } else {
        &["kernelbase.dll", "kernel32.dll"]
    }
}

pub fn normalize_library_name(libname: &str, allow_exe_suffix: bool) -> Option<String> {
    // Guest paths use Windows separators even when the host is Unix.
    // Only the basename is looked up inside the host maps folder.
    let mut dll = libname
        .trim()
        .rsplit(['\\', '/'])
        .next()?
        .to_ascii_lowercase();
    if dll.is_empty() || dll.contains(':') {
        return None;
    }

    if allow_exe_suffix {
        if !dll.ends_with(".dll") && !dll.ends_with(".exe") {
            dll.push_str(".dll");
        }
    } else if !dll.ends_with(".dll") {
        dll.push_str(".dll");
    }

    Some(dll)
}

pub fn module_name_matches(flink_mod: &str, want: &str) -> bool {
    normalize_module_name(flink_mod) == normalize_module_name(want)
}

fn normalize_module_name(module_name: &str) -> String {
    let module_name = module_name.trim().to_ascii_lowercase();
    let module_name = module_name
        .rsplit_once('\\')
        .map(|(_, name)| name)
        .unwrap_or(&module_name);
    let module_name = module_name
        .rsplit_once('/')
        .map(|(_, name)| name)
        .unwrap_or(module_name);
    module_name
        .strip_suffix(".dll")
        .unwrap_or(module_name)
        .to_string()
}

pub fn main_module_path(emu: &Emu) -> String {
    format!("C:\\{}", emu.cfg.exe_name)
}
