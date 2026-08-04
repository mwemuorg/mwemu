use std::collections::HashMap;

use super::{HintNameItem, PE32};
use crate::pe::loader::PeLoader;
use crate::pe::readers::read_u32_le as read_u32_le_shared;

macro_rules! read_u32_le {
    ($raw:expr, $off:expr) => {
        read_u32_le_shared(($raw).as_ref(), $off)
    };
}

/// PE32 IMAGE_THUNK_DATA32 high-bit discriminator.
const IMAGE_ORDINAL_FLAG32: u32 = 0x8000_0000;

/// Mask isolating the import ordinal from a 32-bit thunk.
const IMAGE_ORDINAL_MASK32: u32 = 0x0000_FFFF;

impl PE32 {
    /// Bind the delay-load import table into guest memory via `loader`.
    pub fn delay_load_binding<L: PeLoader>(&mut self, raw: &[u8], loader: &mut L, base_addr: u32) {
        let mut resolved_cache: HashMap<String, u64> = HashMap::new();

        for i in 0..self.delay_load_dir.len() {
            let name = self.delay_load_dir[i].name.clone();
            if name.is_empty() {
                continue;
            }
            let name_table = self.delay_load_dir[i].name_table;
            let address_table = self.delay_load_dir[i].address_table;

            if loader.load_library(&name) == 0 && !is_api_set_contract(&name) {
                log::warn!("cannot find delay-load library `{}` (skipping)", name);
                continue;
            }

            if PE32::vaddr_to_off(&self.sect_hdr, name_table) == 0
                || PE32::vaddr_to_off(&self.sect_hdr, address_table) == 0
            {
                continue;
            }
            let mut off_name = PE32::vaddr_to_off(&self.sect_hdr, name_table) as usize;
            let mut rva = address_table;
            let mut unresolved = 0u32;

            loop {
                let Some(next_off_name) = off_name.checked_add(HintNameItem::size()) else {
                    break;
                };
                let Some(next_rva) = rva.checked_add(4) else {
                    break;
                };
                if raw.len() < next_off_name {
                    break;
                }

                let thunk = read_u32_le!(raw, off_name);
                if thunk == 0 {
                    break;
                }
                let is_ordinal = (thunk & IMAGE_ORDINAL_FLAG32) != 0;
                if is_ordinal {
                    let ordinal = (thunk & IMAGE_ORDINAL_MASK32) as u16;
                    self.bind_ordinal_thunk(
                        loader,
                        base_addr,
                        rva,
                        &name,
                        ordinal,
                        thunk,
                        &mut resolved_cache,
                        &mut unresolved,
                    );
                    off_name = next_off_name;
                    rva = next_rva;
                    continue;
                }
                let off2 = PE32::vaddr_to_off(&self.sect_hdr, thunk) as usize;
                // use checked arithmetic so an overflow does not panic and a
                // truncated hint entry is skipped without reading past `raw`.
                let Some(name_end) = off2.checked_add(2) else {
                    off_name = next_off_name;
                    rva = next_rva;
                    continue;
                };
                if off2 == 0 || raw.len() < name_end {
                    off_name = next_off_name;
                    rva = next_rva;
                    continue;
                }
                let func_name = PE32::read_string(raw, off2 + 2);
                let cache_key = format!("{}!{}", name.to_lowercase(), func_name.to_lowercase());
                let real_addr = if let Some(cached) = resolved_cache.get(&cache_key) {
                    *cached
                } else {
                    let mut resolved = loader.resolve_api_name_in_module(&name, &func_name);
                    if resolved == 0 {
                        // API-set contract DLLs are virtual names with no real
                        // export table; their functions live in the backing
                        // CRT, so only the global lookup finds them.
                        resolved = loader.resolve_api_name(&func_name);
                    }
                    resolved_cache.insert(cache_key, resolved);
                    resolved
                };
                if real_addr != 0 {
                    let patch_addr = base_addr as u64 + rva as u64;
                    loader.write_dword(patch_addr, real_addr as u32);
                    self.iat_names
                        .insert(real_addr as u32, format!("{}!{}", name, func_name));
                } else {
                    self.iat_names
                        .insert(thunk, format!("{}!{}", name, func_name));
                    unresolved += 1;
                    if !is_api_set_contract(&name) {
                        log::trace!(
                            "unresolved delay import {}!{} (slot 0x{:x}); named by thunk 0x{:x}",
                            name,
                            func_name,
                            rva,
                            thunk
                        );
                    }
                }
                off_name = next_off_name;
                rva = next_rva;
            }

            if unresolved > 0 && !is_api_set_contract(&name) {
                log::debug!("{} unresolved delay imports from {}", unresolved, name);
            }
        }
    }
    /// Bind the import address table into guest memory via `loader`.
    pub fn iat_binding<L: PeLoader>(&mut self, raw: &[u8], loader: &mut L, base_addr: u32) {
        log::trace!(
            "IAT binding started, {} import descriptors",
            self.image_import_descriptor.len()
        );

        let mut resolved_cache: HashMap<String, u64> = HashMap::new();

        for i in 0..self.image_import_descriptor.len() {
            let iim_name = self.image_import_descriptor[i].name.clone();
            if iim_name.is_empty() {
                continue;
            }
            let original_first_thunk = self.image_import_descriptor[i].original_first_thunk;
            let first_thunk = self.image_import_descriptor[i].first_thunk;

            // API-set contract DLLs are virtual names: their functions resolve
            // via the backing DLLs (api-ms-win-* -> kernelbase.dll etc.), so
            // a missing stub file is not a reason to skip the descriptor.
            if loader.load_library(&iim_name) == 0 && !is_api_set_contract(&iim_name) {
                log::debug!(
                    "cannot import library `{}` (IAT binding skips it)",
                    iim_name
                );
                continue;
            }

            // Defensive: a zero raw offset for either table means the RVA
            // points outside any mapped section. Skip this descriptor instead
            // of reading the file header as a thunk table.
            if PE32::vaddr_to_off(&self.sect_hdr, first_thunk) == 0 {
                continue;
            }
            let walk_thunk = if original_first_thunk != 0 {
                original_first_thunk
            } else {
                first_thunk
            };
            if PE32::vaddr_to_off(&self.sect_hdr, walk_thunk) == 0 {
                continue;
            }

            let mut off_name = PE32::vaddr_to_off(&self.sect_hdr, walk_thunk) as usize;
            let mut rva = first_thunk;
            let mut unresolved = 0u32;

            loop {
                // Defensive RVA arithmetic: avoid historical off+4 overflow on
                // the last fragment of a section.
                let Some(next_off_name) = off_name.checked_add(HintNameItem::size()) else {
                    break;
                };
                let Some(next_rva) = rva.checked_add(4) else {
                    break;
                };
                if raw.len() < next_off_name {
                    break;
                }

                let thunk = read_u32_le!(raw, off_name);
                // Null thunk terminates the import list for this DLL.
                if thunk == 0 {
                    break;
                }
                let is_ordinal = (thunk & IMAGE_ORDINAL_FLAG32) != 0;
                if is_ordinal {
                    let ordinal = (thunk & IMAGE_ORDINAL_MASK32) as u16;
                    self.bind_ordinal_thunk(
                        loader,
                        base_addr,
                        rva,
                        &iim_name,
                        ordinal,
                        thunk,
                        &mut resolved_cache,
                        &mut unresolved,
                    );
                    off_name = next_off_name;
                    rva = next_rva;
                    continue;
                }
                let off2 = PE32::vaddr_to_off(&self.sect_hdr, thunk) as usize;
                // Malformed thunk RVAs can produce out-of-range raw offsets;
                // use checked arithmetic so an overflow does not panic and a
                // truncated hint entry is skipped without reading past `raw`.
                let Some(name_end) = off2.checked_add(2) else {
                    off_name = next_off_name;
                    rva = next_rva;
                    continue;
                };
                if off2 == 0 || raw.len() < name_end {
                    off_name = next_off_name;
                    rva = next_rva;
                    continue;
                }
                let func_name = PE32::read_string(raw, off2 + 2);
                let cache_key = format!("{}!{}", iim_name.to_lowercase(), func_name.to_lowercase());
                let real_addr = if let Some(cached) = resolved_cache.get(&cache_key) {
                    *cached
                } else {
                    let mut resolved = loader.resolve_api_name_in_module(&iim_name, &func_name);
                    if resolved == 0 {
                        // API-set contract DLLs are virtual names with no real
                        // export table; their functions live in the backing
                        // CRT, so only the global lookup finds them.
                        resolved = loader.resolve_api_name(&func_name);
                    }
                    resolved_cache.insert(cache_key, resolved);
                    resolved
                };
                if real_addr != 0 {
                    let patch_addr = base_addr as u64 + rva as u64;
                    loader.write_dword(patch_addr, real_addr as u32);
                    self.iat_names
                        .insert(real_addr as u32, format!("{}!{}", iim_name, func_name));
                } else {
                    // Unresolved: the IAT slot keeps its on-disk value (the
                    // name-entry RVA, `thunk`). Record that value -> name so a
                    // later call through the slot can still be identified and
                    // emulated by name (import_addr_to_name is the only runtime
                    // hook now that the file bytes are not kept around).
                    self.iat_names
                        .insert(thunk, format!("{}!{}", iim_name, func_name));
                    unresolved += 1;
                    if !is_api_set_contract(&iim_name) {
                        log::trace!(
                            "unresolved import {}!{} (IAT rva 0x{:x}); named by slot 0x{:x}",
                            iim_name,
                            func_name,
                            rva,
                            thunk
                        );
                    }
                }

                off_name = next_off_name;
                rva = next_rva;
            }

            if unresolved > 0 && !is_api_set_contract(&iim_name) {
                log::debug!("{} unresolved imports from {}", unresolved, iim_name);
            }
        }
    }

    /// Map a resolved import address back to its function name (O(1) lookup
    pub fn import_addr_to_name(&self, paddr: u32) -> String {
        self.iat_names
            .get(&paddr)
            .and_then(|s| s.split_once('!'))
            .map(|(_, name)| name.to_string())
            .unwrap_or_default()
    }
    /// Like [`import_addr_to_name`] but returns `"dll!name"`.
    pub fn import_addr_to_dll_and_name(&self, paddr: u32) -> String {
        self.iat_names.get(&paddr).cloned().unwrap_or_default()
    }
    /// Resolve and patch a single 32-bit import thunk that encodes an export
    /// ordinal. Shared by the IAT and delay-load walkers so caching, naming,
    /// and unresolved bookkeeping stay identical. The original encoded thunk
    /// (with the high ordinal flag set) is the lookup key for unresolved
    /// entries so a later call through the slot is still named correctly.
    fn bind_ordinal_thunk<L: PeLoader>(
        &mut self,
        loader: &mut L,
        base_addr: u32,
        rva: u32,
        import_dll: &str,
        ordinal: u16,
        encoded_thunk: u32,
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
            let patch_addr = base_addr as u64 + rva as u64;
            loader.write_dword(patch_addr, real_addr as u32);
            self.iat_names
                .insert(real_addr as u32, format!("{}!#{}", import_dll, ordinal));
        } else {
            // Unresolved: the slot keeps its on-disk encoded thunk so a later
            // call through it can still be named. Track the unresolved count
            // and suppress the per-import log line for API-set contracts.
            self.iat_names
                .insert(encoded_thunk, format!("{}!#{}", import_dll, ordinal));
            *unresolved += 1;
            if !is_api_set_contract(import_dll) {
                log::trace!(
                    "unresolved ordinal import {}!#{} (slot rva 0x{:x})",
                    import_dll,
                    ordinal,
                    rva
                );
            }
        }
    }
}

/// API-set contract DLL names (`api-ms-win-*`, `ext-ms-*`) are virtual: they
/// resolve through backing DLLs rather than a real file, so a missing file is
/// not a reason to skip the import group.
fn is_api_set_contract(module: &str) -> bool {
    let m = module.trim().to_ascii_lowercase();
    m.starts_with("api-ms-win-") || m.starts_with("ext-ms-")
}
