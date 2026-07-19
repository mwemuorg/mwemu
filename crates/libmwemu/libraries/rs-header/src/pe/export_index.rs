//! Pure parsed representation of a PE export directory, independent of any
//! emulator / memory model.
//!
//! Built once per mapped PE image from the file's raw bytes. The runtime
//! resolver then reuses this index for O(1) named / ordinal lookups instead of
//! rescanning export tables (and the emulated guest's exported-name strings)
//! on every IAT import.
//!
//! The data here is architecture-neutral:
//! - function-table RVAs are `u32` (PE RVAs are 32-bit even on PE64/AArch64).
//! - the runtime wrapper re-bases them with the current module base.
//! - PE32 callers must narrow the resolved address back to `u32` after
//!   validation; the parser itself does not enforce a guest pointer width.

use crate::pe::readers::{read_c_string, read_u32_le, read_u16_le};
use crate::pe::shared::ImageSectionHeader;

/// Maximum number of function-table entries we will accept from a hostile /
/// malformed export directory. Real Windows DLLs export well under 100k
/// symbols; 1M is a generous ceiling that still prevents trivial DoS from a
/// PE claiming a bogus `NumberOfFunctions`.
const MAX_EXPORT_FUNCTIONS: u32 = 1 << 20;

/// Maximum number of named exports we will accept.
const MAX_EXPORT_NAMES: u32 = 1 << 20;

/// One function-table slot. `ordinal_targets[i]` corresponds to the export
/// ordinal `export_base + i`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExportTarget {
    /// Direct export: the function lives at `image_base + rva` (runtime
    /// computes the absolute address). The RVA here is a raw RVA, not a VA.
    Direct { rva: u32 },
    /// Forwarder: the function lives in another module. The string is the
    /// original NUL-terminated forwarder string as it appears in the export
    /// table (e.g. `KERNELBASE.HeapAlloc`, `api-ms-win-core-memory-l1-1-0.HeapAlloc`,
    /// or `KERNELBASE.#123`). Resolve at lookup time so the target module can
    /// be mapped later.
    Forwarder { value: String },
}

/// One named export entry. The name maps through `AddressOfNameOrdinals` to a
/// function-table slot; `ordinal_index` is that function-table index (NOT the
/// export ordinal). Keeping the indirection here is what makes aliases work
/// (two names -> same function-table slot) and keeps name/ordinal lookup
/// consistent.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NamedExport {
    /// Original spelling from the export table (not lowercased).
    pub name: String,
    /// Index into the function table that this name points to.
    pub ordinal_index: u32,
}

/// Parsed export directory contents. Holds only owned Rust data and integer
/// RVAs; no emulator references.
#[derive(Debug, Clone, Default)]
pub struct ExportIndexData {
    /// Export ordinal base (`IMAGE_EXPORT_DIRECTORY.Base`). Ordinal
    /// `Base + i` lives at `ordinal_targets[i]`.
    pub export_base: u32,
    /// Number of function-table entries parsed (capped, may be smaller than
    /// the directory's `NumberOfFunctions` for malformed inputs).
    pub number_of_functions: u32,
    /// One slot per function-table entry. `None` means the RVA was 0 or out
    /// of range (treated as an unresolved / missing export, not as address 0).
    pub ordinal_targets: Vec<Option<ExportTarget>>,
    /// Named exports (may be empty). Order follows the name table order so
    /// iteration is deterministic.
    pub named_exports: Vec<NamedExport>,
}

impl ExportIndexData {
    /// True if this index carries no usable data (empty function table).
    pub fn is_empty(&self) -> bool {
        self.ordinal_targets.is_empty()
    }
}

/// Build an `ExportIndexData` from the file's raw bytes, the parsed section
/// headers, and the export data-directory entry (RVA + size). Returns `None`
/// if the export directory is absent, malformed beyond recovery, or too small
/// to safely hold the directory header.
pub fn build_export_index(
    raw: &[u8],
    sections: &[ImageSectionHeader],
    export_dir_va: u32,
    export_dir_size: u32,
) -> Option<ExportIndexData> {
    if export_dir_va == 0 || export_dir_size < 40 {
        return None;
    }

    let export_off = vaddr_to_off(sections, export_dir_va)?;
    if export_off.checked_add(40)? > raw.len() {
        return None;
    }

    let base = read_u32_le(raw, export_off + 16);
    let number_of_functions = read_u32_le(raw, export_off + 20);
    let number_of_names = read_u32_le(raw, export_off + 24);
    let address_of_functions = read_u32_le(raw, export_off + 28);
    let address_of_names = read_u32_le(raw, export_off + 32);
    let address_of_name_ordinals = read_u32_le(raw, export_off + 36);

    // Defensive caps: cap by the explicit limits, by raw length / 4 (each
    // function-table entry is 4 bytes), and by our hard ceiling.
    let nof = number_of_functions.min(MAX_EXPORT_FUNCTIONS);
    let raw_func_cap = (raw.len() / 4) as u32;
    let nof = nof.min(raw_func_cap);
    let non = number_of_names.min(MAX_EXPORT_NAMES);
    let raw_name_cap = (raw.len() / 4) as u32;
    let non = non.min(raw_name_cap);

    // Convert each table VA to its raw offset and bounds-check there. The
    // function/name/ordinal table RVAs are VAs in the section space, NOT raw
    // offsets, so checking them against raw.len() directly would always fail
    // for non-zero section bases.
    let func_table_off = match vaddr_to_off(sections, address_of_functions) {
        Some(o) => o,
        None => return None,
    };
    let func_table_end = func_table_off
        .checked_add((nof as usize).checked_mul(4)?)?;
    if address_of_functions == 0 || func_table_end > raw.len() {
        return None;
    }

    let name_table_off = match vaddr_to_off(sections, address_of_names) {
        Some(o) => o,
        None => return None,
    };
    let name_table_end = name_table_off.checked_add((non as usize).checked_mul(4)?)?;
    if address_of_names == 0 || name_table_end > raw.len() {
        return None;
    }

    let ord_table_off = match vaddr_to_off(sections, address_of_name_ordinals) {
        Some(o) => o,
        None => return None,
    };
    let ord_table_end = ord_table_off.checked_add((non as usize).checked_mul(2)?)?;
    if address_of_name_ordinals == 0 || ord_table_end > raw.len() {
        return None;
    }

    let mut ordinal_targets: Vec<Option<ExportTarget>> = Vec::with_capacity(nof as usize);
    for i in 0..nof {
        let func_off = func_table_off + (i as usize) * 4;
        let rva = read_u32_le(raw, func_off);

        if rva == 0 {
            ordinal_targets.push(None);
            continue;
        }

        if export_dir_size != 0
            && rva >= export_dir_va
            && rva < export_dir_va.saturating_add(export_dir_size)
        {
            // Forwarder: read the NUL-terminated string at the RVA in raw
            // bytes. If it is empty / out of range we record `None` (treated
            // as unresolved) rather than fabricating a target the resolver
            // would have to fall back from anyway.
            let fwd_off = match vaddr_to_off(sections, rva) {
                Some(o) => o,
                None => {
                    ordinal_targets.push(None);
                    continue;
                }
            };
            if fwd_off >= raw.len() {
                ordinal_targets.push(None);
                continue;
            }
            let value = read_c_string(raw, fwd_off);
            if value.is_empty() {
                ordinal_targets.push(None);
                continue;
            }
            ordinal_targets.push(Some(ExportTarget::Forwarder { value }));
        } else {
            ordinal_targets.push(Some(ExportTarget::Direct { rva }));
        }
    }

    let mut named_exports: Vec<NamedExport> = Vec::with_capacity(non as usize);
    for i in 0..non {
        let name_ptr_off = name_table_off + (i as usize) * 4;
        let name_ord_off = ord_table_off + (i as usize) * 2;

        let name_rva = read_u32_le(raw, name_ptr_off);
        let ord_idx = read_u16_le(raw, name_ord_off) as u32;

        if ord_idx >= nof {
            // Malformed: name points outside the function table. Skip.
            continue;
        }

        let name_off = match vaddr_to_off(sections, name_rva) {
            Some(o) => o,
            None => continue,
        };
        if name_off >= raw.len() {
            continue;
        }
        let name = read_c_string(raw, name_off);
        if name.is_empty() {
            continue;
        }

        named_exports.push(NamedExport {
            name,
            ordinal_index: ord_idx,
        });
    }

    Some(ExportIndexData {
        export_base: base,
        number_of_functions: nof,
        ordinal_targets,
        named_exports,
    })
}

/// Convert a section RVA to a raw-file offset. Returns `None` if the RVA is
/// not contained in any section or falls in uninitialized data. Bounds-safe
/// version that mirrors the defensive `vaddr_to_off` already used in the
/// PE64 parser.
fn vaddr_to_off(sections: &[ImageSectionHeader], vaddr: u32) -> Option<usize> {
    for sect in sections {
        let sec_end = sect.virtual_address.saturating_add(sect.virtual_size);
        if vaddr >= sect.virtual_address && vaddr < sec_end {
            let offset_within_section = vaddr - sect.virtual_address;
            if offset_within_section >= sect.size_of_raw_data {
                return None;
            }
            return Some((sect.pointer_to_raw_data + offset_within_section) as usize);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pe::shared::ImageSectionHeader;

    /// Build a section whose virtual range is `[VA, VA + virtual_size)` and
    /// whose raw bytes start at `pointer_to_raw_data` in the file.
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

    /// Build a minimal raw buffer with a single export directory + 1 function
    /// entry + 1 named export. Returns (raw, sections, export_va, export_size).
    ///
    /// File layout:
    ///   0x000: section raw data begins (single .text section, VA = 0x1000)
    ///   0x040: export directory (40 bytes)
    ///   0x080: function table (1 u32)
    ///   0x090: name table (1 u32 name RVA)
    ///   0x0a0: name-ordinals table (1 u16)
    ///   0x0b0: forwarder string (only written when `forwarder` is true)
    ///   0x0d0: name "A"
    fn build_export_raw(
        base: u32,
        func_rva: u32,
        forwarder: bool,
    ) -> (Vec<u8>, Vec<ImageSectionHeader>, u32, u32) {
        let mut raw = vec![0u8; 0x200];
        let export_off = 0x040;
        let func_off = 0x080;
        let name_off_table = 0x090;
        let ord_off = 0x0a0;
        let fwd_str_off = 0x0b0;
        let name_a_off = 0x0d0;

        let export_va: u32 = 0x1040;
        let func_va: u32 = 0x1080;
        let name_table_va: u32 = 0x1090;
        let ord_table_va: u32 = 0x10a0;
        let name_a_va: u32 = 0x10d0;

        raw[export_off + 16..export_off + 20].copy_from_slice(&base.to_le_bytes());
        raw[export_off + 20..export_off + 24].copy_from_slice(&1u32.to_le_bytes()); // nof
        raw[export_off + 24..export_off + 28].copy_from_slice(&1u32.to_le_bytes()); // non
        raw[export_off + 28..export_off + 32].copy_from_slice(&func_va.to_le_bytes());
        raw[export_off + 32..export_off + 36].copy_from_slice(&name_table_va.to_le_bytes());
        raw[export_off + 36..export_off + 40].copy_from_slice(&ord_table_va.to_le_bytes());

        raw[func_off..func_off + 4].copy_from_slice(&func_rva.to_le_bytes());
        raw[name_off_table..name_off_table + 4].copy_from_slice(&name_a_va.to_le_bytes());
        raw[ord_off..ord_off + 2].copy_from_slice(&0u16.to_le_bytes());

        if forwarder {
            let s = b"KERNELBASE.HeapAlloc\0";
            raw[fwd_str_off..fwd_str_off + s.len()].copy_from_slice(s);
        }

        let s = b"A\0";
        raw[name_a_off..name_a_off + s.len()].copy_from_slice(s);

        let sections = vec![section(0x1000, 0, raw.len() as u32)];
        (raw, sections, export_va, 0x100)
    }

    #[test]
    fn absent_export_directory_returns_none() {
        let raw = vec![0u8; 0x200];
        let sections = vec![section(0x1000, 0, raw.len() as u32)];
        // No directory.
        assert!(build_export_index(&raw, &sections, 0, 0).is_none());
        // Truncated directory size.
        assert!(build_export_index(&raw, &sections, 0x1040, 10).is_none());
    }

    #[test]
    fn direct_export_parses() {
        // func_rva = 0x1500 lies outside the export directory range
        // (0x1040..0x1140), so it must be classified as Direct, not Forwarder.
        let (raw, sections, export_va, export_size) = build_export_raw(1, 0x1500, false);
        let idx = build_export_index(&raw, &sections, export_va, export_size).unwrap();
        assert_eq!(idx.export_base, 1);
        assert_eq!(idx.number_of_functions, 1);
        assert_eq!(idx.ordinal_targets.len(), 1);
        match &idx.ordinal_targets[0] {
            Some(ExportTarget::Direct { rva }) => assert_eq!(*rva, 0x1500),
            other => panic!("expected direct, got {:?}", other),
        }
        assert_eq!(idx.named_exports.len(), 1);
        assert_eq!(idx.named_exports[0].name, "A");
        assert_eq!(idx.named_exports[0].ordinal_index, 0);
    }

    #[test]
    fn forwarder_rva_in_range_is_parsed_as_forwarder() {
        // func_rva = 0x10b0 falls inside [0x1040, 0x1140) so it must be a
        // forwarder and the string at that raw offset is the forwarder target.
        let (raw, sections, export_va, export_size) = build_export_raw(1, 0x10b0, true);
        let idx = build_export_index(&raw, &sections, export_va, export_size).unwrap();
        match &idx.ordinal_targets[0] {
            Some(ExportTarget::Forwarder { value }) => {
                assert_eq!(value, "KERNELBASE.HeapAlloc");
            }
            other => panic!("expected forwarder, got {:?}", other),
        }
    }

    #[test]
    fn zero_function_rva_yields_none_slot() {
        let (mut raw, sections, export_va, export_size) = build_export_raw(1, 0x1500, false);
        // Force the function RVA to 0 (read_u32_le returns 0 for that slot).
        raw[0x080..0x084].copy_from_slice(&0u32.to_le_bytes());
        let idx = build_export_index(&raw, &sections, export_va, export_size).unwrap();
        assert!(idx.ordinal_targets[0].is_none());
    }

    #[test]
    fn malformed_counts_do_not_panic() {
        // A hostile PE may claim an absurd NumberOfFunctions. The parser
        // must not panic, must not allocate excessively, and must return
        // either None or a clamped Some — never UB.
        let (mut raw, sections, export_va, export_size) = build_export_raw(1, 0x1500, false);
        raw[0x040 + 20..0x040 + 24].copy_from_slice(&1_000_000u32.to_le_bytes());
        let idx = build_export_index(&raw, &sections, export_va, export_size);
        if let Some(idx) = idx {
            assert!(idx.number_of_functions <= MAX_EXPORT_FUNCTIONS);
        }
        // Out-of-range NumberOfNames as well.
        raw[0x040 + 24..0x040 + 28].copy_from_slice(&u32::MAX.to_le_bytes());
        let _ = build_export_index(&raw, &sections, export_va, export_size);
    }

    #[test]
    fn non_zero_ordinal_base_indexes_correctly() {
        let (raw, sections, export_va, export_size) = build_export_raw(5, 0x1500, false);
        let idx = build_export_index(&raw, &sections, export_va, export_size).unwrap();
        assert_eq!(idx.export_base, 5);
    }
}