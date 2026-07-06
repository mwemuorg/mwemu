use crate::pe::loader::PeLoader;
use super::{HintNameItem, PE32};

impl PE32 {
    /// Bind the delay-load import table into guest memory via `loader`.
    pub fn delay_load_binding<L: PeLoader>(&mut self, raw: &[u8], loader: &mut L, base_addr: u32) {
        for i in 0..self.delay_load_dir.len() {
            let name = self.delay_load_dir[i].name.clone();
            if name.is_empty() {
                continue;
            }
            let name_table = self.delay_load_dir[i].name_table;
            let bound_delay_import_table = self.delay_load_dir[i].bound_delay_import_table;

            if loader.load_library(&name) == 0 {
                log::warn!("cannot find delay-load library `{}` (skipping)", name);
                continue;
            }

            let mut off_name = PE32::vaddr_to_off(&self.sect_hdr, name_table) as usize;
            let mut rva = bound_delay_import_table;

            loop {
                if raw.len() <= off_name + 4 {
                    break;
                }

                let thunk = HintNameItem::load(raw, off_name).func_name_addr;
                // Null thunk terminates the name table.
                if thunk == 0 {
                    break;
                }
                // Ordinal import (high bit set): no name to resolve — skip it
                // instead of aborting the whole table.
                if thunk & 0x8000_0000 != 0 {
                    off_name += HintNameItem::size();
                    rva += 4;
                    continue;
                }
                let off2 = PE32::vaddr_to_off(&self.sect_hdr, thunk) as usize;
                if off2 == 0 {
                    off_name += HintNameItem::size();
                    rva += 4;
                    continue;
                }
                let func_name = PE32::read_string(raw, off2 + 2);
                let real_addr = loader.resolve_api_name(&func_name);
                // An unresolved import must not abort binding of the rest of the
                // table — leave its slot and keep going.
                if real_addr != 0 {
                    let patch_addr = base_addr as u64 + rva as u64;
                    loader.write_dword(patch_addr, real_addr as u32);
                    self.iat_names
                        .insert(real_addr as u32, format!("{}!{}", name, func_name));
                }

                off_name += HintNameItem::size();
                rva += 4;
            }
        }
    }

    /// Bind the import address table into guest memory via `loader`.
    pub fn iat_binding<L: PeLoader>(&mut self, raw: &[u8], loader: &mut L, base_addr: u32) {
        log::debug!(
            "IAT binding started, {} import descriptors",
            self.image_import_descriptor.len()
        );

        for i in 0..self.image_import_descriptor.len() {
            let iim_name = self.image_import_descriptor[i].name.clone();
            if iim_name.is_empty() {
                continue;
            }
            let original_first_thunk = self.image_import_descriptor[i].original_first_thunk;
            let first_thunk = self.image_import_descriptor[i].first_thunk;

            if loader.load_library(&iim_name) == 0 {
                log::debug!("cannot find library `{}` (IAT binding skips it)", iim_name);
                continue;
            }

            // Walk the name list: OriginalFirstThunk when present, else the IAT
            // itself (some linkers leave OriginalFirstThunk null).
            let walk_thunk = if original_first_thunk != 0 {
                original_first_thunk
            } else {
                first_thunk
            };
            let mut off_name = PE32::vaddr_to_off(&self.sect_hdr, walk_thunk) as usize;
            let mut rva = first_thunk;

            loop {
                if raw.len() <= off_name + 4 {
                    break;
                }
                let thunk = HintNameItem::load(raw, off_name).func_name_addr;
                // Null thunk terminates the import list for this DLL.
                if thunk == 0 {
                    break;
                }
                // Ordinal import (high bit set): skip, keep binding the rest.
                if thunk & 0x8000_0000 != 0 {
                    off_name += HintNameItem::size();
                    rva += 4;
                    continue;
                }
                let off2 = PE32::vaddr_to_off(&self.sect_hdr, thunk) as usize;
                if off2 == 0 {
                    off_name += HintNameItem::size();
                    rva += 4;
                    continue;
                }
                let func_name = PE32::read_string(raw, off2 + 2);
                // Resolve in the named module first; fall back to a global search.
                // API-set contract DLLs (`api-ms-win-crt-*`) are virtual names
                // with no real export table — their functions live in the backing
                // CRT (ucrtbase/msvcrt), so only the global lookup finds them.
                let mut real_addr = loader.resolve_api_name_in_module(&iim_name, &func_name);
                if real_addr == 0 {
                    real_addr = loader.resolve_api_name(&func_name);
                }
                // Do NOT abort the table on an unresolved import; leave the slot
                // and continue so later (resolvable) imports still get bound.
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
                    log::trace!(
                        "unresolved import {}!{} (IAT rva 0x{:x}); named by slot 0x{:x}",
                        iim_name, func_name, rva, thunk
                    );
                }

                off_name += HintNameItem::size();
                rva += 4;
            }
        }
    }

    /// Map a resolved import address back to its function name (O(1) lookup
    /// against the table built during binding — no file bytes needed).
    pub fn import_addr_to_name(&self, paddr: u32) -> String {
        self.iat_names
            .get(&paddr)
            .and_then(|s| s.split_once('!'))
            .map(|(_, name)| name.to_string())
            .unwrap_or_default()
    }
}
