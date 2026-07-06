use crate::api::windows::common::kernel32 as kernel32_common;
use crate::emu;
use crate::windows::peb::peb32;

pub fn dump_module_iat(emu: &mut emu::Emu, module: &str) {
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
        return resolve_api_name(emu, name);
    }

    let mut flink = peb32::Flink::new(emu);
    flink.load(emu);
    let first_ptr = flink.get_ptr();

    loop {
        if flink.mod_name.to_lowercase().contains(&module_lc)
            && flink.export_table_rva > 0 {
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
                if ordinal.func_va == addr as u64 {
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
