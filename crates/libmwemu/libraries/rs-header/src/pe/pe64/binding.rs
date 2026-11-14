use std::collections::HashMap;

use super::PE64;
use crate::pe::loader::PeLoader;
use crate::pe::readers::{read_c_string, read_u64_le};

const THUNK_SIZE: usize = size_of::<u64>();
const ORDINAL_FLAG: u64 = 1 << (64 - 1);
const ORDINAL_MASK: u64 = u16::MAX as u64;
const HINT_SIZE: usize = size_of::<u16>();

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

    /// Resolve delayed imports using the same thunk width and ordinal rules as the IAT.
    pub fn delay_load_binding<L: PeLoader>(&mut self, raw: &[u8], loader: &mut L, base_addr: u64) {
        let mut cache = HashMap::new();
        for index in 0..self.delay_load_dir.len() {
            let descriptor = &self.delay_load_dir[index];
            let name = descriptor.name.clone();
            let lookup = descriptor.name_table;
            let slots = descriptor.address_table;
            self.bind_imports(raw, loader, base_addr, &name, lookup, slots, &mut cache);
        }
    }

    /// Bind imports into guest memory, retaining names for unresolved thunks.
    pub fn iat_binding<L: PeLoader>(&mut self, raw: &[u8], loader: &mut L, base_addr: u64) {
        log::trace!(
            "IAT binding started, {} import descriptors",
            self.image_import_descriptor.len()
        );
        let mut cache = HashMap::new();
        for index in 0..self.image_import_descriptor.len() {
            let descriptor = &self.image_import_descriptor[index];
            let name = descriptor.name.clone();
            let slots = descriptor.first_thunk;
            let lookup = if descriptor.original_first_thunk == 0 {
                slots
            } else {
                descriptor.original_first_thunk
            };
            self.bind_imports(raw, loader, base_addr, &name, lookup, slots, &mut cache);
        }
    }

    fn bind_imports<L: PeLoader>(
        &mut self,
        raw: &[u8],
        loader: &mut L,
        base_addr: u64,
        module: &str,
        mut lookup: u32,
        mut slot: u32,
        cache: &mut HashMap<String, u64>,
    ) {
        if module.is_empty() || lookup == 0 || slot == 0 {
            return;
        }
        if loader.load_library(module) == 0 && !is_api_set_contract(module) {
            log::debug!("cannot import library {} (IAT binding skips it)", module);
            return;
        }
        let mut unresolved = 0;
        loop {
            let Some(data) = self
                .image_layout()
                .data_at(raw, lookup)
                .and_then(|data| data.get(..THUNK_SIZE))
            else {
                break;
            };
            let thunk = read_u64_le(data, 0);
            if thunk == 0 {
                break;
            }
            let Some(next_lookup) = lookup.checked_add(THUNK_SIZE as u32) else {
                break;
            };
            let Some(next_slot) = slot.checked_add(THUNK_SIZE as u32) else {
                break;
            };
            if next_slot as u64 > self.image_layout().size {
                break;
            }
            let Some(address) = (base_addr).checked_add(slot as u64) else {
                break;
            };
            let Some(last) = address.checked_add((THUNK_SIZE - 1) as u64) else {
                break;
            };
            if !loader.is_mapped(address) || !loader.is_mapped(last) {
                break;
            }
            lookup = next_lookup;
            slot = next_slot;

            let ordinal = (thunk & ORDINAL_FLAG != 0).then_some((thunk & ORDINAL_MASK) as u16);
            let name = if let Some(ordinal) = ordinal {
                format!("#{ordinal}")
            } else {
                let Ok(rva) = u32::try_from(thunk) else {
                    continue;
                };
                let Some(data) = self
                    .image_layout()
                    .data_at(raw, rva)
                    .and_then(|data| data.get(HINT_SIZE..))
                else {
                    continue;
                };
                let name = read_c_string(data, 0);
                if name.is_empty() {
                    continue;
                }
                name
            };
            let key = format!("{}!{}", module.to_lowercase(), name.to_lowercase());
            let resolved = *cache.entry(key).or_insert_with(|| {
                if let Some(ordinal) = ordinal {
                    loader.resolve_api_ordinal_in_module(module, ordinal)
                } else {
                    loader.resolve_api_name_in_module(module, &name)
                }
            });
            if resolved != 0 {
                loader.write_qword(address, resolved);
                self.iat_names.insert(resolved, format!("{module}!{name}"));
            } else {
                self.iat_names.insert(thunk, format!("{module}!{name}"));
                unresolved += 1;
                if !is_api_set_contract(module) {
                    log::trace!(
                        "unresolved import {}!{} (slot 0x{:x})",
                        module,
                        name,
                        address
                    );
                }
            }
        }
        if unresolved > 0 && !is_api_set_contract(module) {
            log::debug!("{} unresolved imports from {}", unresolved, module);
        }
    }

    /// Resolve a bound or unresolved import address to its function name.
    pub fn import_addr_to_name(&self, paddr: u64) -> String {
        self.iat_names
            .get(&paddr)
            .and_then(|s| s.split_once('!'))
            .map(|(_, name)| name.to_string())
            .unwrap_or_default()
    }

    /// Resolve a bound or unresolved import address to its module and name.
    pub fn import_addr_to_dll_and_name(&self, paddr: u64) -> String {
        self.iat_names.get(&paddr).cloned().unwrap_or_default()
    }
}

fn is_api_set_contract(module: &str) -> bool {
    let name = module.trim().to_ascii_lowercase();
    name.starts_with("api-ms-win-") || name.starts_with("ext-ms-")
}
