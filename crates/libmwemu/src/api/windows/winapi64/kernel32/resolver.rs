use crate::api::windows::common::kernel32 as kernel32_common;
use crate::api::windows::export_index::IndexedExport;
use crate::emu;
use crate::windows::peb::peb64;

/// Maximum depth for forwarder chain resolution. Kept in sync with
/// `crate::api::windows::export_index::MAX_FORWARDER_DEPTH`.
const FORWARDER_MAX_DEPTH: u32 = 8;

pub fn dump_module_iat(emu: &mut emu::Emu, module: &str) {
    let needle = module.to_ascii_lowercase();
    if emu.export_indexes.len() == 0 {
        dump_module_iat_scanner(emu, &needle);
        return;
    }
    for index in emu.export_indexes.iter_ordered() {
        if !index.normalized_name.contains(&needle) {
            continue;
        }
        log::trace!("---- exports of {} ----", index.module_name);
        for named in index_module_named_list(index) {
            log::trace!("0x{:x} {}!{}", named.1, index.module_name, named.0);
        }
    }
}

fn dump_module_iat_scanner(emu: &mut emu::Emu, module_lc: &str) {
    let mut flink = peb64::Flink::new(emu);
    flink.load(emu);
    let first_ptr = flink.get_ptr();

    loop {
        if flink.mod_name.to_lowercase().contains(module_lc) && flink.export_table_rva > 0 {
            for i in 0..flink.num_of_funcs {
                if flink.pe_hdr == 0 {
                    continue;
                }

                let ordinal = flink.get_function_ordinal(emu, i);
                log::trace!(
                    "0x{:x} {}!{}",
                    ordinal.func_va,
                    flink.mod_name,
                    ordinal.func_name
                );
            }
        }
        if !flink.next(emu) || flink.get_ptr() == first_ptr {
            break;
        }
    }
}

pub fn resolve_api_addr_to_name(emu: &mut emu::Emu, addr: u64) -> String {
    if let Some(n) = emu.api_addr_name_cache.get(&addr) {
        return n.clone();
    }
    let name = resolve_api_addr_to_name_uncached(emu, addr);
    if !name.is_empty() {
        emu.api_addr_name_cache.insert(addr, name.clone());
    }
    name
}

fn resolve_api_addr_to_name_uncached(emu: &mut emu::Emu, addr: u64) -> String {
    // Index-first: O(1) lookups against the registered direct-address map.
    for module in emu.export_indexes.iter_ordered() {
        if let Some(name) = module.resolve_address(addr) {
            return name.to_string();
        }
    }
    // Fallback: walk the PEB LDR list.
    let mut flink = peb64::Flink::new(emu);
    flink.load(emu);
    let first_ptr = flink.get_ptr();

    loop {
        if flink.export_table_rva > 0 {
            for i in 0..flink.num_of_funcs {
                if flink.pe_hdr == 0 {
                    continue;
                }

                let ordinal = flink.get_function_ordinal(emu, i);
                if ordinal.func_va == addr {
                    return ordinal.func_name.to_string();
                }
            }
        }
        if !flink.next(emu) || flink.get_ptr() == first_ptr {
            break;
        }
    }

    String::new()
}

fn resolve_in_module_exports_depth(
    emu: &mut emu::Emu,
    module_hint: &str,
    name: &str,
    depth: u32,
) -> u64 {
    if depth > FORWARDER_MAX_DEPTH {
        return 0;
    }
    let want = module_hint.trim().to_ascii_lowercase();
    let name_lc = name.to_ascii_lowercase();

    let mut flink = peb64::Flink::new(emu);
    flink.load(emu);
    let first_ptr = flink.get_ptr();
    loop {
        if flink.export_table_rva > 0
            && kernel32_common::module_name_matches(&flink.mod_name, &want)
        {
            for i in 0..flink.num_of_funcs {
                if flink.pe_hdr == 0 {
                    continue;
                }

                let func_name_rva = emu
                    .maps
                    .read_dword(flink.func_name_tbl + i * 4)
                    .unwrap_or(0) as u64;
                let export_name = emu.maps.read_string(func_name_rva + flink.mod_base);
                if export_name.to_lowercase() != name_lc {
                    continue;
                }

                let ordinal = flink.get_function_ordinal_depth(emu, i, depth);
                return ordinal.func_va;
            }
        }
        if !flink.next(emu) || flink.get_ptr() == first_ptr {
            break;
        }
    }

    0
}

/// Resolve `name` preferring the module named in the PE import table (`KERNEL32.DLL`, etc.).
/// API-set / `ext-ms-*` names are mapped to the backing DLLs used in `maps/windows/` (see `get_dependencies`).
pub fn resolve_api_name_in_module(emu: &mut emu::Emu, module: &str, name: &str) -> u64 {
    let module_lc = module.trim().to_ascii_lowercase();

    // Memoize: the (module, name) -> VA mapping is stable for a run, and the
    // loader re-resolves the same apiset imports ~100x (the kernelbase dance),
    // each time scanning every export name (read_string + to_lowercase). The
    // cache turns that O(exports) scan into a hash hit.
    let cache_key = {
        let mut k = String::with_capacity(module_lc.len() + 1 + name.len());
        k.push_str(&module_lc);
        k.push('\x01');
        k.push_str(name);
        k
    };
    if let Some(&a) = emu.api_resolve_cache.get(&cache_key) {
        return a;
    }
    let resolved = resolve_api_name_in_module_inner(emu, &module_lc, name);
    if resolved != 0 {
        emu.api_resolve_cache.insert(cache_key, resolved);
    }
    resolved
}

fn resolve_api_name_in_module_inner(emu: &mut emu::Emu, module_lc: &str, name: &str) -> u64 {
    let module_lc = module_lc.to_string();

    if kernel32_common::is_api_set_contract(&module_lc) {
        // 1) Try the apiset stub itself (host-side index first).
        let addr = emu.export_indexes.resolve_name_in_module(&module_lc, name);
        if addr != 0 {
            return addr;
        }
        let addr = resolve_in_module_exports_depth(emu, &module_lc, name, 0);
        if addr != 0 {
            return addr;
        }
        // 2) Fallback: map by contract category. ntdll's ApiSet schema (a
        //    binary blob compiled per-build) gives the authoritative mapping,
        //    but the high-traffic prefixes are stable across builds and
        //    cheaper to check inline.
        let fallback_modules: &[&str] = if module_lc.starts_with("api-ms-win-crt-") {
            &["ucrtbase.dll", "msvcp_win.dll", "kernelbase.dll"]
        } else if module_lc.starts_with("api-ms-win-core-rtlsupport-") {
            // RtlCompareMemory, RtlUnwind, RtlCaptureContext, … all live in ntdll.
            &["ntdll.dll", "kernelbase.dll"]
        } else if module_lc.starts_with("api-ms-win-core-apiquery-") {
            // ApiSetQueryApiSetPresence — ntdll-resident query helper.
            &["ntdll.dll", "kernelbase.dll"]
        } else if module_lc.starts_with("api-ms-win-shcore-") {
            &["shcore.dll", "kernelbase.dll"]
        } else if module_lc.starts_with("api-ms-win-eventing-") {
            // EventRegister, EventWrite*, EventActivityIdControl — in ntdll
            // on recent Windows (ETW user-mode helpers); fall back to advapi32
            // for the legacy callers.
            &["ntdll.dll", "kernelbase.dll", "advapi32.dll"]
        } else if module_lc.starts_with("api-ms-win-security-") {
            &["sechost.dll", "advapi32.dll", "kernelbase.dll"]
        } else if module_lc.starts_with("api-ms-win-service-") {
            &["sechost.dll", "kernelbase.dll"]
        } else {
            &["kernelbase.dll", "kernel32.dll"]
        };
        for m in fallback_modules {
            // Index-first, scanner fallback.
            let addr = emu.export_indexes.resolve_name_in_module(m, name);
            if addr != 0 {
                return addr;
            }
            let addr = resolve_in_module_exports_depth(emu, m, name, 0);
            if addr != 0 {
                return addr;
            }
        }
        return 0;
    }

    // Index-first.
    let addr = emu.export_indexes.resolve_name_in_module(&module_lc, name);
    if addr != 0 {
        return addr;
    }
    let addr = resolve_in_module_exports_depth(emu, &module_lc, name, 0);
    if addr != 0 {
        return addr;
    }
    resolve_api_name(emu, name)
}

/// Resolve a PE export forwarder string (`KERNELBASE.QueryPerformanceCounter`).
pub fn resolve_forwarded_export_string(emu: &mut emu::Emu, forwarder: &str) -> u64 {
    resolve_forwarded_export_string_depth(emu, forwarder, 0)
}

pub(crate) fn resolve_forwarded_export_string_depth(
    emu: &mut emu::Emu,
    forwarder: &str,
    inner_depth: u32,
) -> u64 {
    if inner_depth > FORWARDER_MAX_DEPTH {
        return 0;
    }
    let forwarder = forwarder.trim();
    let Some(dot) = forwarder.find('.') else {
        return 0;
    };
    let dll_part = forwarder[..dot].trim();
    let sym_part = forwarder[dot + 1..].trim();
    if dll_part.is_empty() || sym_part.is_empty() {
        return 0;
    }
    let Some(dll) = kernel32_common::normalize_library_name(dll_part, false) else {
        return 0;
    };
    // Avoid re-entering the PE loader during export resolution (can recurse from delay-load binding).
    // Forwarders should resolve against already-linked modules in the current LDR state.
    let mapped_dll = if kernel32_common::is_api_set_contract(&dll) {
        "kernelbase.dll".to_string()
    } else {
        dll
    };
    if peb64::get_module_base(&mapped_dll, emu).is_none() {
        return 0;
    }
    resolve_in_module_exports_depth(emu, &mapped_dll, sym_part, inner_depth)
}

pub fn resolve_api_name(emu: &mut emu::Emu, name: &str) -> u64 {
    let addr = emu.export_indexes.resolve_name_global(name);
    if addr != 0 {
        return addr;
    }
    let mut flink = peb64::Flink::new(emu);
    flink.load(emu);
    let first_ptr = flink.get_ptr();
    let name_lc = name.to_ascii_lowercase();
    loop {
        if flink.export_table_rva > 0 {
            for i in 0..flink.num_of_funcs {
                if flink.pe_hdr == 0 {
                    continue;
                }

                let ordinal = flink.get_function_ordinal(emu, i);
                if ordinal.func_name.to_lowercase() == name_lc {
                    return ordinal.func_va;
                }
            }
        }

        if !flink.next(emu) || flink.get_ptr() == first_ptr {
            break;
        }
    }

    0
}

pub fn search_api_name(emu: &mut emu::Emu, name: &str) -> (u64, String, String) {
    if emu.export_indexes.len() != 0 {
        for module in emu.export_indexes.iter_ordered() {
            for (export_name, ordinal_index) in &module_display_names(module) {
                if export_name.contains(name) {
                    if let Some(Some(IndexedExport::Direct { address })) =
                        module.by_ordinal.get(*ordinal_index as usize)
                    {
                        return (*address, module.module_name.clone(), export_name.clone());
                    }
                }
            }
        }
    }

    let mut flink = peb64::Flink::new(emu);
    flink.load(emu);
    let first_ptr = flink.get_ptr();

    loop {
        if flink.export_table_rva > 0 {
            for i in 0..flink.num_of_funcs {
                if flink.pe_hdr == 0 {
                    continue;
                }

                let ordinal = flink.get_function_ordinal(emu, i);
                if ordinal.func_name.contains(name) {
                    return (
                        ordinal.func_va,
                        flink.mod_name.clone(),
                        ordinal.func_name.clone(),
                    );
                }
            }
        }
        if !flink.next(emu) || flink.get_ptr() == first_ptr {
            break;
        }
    }

    (0, String::new(), String::new())
}

pub fn guess_api_name(emu: &mut emu::Emu, addr: u64) -> String {
    for module in emu.export_indexes.iter_ordered() {
        if let Some(name) = module.resolve_address(addr) {
            let lib = module
                .module_name
                .rsplit_once('.')
                .map(|(name, _)| name)
                .unwrap_or(&module.module_name);
            return format!("{}!{}", lib, name);
        }
    }

    let mut flink = peb64::Flink::new(emu);
    flink.load(emu);
    let first_ptr = flink.get_ptr();

    loop {
        if flink.export_table_rva > 0 {
            for i in 0..flink.num_of_funcs {
                if flink.pe_hdr == 0 {
                    continue;
                }

                let ordinal = flink.get_function_ordinal(emu, i);

                if ordinal.func_va == addr {
                    let lib = flink
                        .mod_name
                        .rsplit_once('.')
                        .map(|(name, _)| name)
                        .unwrap_or(&flink.mod_name);

                    return format!("{}!{}", lib, ordinal.func_name);
                }
            }
        }

        if !flink.next(emu) || flink.get_ptr() == first_ptr {
            break;
        }
    }

    String::new()
}

/// Iterate `(name, ordinal_index)` pairs for a module in registration-stable
/// order. The runtime struct stores display names privately; this helper
/// exposes them for `dump_module_iat` and `search_api_name`.
fn module_display_names(
    module: &crate::api::windows::export_index::ModuleExportIndex,
) -> Vec<(String, u32)> {
    module.display_names_for_iter()
}

/// Iterate `(display_name, resolved_va_or_zero)` pairs for a module so
/// `dump_module_iat` can log address+name without depending on private state.
fn index_module_named_list(
    module: &crate::api::windows::export_index::ModuleExportIndex,
) -> Vec<(String, u64)> {
    module.iter_for_dump()
}
