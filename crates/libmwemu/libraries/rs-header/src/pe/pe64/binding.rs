use std::collections::HashMap;

use super::PE64;
use crate::pe::loader::PeLoader;
use crate::pe::pe32::HintNameItem;
use crate::pe::readers::read_u64_le as read_u64_le_shared;

macro_rules! read_u64_le {
    ($raw:expr, $off:expr) => {
        read_u64_le_shared(($raw).as_ref(), $off)
    };
}

/// API-set contract DLL names (`api-ms-win-*`, `ext-ms-*`) are virtual: they
/// resolve through backing DLLs rather than a real file, so a missing file is
/// not a reason to skip the import group.
fn is_api_set_contract(module: &str) -> bool {
    let m = module.trim().to_ascii_lowercase();
    m.starts_with("api-ms-win-") || m.starts_with("ext-ms-")
}

/// PE64 IMAGE_THUNK_DATA64 high bit. When set, the low 16 bits encode an
/// export ordinal rather than an RVA pointing at IMAGE_IMPORT_BY_NAME.
const IMAGE_ORDINAL_FLAG64: u64 = 0x8000_0000_0000_0000;

/// Mask isolating the export ordinal from a 64-bit import thunk.
const IMAGE_ORDINAL_MASK64: u64 = 0x0000_0000_0000_FFFF;
impl PE64 {
    /// DLL names this image imports from (apiset contracts collapsed to kernelbase).
    pub fn get_dependencies(&self) -> Vec<String> {
        let mut dependencies: Vec<String> = Vec::new();
        for iim in &self.image_import_descriptor {
            if iim.name.is_empty() {
                continue;
            }
            let mut libname = iim.name.clone();
            if iim.name.starts_with("api-ms-win-") {
                libname = "kernelbase".to_string();
            }
            dependencies.push(libname);
        }
        dependencies
    }

    /// Bind the delay-load import table: resolve each function and patch its
    /// slot in guest memory via `loader`. `raw` is the (caller-owned) file image.
    pub fn delay_load_binding<L: PeLoader>(&mut self, raw: &[u8], loader: &mut L, base_addr: u64) {
        let mut resolved_cache: HashMap<String, u64> = HashMap::new();

        for i in 0..self.delay_load_dir.len() {
            let name = self.delay_load_dir[i].name.clone();
            if name.is_empty() {
                continue;
            }
            let name_table = self.delay_load_dir[i].name_table;
            let address_table = self.delay_load_dir[i].address_table;

            let mut off_name = PE64::vaddr_to_off(&self.sect_hdr, name_table) as usize;
            let mut off_addr = PE64::vaddr_to_off(&self.sect_hdr, address_table) as usize;
            // RVA of the current Delay-IAT slot — the location to patch (advances
            // in lock-step with off_addr).
            let mut slot_rva = address_table;

            loop {
                if raw.len() <= off_name + 4 || raw.len() <= off_addr + 4 {
                    break;
                }

                let hint = HintNameItem::load(raw, off_name);
                let off2 = PE64::vaddr_to_off(&self.sect_hdr, hint.func_name_addr) as usize;
                if off2 == 0 {
                    off_name += HintNameItem::size();
                    off_addr += 8;
                    slot_rva += 8;
                    continue;
                }

                let func_name = PE64::read_string(raw, off2 + 2);
                let cache_key = format!("{}!{}", name.to_lowercase(), func_name.to_lowercase());
                let real_addr = if let Some(cached) = resolved_cache.get(&cache_key) {
                    *cached
                } else {
                    let resolved = loader.resolve_api_name_in_module(&name, &func_name);
                    resolved_cache.insert(cache_key, resolved);
                    resolved
                };
                if real_addr == 0 {
                    break;
                }

                let patch_addr = base_addr + slot_rva as u64;
                loader.write_qword(patch_addr, real_addr);
                self.iat_names
                    .insert(real_addr, format!("{}!{}", name, func_name));

                off_name += HintNameItem::size();
                off_addr += 8;
                slot_rva += 8;
            }
        }
    }

    /// Bind the import address table: resolve each import and patch its slot in
    /// guest memory via `loader`.
    pub fn iat_binding<L: PeLoader>(&mut self, raw: &[u8], loader: &mut L, base_addr: u64) {
        log::trace!(
            "IAT binding started, {} import descriptors",
            self.image_import_descriptor.len()
        );

        let mut resolved_cache: HashMap<String, u64> = HashMap::new();

        for i in 0..self.image_import_descriptor.len() {
            let import_dll = self.image_import_descriptor[i].name.clone();
            if import_dll.is_empty() {
                continue;
            }
            let original_first_thunk = self.image_import_descriptor[i].original_first_thunk;
            let first_thunk = self.image_import_descriptor[i].first_thunk;

            // API-set contract DLLs are virtual names: even if the stub file is
            // absent, their functions still resolve via the backing DLLs below, so
            // don't skip them.
            if loader.load_library(&import_dll) == 0 && !is_api_set_contract(&import_dll) {
                log::debug!(
                    "cannot import library `{}` (IAT binding skips it)",
                    import_dll
                );
                continue;
            }

            if original_first_thunk == 0 {
                self.iat_binding_alternative(
                    raw,
                    loader,
                    base_addr,
                    first_thunk,
                    &import_dll,
                    &mut resolved_cache,
                );
            } else {
                self.iat_binding_original(
                    raw,
                    loader,
                    base_addr,
                    original_first_thunk,
                    first_thunk,
                    &import_dll,
                    &mut resolved_cache,
                );
            }
        }
    }

    fn iat_binding_alternative<L: PeLoader>(
        &mut self,
        raw: &[u8],
        loader: &mut L,
        base_addr: u64,
        first_thunk: u32,
        import_dll: &str,
        resolved_cache: &mut HashMap<String, u64>,
    ) {
        let mut rva = first_thunk;
        let mut unresolved = 0u32;

        loop {
            // Defensive RVA arithmetic: avoid the historical off+8 overflow on
            // the last fragment of a section.
            let Some(next_rva) = rva.checked_add(8) else {
                break;
            };
            let off = PE64::vaddr_to_off(&self.sect_hdr, rva) as usize;
            if off == 0 || raw.len() < off + 8 {
                break;
            }

            let func_name_addr_or_ordinal = read_u64_le!(raw, off);
            if func_name_addr_or_ordinal == 0 {
                break;
            }

            let is_ordinal = (func_name_addr_or_ordinal & IMAGE_ORDINAL_FLAG64) != 0;
            if is_ordinal {
                let ordinal = (func_name_addr_or_ordinal & IMAGE_ORDINAL_MASK64) as u16;
                self.bind_ordinal_thunk(
                    loader,
                    base_addr,
                    rva,
                    import_dll,
                    ordinal,
                    func_name_addr_or_ordinal,
                    resolved_cache,
                    &mut unresolved,
                );
                rva = next_rva;
                continue;
            }

            let func_name_addr = (func_name_addr_or_ordinal & 0x7fff_ffff_ffff_ffff) as u32;
            let off_name = PE64::vaddr_to_off(&self.sect_hdr, func_name_addr) as usize;
            // Invalid/zero RVA or truncated hint entry -> skip the entry but
            // keep binding the table. The hint entry is `u16 hint + u8[] name`,
            // so we need at least 2 bytes of name after the hint.
            if off_name == 0 || raw.len() < off_name + 2 {
                rva = next_rva;
                continue;
            }
            let api_name = PE64::read_string(raw, off_name + 2);

            let cache_key = format!("{}!{}", import_dll.to_lowercase(), api_name.to_lowercase());
            let real_addr = if let Some(cached) = resolved_cache.get(&cache_key) {
                *cached
            } else {
                let resolved = loader.resolve_api_name_in_module(import_dll, &api_name);
                resolved_cache.insert(cache_key, resolved);
                resolved
            };

            if real_addr > 0 {
                let patch_addr = base_addr + rva as u64;
                loader.write_qword(patch_addr, real_addr);
                self.iat_names
                    .insert(real_addr, format!("{}!{}", import_dll, api_name));
            } else {
                // Unresolved: the IAT slot keeps its on-disk value (the
                // name-entry address). Record it -> name so a call through the
                // slot is still identified and emulated by name.
                self.iat_names.insert(
                    func_name_addr_or_ordinal,
                    format!("{}!{}", import_dll, api_name),
                );
                unresolved += 1;
                // api-set contracts (api-ms-win-*, ext-ms-*) are name-forwarders
                // resolved by the gateway, not the IAT — expected "unresolved", so
                // don't log them (they'd flood the output).
                if !is_api_set_contract(import_dll) {
                    log::trace!(
                        "unresolved import {}!{} (IAT rva 0x{:x})",
                        import_dll,
                        api_name,
                        rva
                    );
                }
            }

            rva = next_rva;
        }

        if unresolved > 0 && !is_api_set_contract(import_dll) {
            log::debug!("{} unresolved imports from {}", unresolved, import_dll);
        }
    }

    /// Resolve and patch a single 64-bit import thunk that encodes an export
    /// ordinal. Shared by the two IAT walkers so caching, naming, and
    /// unresolved bookkeeping stay identical. The original encoded thunk
    /// (with the high ordinal flag set) is the lookup key for unresolved
    /// entries so a later call through the slot is still named correctly.
    fn bind_ordinal_thunk<L: PeLoader>(
        &mut self,
        loader: &mut L,
        base_addr: u64,
        rva: u32,
        import_dll: &str,
        ordinal: u16,
        encoded_thunk: u64,
        resolved_cache: &mut HashMap<String, u64>,
        unresolved: &mut u32,
    ) {
        let cache_key = format!("{}!#{}", import_dll, ordinal);
        let real_addr = if let Some(cached) = resolved_cache.get(&cache_key) {
            *cached
        } else {
            let resolved = loader.resolve_api_ordinal_in_module(import_dll, ordinal);
            resolved_cache.insert(cache_key, resolved);
            resolved
        };

        if real_addr != 0 {
            let patch_addr = base_addr + rva as u64;
            loader.write_qword(patch_addr, real_addr);
            self.iat_names
                .insert(real_addr, format!("{}!#{}", import_dll, ordinal));
        } else {
            // Unresolved: the slot keeps its on-disk encoded thunk so a later
            // call through it can still be named. Track the unresolved
            // count and suppress the per-import log line for API-set
            // contracts (their imports always flow through the gateway).
            self.iat_names
                .insert(encoded_thunk, format!("{}!#{}", import_dll, ordinal));
            *unresolved += 1;
            if !is_api_set_contract(import_dll) {
                log::trace!(
                    "unresolved ordinal import {}!#{} (IAT rva 0x{:x})",
                    import_dll,
                    ordinal,
                    rva
                );
            }
        }
    }

    fn iat_binding_original<L: PeLoader>(
        &mut self,
        raw: &[u8],
        loader: &mut L,
        base_addr: u64,
        original_first_thunk: u32,
        first_thunk: u32,
        import_dll: &str,
        resolved_cache: &mut HashMap<String, u64>,
    ) {
        let mut off_name = PE64::vaddr_to_off(&self.sect_hdr, original_first_thunk) as usize;
        let mut off_addr = PE64::vaddr_to_off(&self.sect_hdr, first_thunk) as usize;
        let mut rva = first_thunk;
        let mut unresolved = 0u32;

        // Invalid/zero raw offset for either table means the RVA points
        // outside any mapped section. Stop this descriptor rather than
        // reading the file header as a thunk table.
        if off_name == 0 || off_addr == 0 {
            return;
        }

        loop {
            // Defensive bounds: both lookup and destination tables must hold a
            // full 8-byte entry before we read or advance. Plus checked
            // advances for every table index so a malformed input cannot
            // wrap `rva`/`off_name`/`off_addr` into garbage.
            let Some(next_off_name) = off_name.checked_add(8) else {
                break;
            };
            let Some(next_off_addr) = off_addr.checked_add(8) else {
                break;
            };
            let Some(next_rva) = rva.checked_add(8) else {
                break;
            };
            if raw.len() < next_off_name || raw.len() < next_off_addr {
                break;
            }

            let thunk_data = read_u64_le!(raw, off_name);
            if thunk_data == 0 {
                break;
            }

            let is_ordinal = (thunk_data & IMAGE_ORDINAL_FLAG64) != 0;
            if is_ordinal {
                let ordinal = (thunk_data & IMAGE_ORDINAL_MASK64) as u16;
                self.bind_ordinal_thunk(
                    loader,
                    base_addr,
                    rva,
                    import_dll,
                    ordinal,
                    thunk_data,
                    resolved_cache,
                    &mut unresolved,
                );
                off_name = next_off_name;
                off_addr = next_off_addr;
                rva = next_rva;
                continue;
            }

            let func_name_addr = (thunk_data & 0x7fff_ffff_ffff_ffff) as u32;
            let off2 = PE64::vaddr_to_off(&self.sect_hdr, func_name_addr) as usize;
            if off2 == 0 {
                off_name = next_off_name;
                off_addr = next_off_addr;
                rva = next_rva;
                continue;
            }
            let func_name = PE64::read_string(raw, off2 + 2);
            let cache_key = format!("{}!{}", import_dll.to_lowercase(), func_name.to_lowercase());
            let real_addr = if let Some(cached) = resolved_cache.get(&cache_key) {
                *cached
            } else {
                let resolved = loader.resolve_api_name_in_module(import_dll, &func_name);
                resolved_cache.insert(cache_key, resolved);
                resolved
            };

            if real_addr != 0 {
                let patch_addr = base_addr + rva as u64;
                loader.write_qword(patch_addr, real_addr);
                self.iat_names
                    .insert(real_addr, format!("{}!{}", import_dll, func_name));
            } else {
                // Unresolved: the slot keeps its on-disk thunk value; record it
                // -> name so a call through the slot is still named/emulated.
                self.iat_names
                    .insert(thunk_data, format!("{}!{}", import_dll, func_name));
                unresolved += 1;
                if !is_api_set_contract(import_dll) {
                    log::trace!(
                        "unresolved import {}!{} (IAT rva 0x{:x})",
                        import_dll,
                        func_name,
                        rva
                    );
                }
            }

            off_name = next_off_name;
            off_addr = next_off_addr;
            rva = next_rva;
        }

        if unresolved > 0 && !is_api_set_contract(import_dll) {
            log::debug!("{} unresolved imports from {}", unresolved, import_dll);
        }
    }


    /// Map a resolved (post-binding) import address back to its function name.
    /// O(1) lookup against the table built during binding — no file bytes needed.
    pub fn import_addr_to_name(&self, paddr: u64) -> String {
        self.iat_names
            .get(&paddr)
            .and_then(|s| s.split_once('!'))
            .map(|(_, name)| name.to_string())
            .unwrap_or_default()
    }

    /// Like [`import_addr_to_name`] but returns `"dll!name"`.
    pub fn import_addr_to_dll_and_name(&self, paddr: u64) -> String {
        self.iat_names.get(&paddr).cloned().unwrap_or_default()
    }
}
