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

use crate::api::windows::common::modules;
use std::collections::HashMap;
use std::sync::OnceLock;

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
    #[serde(skip)]
    pub(crate) service_names: OnceLock<HashMap<u64, String>>,
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
            service_names: OnceLock::new(),
        }
    }

    /// Recompute all direct addresses for a new module base. Forwarder
    /// entries are left untouched. The reverse-address map is rebuilt using
    /// the preserved `display_names`.
    pub fn rebase(&mut self, new_base: u64) {
        if self.base == new_base {
            return;
        }
        self.service_names.take();
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

    /// Resolve a guest export name without folding case or trimming it.
    pub fn resolve_name_exact(&self, name: &str) -> Option<&IndexedExport> {
        let (index, _) = self
            .display_names
            .iter()
            .find(|(_, export)| export == name)?;
        self.by_ordinal.get(*index as usize)?.as_ref()
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

    // Resolve address to its name
    #[inline]
    pub fn resolve_address_module(&self, module_name: &str, addr: u64) -> Option<&str> {
        self.by_name.get(module_name)?.resolve_address(addr)
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
            false,
            &mut Vec::new(),
            MAX_FORWARDER_DEPTH,
        )
    }

    fn resolve_name_in_module_inner(
        &self,
        normalized_module: &str,
        name: &str,
        exact: bool,
        visited: &mut Vec<(String, String)>,
        depth: usize,
    ) -> u64 {
        if depth == 0 {
            return 0;
        }
        let Some(module) = self.by_name.get(normalized_module) else {
            return 0;
        };
        let target = if exact {
            module.resolve_name_exact(name)
        } else {
            module.resolve_name(name)
        };
        let Some(target) = target else {
            return 0;
        };
        match target {
            IndexedExport::Direct { address } => *address,
            IndexedExport::Forwarder { value } => {
                // Cycle guard: skip if we have already started resolving this
                // (module, symbol) pair.
                let key = (normalized_module.to_string(), name.to_string());
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

    /// Resolve an ordinal export by module name (import-time use, e.g. PE64
    /// import binding). Module name is normalized before lookup; unknown
    /// modules, below-base ordinals, missing slots, and unresolvable
    /// forwarders all return 0. Ordinals are module-scoped on purpose:
    /// a global ordinal search would be semantically wrong.
    pub fn resolve_ordinal_in_module(&self, module: &str, ordinal: u32) -> u64 {
        let key = normalize_module_name(module);
        let Some(index) = self.by_name.get(&key) else {
            return 0;
        };
        match index.resolve_ordinal(ordinal) {
            Some(IndexedExport::Direct { address }) => *address,
            Some(IndexedExport::Forwarder { value }) => {
                parse_forwarder_and_resolve(self, value, &mut Vec::new(), MAX_FORWARDER_DEPTH)
            }
            None => 0,
        }
    }

    /// Resolve a named export using the supplied module base as a module
    /// handle. Names are case-sensitive, as required by `GetProcAddress`.
    pub fn resolve_name_by_base(&self, base: u64, name: &str) -> u64 {
        let Some(module) = self.get_by_base(base) else {
            return 0;
        };
        let Some(target) = module.resolve_name_exact(name) else {
            return 0;
        };
        match target {
            IndexedExport::Direct { address } => *address,
            IndexedExport::Forwarder { value } => {
                parse_forwarder_and_resolve(self, value, &mut Vec::new(), MAX_FORWARDER_DEPTH)
            }
        }
    }

    /// Find a reflective-loader entry across every registered module.
    ///
    /// Reflective-loader payloads (e.g. Cobalt Strike beacons) export a
    /// `ReflectiveLoader` function — the decorated name varies
    /// (`_ReflectiveLoader@4`, `ReflectiveLoader`, ...). That export, not the
    /// PE `AddressOfEntryPoint`, is the real start: it maps/decrypts the image
    /// and only then calls the true `DllMain`. Returns the
    /// `(module_name, address)` of the first direct match, or `None`.
    pub fn resolve_reflective_loader(&self) -> Option<(String, u64)> {
        for module in self.iter_ordered() {
            for (name_lc, &ord_idx) in module.by_name.iter() {
                if name_lc.contains("reflectiveloader")
                    && let Some(Some(IndexedExport::Direct { address })) =
                        module.by_ordinal.get(ord_idx as usize)
                {
                    return Some((module.module_name.clone(), *address));
                }
            }
        }
        None
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
    let dll = normalize_module_name(dll_part);
    let ordinal = match sym_part.strip_prefix('#') {
        Some(text) => match text.parse::<u32>() {
            Ok(value) => Some(value),
            Err(_) => return 0,
        },
        None => None,
    };
    let mut resolve = |module_name: &str| {
        let module = registry.by_name.get(module_name)?;
        let entry = match ordinal {
            Some(value) => module.resolve_ordinal(value),
            None => module.resolve_name_exact(sym_part),
        }?;
        match entry {
            IndexedExport::Direct { address } => Some(*address),
            IndexedExport::Forwarder { value } => {
                let key = (module_name.to_owned(), sym_part.to_owned());
                if visited.contains(&key) {
                    return None;
                }
                let previous = visited.len();
                visited.push(key);
                let address = parse_forwarder_and_resolve(registry, value, visited, depth - 1);
                visited.truncate(previous);
                (address != 0).then_some(address)
            }
        }
    };
    if registry.by_name.contains_key(&dll) || !modules::is_api_set_contract(&dll) {
        return resolve(&dll).unwrap_or(0);
    }
    modules::api_set_hosts(&dll)
        .iter()
        .find_map(|host| resolve(&normalize_module_name(host)))
        .unwrap_or(0)
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
