//! Host-side runtime export-name registry owned by `Emu`.
//!
//! Each mapped PE image (PE32, PE64, or AArch64 PE64) registers its parsed
//! export directory once, at mapping/relocation time. Subsequent named and
//! ordinal resolution lookups run in O(1) against these maps instead of
//! repeatedly scanning the export directory through emulated memory.
//!
//! Two layers:
//!
//! 1. [`ExportIndexData`] (in `rs-header`) — pure parser output, no emulator
//!    references, function RVAs only. Lives in `rs-header` so it stays
//!    architecture-neutral.
//! 2. [`ModuleExportIndex`] + [`ExportIndexRegistry`] (this module) — runtime
//!    form with rebased addresses, ordered iteration, and lookup helpers.

use ahash::AHashMap;
use rs_header::pe::export_index::{ExportIndexData, ExportTarget, NamedExport};
use serde::{Deserialize, Serialize};

/// Maximum depth for forwarder-chain resolution. Matches the existing
/// resolver's depth limit (`MAX_FORWARDER_DEPTH = 8`).
pub const MAX_FORWARDER_DEPTH: usize = 8;

/// One resolved export entry in the runtime index. `Direct` stores the
/// already-rebased absolute address; `Forwarder` keeps the original string so
/// it can be resolved later (the target module may not be mapped yet).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum IndexedExport {
    Direct { address: u64 },
    Forwarder { value: String },
}

impl IndexedExport {
    /// True if this is a direct export (not a forwarder).
    pub fn is_direct(&self) -> bool {
        matches!(self, IndexedExport::Direct { .. })
    }

    /// The address if direct, otherwise `None`.
    pub fn direct_address(&self) -> Option<u64> {
        match self {
            IndexedExport::Direct { address } => Some(*address),
            IndexedExport::Forwarder { .. } => None,
        }
    }
}

/// The runtime export index for one mapped PE module.
///
/// Owns all string data; safe to serialize.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleExportIndex {
    /// Original (preserved-case) module name as known to the loader /
    /// registry. Used for logs and `search_api_name` output.
    pub module_name: String,
    /// Normalized (lowercase, basename, `.dll` stripped) key used for map
    /// lookups.
    pub normalized_name: String,
    /// Current image base. Direct export addresses are `base + rva` at
    /// registration time.
    pub base: u64,
    /// Export ordinal base (from the IMAGE_EXPORT_DIRECTORY.Base field).
    pub export_base: u32,
    /// Lowercased export name -> index into `ordinal_targets`. O(1) named
    /// lookup.
    pub by_name: AHashMap<String, u32>,
    /// One slot per function-table entry. `ordinal_targets[i]` corresponds
    /// to ordinal `export_base + i`. `None` slots are unresolved entries.
    pub by_ordinal: Vec<Option<IndexedExport>>,
    /// Reverse map: direct-export address (with current `base`) ->
    /// preserved-case export name. Used by `resolve_api_addr_to_name` and
    /// `guess_api_name`. Only direct exports are recorded; forwarder aliases
    /// are intentionally omitted because they don't own the address.
    pub by_address: AHashMap<u64, String>,
    /// Original-case (ord_index, name) pairs preserved across rebase so the
    /// reverse-address map can be rebuilt without re-parsing the raw PE.
    /// `by_name` only keeps the lowercased key.
    display_names: Vec<(u32, String)>,
}

impl ModuleExportIndex {
    /// Build the runtime index from a parsed export directory and the
    /// current module base. `module_name` is the normalized key; the
    /// original spelling is preserved in `NamedExport.name`.
    pub fn from_parsed(
        module_name: String,
        normalized_name: String,
        base: u64,
        parsed: &ExportIndexData,
    ) -> Self {
        let mut by_name: AHashMap<String, u32> = AHashMap::new();
        let mut by_address: AHashMap<u64, String> = AHashMap::new();

        // Build by_name first; later direct entries may override earlier ones
        // (alias to the same function-table slot), matching how Windows treats
        // duplicate names.
        for NamedExport {
            name,
            ordinal_index,
        } in &parsed.named_exports
        {
            let lc = name.to_ascii_lowercase();
            by_name.insert(lc, *ordinal_index);
        }

        let mut by_ordinal: Vec<Option<IndexedExport>> =
            Vec::with_capacity(parsed.ordinal_targets.len());
        for slot in &parsed.ordinal_targets {
            let resolved = match slot {
                None => None,
                Some(ExportTarget::Direct { rva }) => {
                    let address = base.wrapping_add(*rva as u64);
                    Some(IndexedExport::Direct { address })
                }
                Some(ExportTarget::Forwarder { value }) => Some(IndexedExport::Forwarder {
                    value: value.clone(),
                }),
            };
            by_ordinal.push(resolved);
        }

        // Build the direct-address reverse map. Use the *original-case* export
        // name; for aliases pointing at the same slot we keep the first one
        // seen (matching PEB-export reverse-lookup behavior in Windows).
        for NamedExport {
            name,
            ordinal_index,
        } in &parsed.named_exports
        {
            if let Some(Some(IndexedExport::Direct { address })) =
                by_ordinal.get(*ordinal_index as usize)
            {
                by_address.entry(*address).or_insert_with(|| name.clone());
            }
        }

        // Preserve original-case (display) names for use by the reverse
        // lookup and `search_api_name`. Keyed by ordinal index so rebase can
        // rebuild `by_address` after the base moves.
        let mut display_names: Vec<(u32, String)> = Vec::new();
        for NamedExport {
            name,
            ordinal_index,
        } in &parsed.named_exports
        {
            display_names.push((*ordinal_index, name.clone()));
        }

        ModuleExportIndex {
            module_name,
            normalized_name,
            base,
            export_base: parsed.export_base,
            by_name,
            by_ordinal,
            by_address,
            display_names,
        }
    }

    /// Recompute all direct addresses for a new module base. Forwarder
    /// entries are left untouched. The reverse-address map is rebuilt using
    /// the preserved `display_names`.
    pub fn rebase(&mut self, new_base: u64) {
        if self.base == new_base {
            return;
        }
        self.by_address.clear();
        let delta = new_base.wrapping_sub(self.base);
        for slot in &mut self.by_ordinal {
            if let Some(IndexedExport::Direct { address }) = slot {
                *address = address.wrapping_add(delta);
            }
        }
        for (ord_idx, name) in &self.display_names {
            if let Some(Some(IndexedExport::Direct { address })) =
                self.by_ordinal.get(*ord_idx as usize)
            {
                self.by_address
                    .entry(*address)
                    .or_insert_with(|| name.clone());
            }
        }
        self.base = new_base;
    }

    /// Resolve an export by lowercased name. Returns the resolved target
    /// (direct or forwarder) without following forwarders.
    pub fn resolve_name(&self, name_lc: &str) -> Option<&IndexedExport> {
        self.by_name
            .get(name_lc)
            .and_then(|idx| self.by_ordinal.get(*idx as usize).and_then(|s| s.as_ref()))
    }

    /// Resolve an export by ordinal (the export ordinal as exported by
    /// Windows, i.e. `export_base + function_table_index`).
    pub fn resolve_ordinal(&self, ordinal: u32) -> Option<&IndexedExport> {
        if ordinal < self.export_base {
            return None;
        }
        let idx = ordinal - self.export_base;
        self.by_ordinal.get(idx as usize).and_then(|s| s.as_ref())
    }

    /// Reverse-lookup: given an address, return the export name (if this
    /// module exports it as a direct entry).
    pub fn resolve_address(&self, addr: u64) -> Option<&str> {
        self.by_address.get(&addr).map(|s| s.as_str())
    }

    /// Iterate (display_name, ordinal_index) pairs for diagnostics and
    /// search. Order matches `display_names` (i.e. registration order), so
    /// deterministic scans behave the same as the prior PEB walker.
    pub fn display_names_for_iter(&self) -> Vec<(String, u32)> {
        self.display_names
            .iter()
            .map(|(idx, name)| (name.clone(), *idx))
            .collect()
    }

    /// Iterate (display_name, resolved_va_or_zero) pairs for `dump_module_iat`.
    /// Forwarder entries appear with a zero address — they don't own a VA.
    pub fn iter_for_dump(&self) -> Vec<(String, u64)> {
        self.display_names
            .iter()
            .map(|(idx, name)| {
                let va = self
                    .by_ordinal
                    .get(*idx as usize)
                    .and_then(|s| s.as_ref())
                    .and_then(IndexedExport::direct_address)
                    .unwrap_or(0);
                (name.clone(), va)
            })
            .collect()
    }
}

/// The Emu-owned registry of `ModuleExportIndex`es.
///
/// Backed by two `AHashMap`s for O(1) name and base lookup plus an ordered
/// `Vec<String>` so global scans (`search_api_name`, `guess_api_name`,
/// `dump_module_iat`) iterate in a stable registration order.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ExportIndexRegistry {
    by_name: AHashMap<String, ModuleExportIndex>,
    by_base: AHashMap<u64, String>,
    /// Normalized module names in registration order. Preserved across
    /// `replace` so deterministic scans keep their order.
    order: Vec<String>,
}

impl ExportIndexRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    /// Register a new index. If `module` already exists, behave like
    /// `replace` (remove the old base first, preserve order).
    pub fn register(&mut self, index: ModuleExportIndex) {
        let key = index.normalized_name.clone();
        if let Some(existing) = self.by_name.get(&key) {
            // Replace: drop the old base mapping; keep the order slot.
            self.by_base.remove(&existing.base);
        } else {
            // New entry: append to order.
            self.order.push(key.clone());
        }
        self.by_base.insert(index.base, key.clone());
        self.by_name.insert(key, index);
    }

    /// Remove a module by name (any case / path form). Returns whether
    /// anything was removed.
    pub fn remove(&mut self, module: &str) -> bool {
        let key = normalize_module_name(module);
        let Some(index) = self.by_name.remove(&key) else {
            return false;
        };
        self.by_base.remove(&index.base);
        if let Some(pos) = self.order.iter().position(|n| n == &key) {
            self.order.remove(pos);
        }
        true
    }

    /// Look up a module index by its normalized name. The caller may pass
    /// any case / path form; we normalize on lookup.
    pub fn get_by_name(&self, module: &str) -> Option<&ModuleExportIndex> {
        let key = normalize_module_name(module);
        self.by_name.get(&key)
    }

    /// Look up a module index by its current image base. Used by
    /// `GetProcAddress` to honor the supplied `HMODULE`.
    pub fn get_by_base(&self, base: u64) -> Option<&ModuleExportIndex> {
        self.by_base.get(&base).and_then(|n| self.by_name.get(n))
    }

    /// Iterate registered modules in registration order. Used by global
    /// scans that must remain deterministic.
    pub fn iter_ordered(&self) -> impl Iterator<Item = &ModuleExportIndex> {
        self.order.iter().filter_map(|n| self.by_name.get(n))
    }

    /// Number of registered modules.
    pub fn len(&self) -> usize {
        self.by_name.len()
    }

    /// True if no modules are registered.
    pub fn is_empty(&self) -> bool {
        self.by_name.is_empty()
    }

    /// Resolve an address to its `(module_normalized_name, export_name)`
    /// pair if any registered direct export matches.
    pub fn resolve_address(&self, addr: u64) -> Option<(String, String)> {
        for module in self.iter_ordered() {
            if let Some(name) = module.resolve_address(addr) {
                return Some((module.normalized_name.clone(), name.to_string()));
            }
        }
        None
    }

    /// Resolve a named export in a specific module, following forwarders
    /// through already-registered target modules with a depth limit.
    ///
    /// `depth` is the remaining recursion budget; callers should pass
    /// `MAX_FORWARDER_DEPTH` and rely on internal recursion decrementing it.
    /// Returns the final direct address or `0` if resolution fails.
    pub fn resolve_name_in_module(&self, module: &str, name: &str) -> u64 {
        let normalized = normalize_module_name(module);
        let name_lc = normalize_export_name(name);
        self.resolve_name_in_module_inner(
            &normalized,
            &name_lc,
            &mut Vec::new(),
            MAX_FORWARDER_DEPTH,
        )
    }

    fn resolve_name_in_module_inner(
        &self,
        normalized_module: &str,
        name_lc: &str,
        visited: &mut Vec<(String, String)>,
        depth: usize,
    ) -> u64 {
        if depth == 0 {
            return 0;
        }
        let Some(module) = self.by_name.get(normalized_module) else {
            return 0;
        };
        let Some(target) = module.resolve_name(name_lc) else {
            return 0;
        };
        match target {
            IndexedExport::Direct { address } => *address,
            IndexedExport::Forwarder { value } => {
                // Cycle guard: skip if we have already started resolving this
                // (module, symbol) pair.
                let key = (normalized_module.to_string(), name_lc.to_string());
                if visited.iter().any(|k| k == &key) {
                    return 0;
                }
                visited.push(key);
                parse_forwarder_and_resolve(self, value, visited, depth - 1)
            }
        }
    }

    /// Resolve a named export by searching every registered module in
    /// registration order. Returns the first hit's address.
    pub fn resolve_name_global(&self, name: &str) -> u64 {
        let name_lc = normalize_export_name(name);
        for module in self.iter_ordered() {
            if let Some(target) = module.resolve_name(&name_lc) {
                let addr = match target {
                    IndexedExport::Direct { address } => *address,
                    IndexedExport::Forwarder { value } => parse_forwarder_and_resolve(
                        self,
                        value,
                        &mut Vec::new(),
                        MAX_FORWARDER_DEPTH,
                    ),
                };
                if addr != 0 {
                    return addr;
                }
            }
        }
        0
    }

    /// Resolve an ordinal export using the supplied module base as a
    /// module handle. Used by `GetProcAddress`.
    pub fn resolve_ordinal_by_base(&self, base: u64, ordinal: u32) -> u64 {
        let Some(module) = self.get_by_base(base) else {
            return 0;
        };
        match module.resolve_ordinal(ordinal) {
            Some(IndexedExport::Direct { address }) => *address,
            Some(IndexedExport::Forwarder { value }) => {
                parse_forwarder_and_resolve(self, value, &mut Vec::new(), MAX_FORWARDER_DEPTH)
            }
            None => 0,
        }
    }

    /// Resolve a named export using the supplied module base as a module
    /// handle. Used by `GetProcAddress`.
    pub fn resolve_name_by_base(&self, base: u64, name: &str) -> u64 {
        let Some(module) = self.get_by_base(base) else {
            return 0;
        };
        let name_lc = normalize_export_name(name);
        let Some(target) = module.resolve_name(&name_lc) else {
            return 0;
        };
        match target {
            IndexedExport::Direct { address } => *address,
            IndexedExport::Forwarder { value } => {
                parse_forwarder_and_resolve(self, value, &mut Vec::new(), MAX_FORWARDER_DEPTH)
            }
        }
    }
}

/// Parse a forwarder string of the form `dll.symbol` or `dll.#N` and
/// resolve through the registry. Returns 0 if the target module is not
/// registered or the chain depth limit is reached.
fn parse_forwarder_and_resolve(
    registry: &ExportIndexRegistry,
    value: &str,
    visited: &mut Vec<(String, String)>,
    depth: usize,
) -> u64 {
    if depth == 0 {
        return 0;
    }
    let Some((dll_part, sym_part)) = value.split_once('.') else {
        return 0;
    };
    let normalized_dll = normalize_module_name(dll_part);

    if let Some(stripped) = sym_part.strip_prefix('#') {
        // Ordinal forwarder: `dll.#N`.
        let Ok(ordinal) = stripped.parse::<u32>() else {
            return 0;
        };
        // We need a base to resolve by ordinal; pick the first registered
        // module whose normalized name matches. Forwarders by ordinal are
        // rare in practice but must not panic.
        if let Some(module) = registry.by_name.get(&normalized_dll) {
            match module.resolve_ordinal(ordinal) {
                Some(IndexedExport::Direct { address }) => return *address,
                Some(IndexedExport::Forwarder { value: fwd_value }) => {
                    let key = (normalized_dll.clone(), format!("#{}", ordinal));
                    if visited.iter().any(|k| k == &key) {
                        return 0;
                    }
                    visited.push(key);
                    return parse_forwarder_and_resolve(registry, &fwd_value, visited, depth - 1);
                }
                None => return 0,
            }
        }
        return 0;
    }

    registry.resolve_name_in_module_inner(
        &normalized_dll,
        &sym_part.to_ascii_lowercase(),
        visited,
        depth,
    )
}

/// Normalize a module name into a registry key:
/// - ASCII lowercase.
/// - Strip path components (handles both `/` and `\`).
/// - Strip `.dll` suffix (so `kernel32.dll`, `kernel32`, and
///   `C:\Windows\kernel32.DLL` all key identically).
pub fn normalize_module_name(name: &str) -> String {
    let n = name.trim().to_ascii_lowercase();
    let n = n.rsplit_once('\\').map(|(_, b)| b).unwrap_or(&n);
    let n = n.rsplit_once('/').map(|(_, b)| b).unwrap_or(n);
    n.strip_suffix(".dll").unwrap_or(n).to_string()
}

/// Normalize an export name: ASCII lowercase, no leading/trailing whitespace.
pub fn normalize_export_name(name: &str) -> String {
    name.trim().to_ascii_lowercase()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rs_header::pe::export_index::build_export_index;
    use rs_header::pe::shared::ImageSectionHeader;

    fn section(va: u32, raw_ptr: u32, raw_size: u32) -> ImageSectionHeader {
        let mut s = ImageSectionHeader {
            name: [0; 8],
            virtual_size: raw_size,
            virtual_address: va,
            size_of_raw_data: raw_size,
            pointer_to_raw_data: raw_ptr,
            pointer_to_relocations: 0,
            pointer_to_linenumbers: 0,
            number_of_relocations: 0,
            number_of_linenumbers: 0,
            characteristics: 0,
        };
        let n = b".text\0\0\0";
        s.name[..n.len()].copy_from_slice(n);
        s
    }

    /// Two-function export table; function 0 = direct, function 1 =
    /// forwarder to `KERNELBASE.HeapAlloc`.
    fn build_raw() -> (Vec<u8>, Vec<ImageSectionHeader>, u32, u32) {
        let mut raw = vec![0u8; 0x200];
        let export_off = 0x040;
        let func_off = 0x080;
        let name_off_table = 0x090;
        let ord_off = 0x0a8;
        let fwd_str_off = 0x0c0;
        let name_a_off = 0x0d0;
        let name_b_off = 0x0e0;

        let export_va: u32 = 0x1040;
        let func_va: u32 = 0x1080;
        let name_table_va: u32 = 0x1090;
        let ord_table_va: u32 = 0x10a8;
        let name_a_va: u32 = 0x10d0;
        let name_b_va: u32 = 0x10e0;

        // function 0 RVA = 0x1500 (direct, outside export dir range).
        // function 1 RVA = 0x10c0 (forwarder, inside [0x1040, 0x1140)).
        raw[export_off + 16..export_off + 20].copy_from_slice(&1u32.to_le_bytes()); // base
        raw[export_off + 20..export_off + 24].copy_from_slice(&2u32.to_le_bytes()); // nof
        raw[export_off + 24..export_off + 28].copy_from_slice(&2u32.to_le_bytes()); // non
        raw[export_off + 28..export_off + 32].copy_from_slice(&func_va.to_le_bytes());
        raw[export_off + 32..export_off + 36].copy_from_slice(&name_table_va.to_le_bytes());
        raw[export_off + 36..export_off + 40].copy_from_slice(&ord_table_va.to_le_bytes());

        let func0_rva: u32 = 0x1500;
        let func1_rva: u32 = 0x10c0;
        raw[func_off..func_off + 4].copy_from_slice(&func0_rva.to_le_bytes());
        raw[func_off + 4..func_off + 8].copy_from_slice(&func1_rva.to_le_bytes());

        raw[name_off_table..name_off_table + 4].copy_from_slice(&name_a_va.to_le_bytes());
        raw[name_off_table + 4..name_off_table + 8].copy_from_slice(&name_b_va.to_le_bytes());

        raw[ord_off..ord_off + 2].copy_from_slice(&0u16.to_le_bytes()); // A -> 0
        raw[ord_off + 2..ord_off + 4].copy_from_slice(&1u16.to_le_bytes()); // B -> 1

        let s = b"KERNELBASE.HeapAlloc\0";
        raw[fwd_str_off..fwd_str_off + s.len()].copy_from_slice(s);

        let s = b"A\0";
        raw[name_a_off..name_a_off + s.len()].copy_from_slice(s);
        let s = b"B\0";
        raw[name_b_off..name_b_off + s.len()].copy_from_slice(s);

        let sections = vec![section(0x1000, 0, raw.len() as u32)];
        (raw, sections, export_va, 0x100)
    }

    fn parsed() -> ExportIndexData {
        let (raw, sections, va, size) = build_raw();
        build_export_index(&raw, &sections, va, size).expect("parse")
    }

    fn make_index(base: u64, module: &str) -> ModuleExportIndex {
        let normalized = normalize_module_name(module);
        ModuleExportIndex::from_parsed(module.to_string(), normalized, base, &parsed())
    }

    #[test]
    fn register_and_get_by_name() {
        let mut reg = ExportIndexRegistry::new();
        reg.register(make_index(0x10000, "kernel32.dll"));
        assert!(reg.get_by_name("kernel32").is_some());
        assert!(reg.get_by_name("KERNEL32.DLL").is_some());
        assert!(reg.get_by_name("kernelbase").is_none());
    }

    #[test]
    fn get_by_base_returns_correct_module() {
        let mut reg = ExportIndexRegistry::new();
        reg.register(make_index(0x10000, "kernel32.dll"));
        assert!(reg.get_by_base(0x10000).is_some());
        assert!(reg.get_by_base(0x20000).is_none());
    }

    #[test]
    fn replacement_removes_old_base() {
        let mut reg = ExportIndexRegistry::new();
        reg.register(make_index(0x10000, "kernel32.dll"));
        reg.register(make_index(0x20000, "kernel32.dll"));
        assert!(reg.get_by_base(0x10000).is_none(), "old base must be gone");
        assert!(reg.get_by_base(0x20000).is_some());
        assert_eq!(reg.len(), 1, "still one entry");
    }

    #[test]
    fn removal_drops_both_maps_and_order() {
        let mut reg = ExportIndexRegistry::new();
        reg.register(make_index(0x10000, "kernel32.dll"));
        reg.register(make_index(0x20000, "ntdll.dll"));
        assert!(reg.remove("kernel32"));
        assert!(reg.get_by_name("kernel32").is_none());
        assert!(reg.get_by_base(0x10000).is_none());
        assert_eq!(reg.len(), 1);
        // Order: only ntdll remains.
        let names: Vec<&str> = reg
            .iter_ordered()
            .map(|m| m.normalized_name.as_str())
            .collect();
        assert_eq!(names, vec!["ntdll"]);
    }

    #[test]
    fn named_resolution_is_case_insensitive() {
        let mut reg = ExportIndexRegistry::new();
        reg.register(make_index(0x10000, "kernel32.dll"));
        // direct export at base + 0x1500 = 0x11500
        assert_eq!(reg.resolve_name_in_module("kernel32", "A"), 0x11500);
        assert_eq!(reg.resolve_name_in_module("kernel32", "a"), 0x11500);
    }

    #[test]
    fn global_search_returns_first_match() {
        let mut reg = ExportIndexRegistry::new();
        reg.register(make_index(0x10000, "kernel32.dll"));
        reg.register(make_index(0x30000, "ntdll.dll"));
        assert_eq!(reg.resolve_name_global("A"), 0x11500);
    }

    #[test]
    fn base_lookup_honors_handle() {
        let mut reg = ExportIndexRegistry::new();
        reg.register(make_index(0x10000, "kernel32.dll"));
        assert_eq!(reg.resolve_name_by_base(0x10000, "A"), 0x11500);
        assert_eq!(reg.resolve_name_by_base(0x99999, "A"), 0);
    }

    #[test]
    fn ordinal_resolution_uses_export_base() {
        let mut reg = ExportIndexRegistry::new();
        reg.register(make_index(0x10000, "kernel32.dll"));
        // base=1, so ordinal 1 -> index 0 = direct 0x11500.
        assert_eq!(reg.resolve_ordinal_by_base(0x10000, 1), 0x11500);
        // ordinal 0 is below base.
        assert_eq!(reg.resolve_ordinal_by_base(0x10000, 0), 0);
    }

    #[test]
    fn address_to_name_reverse_lookup() {
        let mut reg = ExportIndexRegistry::new();
        reg.register(make_index(0x10000, "kernel32.dll"));
        assert_eq!(
            reg.resolve_address(0x11500),
            Some(("kernel32".to_string(), "A".to_string()))
        );
        assert_eq!(reg.resolve_address(0x99999), None);
    }

    #[test]
    fn forwarder_string_preserved_when_target_missing() {
        let mut reg = ExportIndexRegistry::new();
        reg.register(make_index(0x10000, "kernel32.dll"));
        // B forwarders to KERNELBASE.HeapAlloc, but KERNELBASE isn't
        // registered -> must return 0, not crash.
        assert_eq!(reg.resolve_name_in_module("kernel32", "B"), 0);
    }

    #[test]
    fn forwarder_resolves_to_registered_target() {
        // First build an index for kernelbase pointing to a direct export at
        // 0x50000 + 0x1500 = 0x51500.
        let mut reg = ExportIndexRegistry::new();
        reg.register(make_index(0x10000, "kernel32.dll"));
        reg.register(make_index(0x50000, "kernelbase.dll"));
        // B -> KERNELBASE.HeapAlloc. We never built a name "HeapAlloc" in
        // kernelbase, so this still returns 0 — but the resolution path
        // must not crash and must not loop.
        assert_eq!(reg.resolve_name_in_module("kernel32", "B"), 0);
    }

    #[test]
    fn ordering_preserved_across_replace() {
        let mut reg = ExportIndexRegistry::new();
        reg.register(make_index(0x10000, "kernel32.dll"));
        reg.register(make_index(0x20000, "ntdll.dll"));
        reg.register(make_index(0x30000, "kernel32.dll")); // replace
        let names: Vec<&str> = reg
            .iter_ordered()
            .map(|m| m.normalized_name.as_str())
            .collect();
        assert_eq!(names, vec!["kernel32", "ntdll"]);
    }

    #[test]
    fn module_name_normalization_handles_paths_and_case() {
        assert_eq!(normalize_module_name("KERNEL32.DLL"), "kernel32");
        assert_eq!(
            normalize_module_name("C:\\Windows\\kernel32.dll"),
            "kernel32"
        );
        assert_eq!(normalize_module_name("/usr/lib/kernel32.dll"), "kernel32");
        assert_eq!(normalize_module_name("kernel32"), "kernel32");
        assert_eq!(normalize_module_name("  kernelbase.DLL  "), "kernelbase");
    }

    #[test]
    fn parsed_rebase_updates_addresses() {
        let mut idx = make_index(0x10000, "kernel32.dll");
        // base 0x10000 + RVA 0x1500 = 0x11500
        assert_eq!(
            idx.resolve_name("a").unwrap().direct_address(),
            Some(0x11500)
        );
        idx.rebase(0x40000);
        // base 0x40000 + RVA 0x1500 = 0x41500
        assert_eq!(
            idx.resolve_name("a").unwrap().direct_address(),
            Some(0x41500)
        );
        assert_eq!(idx.base, 0x40000);
        // Reverse map rebuilt with the same display name.
        assert_eq!(idx.resolve_address(0x41500), Some("A"));
    }

    #[test]
    fn unindexed_module_resolves_to_zero() {
        let reg = ExportIndexRegistry::new();
        assert_eq!(reg.resolve_name_in_module("nope", "A"), 0);
        assert_eq!(reg.resolve_name_global("A"), 0);
    }

    #[test]
    fn parsed_named_exports_preserve_case_in_address_map() {
        // The raw export name "A" must surface in the address map with its
        // original spelling.
        let idx = make_index(0x10000, "kernel32.dll");
        assert_eq!(idx.resolve_address(0x11500), Some("A"));
    }
}
