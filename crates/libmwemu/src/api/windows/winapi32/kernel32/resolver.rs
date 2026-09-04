use crate::api::windows::common::kernel32 as kernel32_common;
use crate::api::windows::export_index::IndexedExport;
use crate::emu;
use crate::windows::peb::peb32;

pub fn dump_module_iat(emu: &mut emu::Emu, module: &str) {
    let needle = module.to_ascii_lowercase();
    if emu.export_indexes.len() != 0 {
        for index in emu.export_indexes.iter_ordered() {
            if !index.normalized_name.contains(&needle) {
                continue;
            }
            log::trace!("---- exports of {} ----", index.module_name);
            for (name, va) in index.iter_for_dump() {
                log::trace!("0x{:x} {}!{}", va, index.module_name, name);
            }
        }
        return;
    }
    let mut flink = peb32::Flink::new(emu);
    flink.load(emu);
    let first_ptr = flink.get_ptr();

    loop {
        if flink.mod_name.to_lowercase().contains(module) && flink.export_table_rva > 0 {
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
        flink.next(emu);

        if flink.get_ptr() == first_ptr {
            break;
        }
    }
}

pub fn resolve_api_name_in_module(emu: &mut emu::Emu, module: &str, name: &str) -> u64 {
    // API set DLL names (api-ms-win-*, ext-ms-*) are virtual contracts.
    // Resolve by function name globally like 64-bit path does.
    let module_lc = module.to_lowercase();
    if kernel32_common::is_api_set_contract(&module_lc) {
        let addr = emu.export_indexes.resolve_name_global(name);
        if addr != 0 {
            return addr;
        }
        return resolve_api_name(emu, name);
    }

    // Index-first (case-insensitive lookup handled by the registry).
    let addr = emu.export_indexes.resolve_name_in_module(&module_lc, name);
    if addr != 0 {
        return addr;
    }

    let mut flink = peb32::Flink::new(emu);
    flink.load(emu);
    let first_ptr = flink.get_ptr();

    loop {
        if flink.mod_name.to_lowercase().contains(&module_lc) && flink.export_table_rva > 0 {
            for i in 0..flink.num_of_funcs {
                if flink.pe_hdr == 0 {
                    continue;
                }

                let ordinal = flink.get_function_ordinal(emu, i);
                if ordinal.func_name == name {
                    return ordinal.func_va;
                }
            }
        }
        flink.next(emu);

        if flink.get_ptr() == first_ptr {
            break;
        }
    }

    0
}

pub fn resolve_api_addr_to_name(emu: &mut emu::Emu, addr: u64) -> String {
    for module in emu.export_indexes.iter_ordered() {
        if let Some(name) = module.resolve_address(addr) {
            return name.to_string();
        }
    }
    // Same guard: a miss against a populated index is authoritative; skip the
    // O(total exports) PEB walk when the registry is non-empty.
    if !emu.export_indexes.is_empty() {
        return String::new();
    }

    let mut flink = peb32::Flink::new(emu);
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
        flink.next(emu);

        if flink.get_ptr() == first_ptr {
            break;
        }
    }

    String::new()
}

pub fn resolve_api_name(emu: &mut emu::Emu, name: &str) -> u64 {
    let addr = emu.export_indexes.resolve_name_global(name);
    if addr != 0 {
        return addr;
    }
    let mut flink = peb32::Flink::new(emu);
    flink.load(emu);
    let first_ptr = flink.get_ptr();

    loop {
        if flink.export_table_rva > 0 {
            for i in 0..flink.num_of_funcs {
                if flink.pe_hdr == 0 {
                    continue;
                }

                let ordinal = flink.get_function_ordinal(emu, i);
                if ordinal.func_name == name {
                    return ordinal.func_va;
                }
            }
        }
        flink.next(emu);

        if flink.get_ptr() == first_ptr {
            break;
        }
    }

    0
}

pub fn search_api_name(emu: &mut emu::Emu, name: &str) -> (u64, String, String) {
    if emu.export_indexes.len() != 0 {
        for module in emu.export_indexes.iter_ordered() {
            for (export_name, ord_idx) in module.display_names_for_iter() {
                if export_name.contains(name) {
                    if let Some(Some(IndexedExport::Direct { address })) =
                        module.by_ordinal.get(ord_idx as usize)
                    {
                        return (*address, module.module_name.clone(), export_name);
                    }
                }
            }
        }
    }
    // Same guard as `guess_api_name`: a miss against a populated index is
    // authoritative; never pay the PEB walk when the registry is non-empty.
    if !emu.export_indexes.is_empty() {
        return (0, String::new(), String::new());
    }

    let mut flink = peb32::Flink::new(emu);
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
        flink.next(emu);

        if flink.get_ptr() == first_ptr {
            break;
        }
    }

    (0, String::new(), String::new())
}

pub fn guess_api_name(emu: &mut emu::Emu, addr: u32) -> String {
    let addr = addr as u64;
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
    // Same guard as the winapi64 sibling: a miss against a populated index is
    // authoritative; skip the O(total exports) PEB walk when any module is
    // registered (loaders register every mapped module).
    if !emu.export_indexes.is_empty() {
        return String::new();
    }

    let mut flink = peb32::Flink::new(emu);
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

        flink.next(emu);

        if flink.get_ptr() == first_ptr {
            break;
        }
    }

    String::new()
}
