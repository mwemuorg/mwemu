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

use crate::pe::image::ImageLayout;
use crate::pe::readers::{read_c_string, read_u16_le, read_u32_le};
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
    parse_export_index(raw.len(), export_dir_va, export_dir_size, |rva| {
        raw.get(vaddr_to_off(sections, rva)?..)
    })
}

/// Parse exports using the loaded image's headers, alignment and initialized ranges.
pub fn build_image_export_index(
    raw: &[u8],
    layout: ImageLayout<'_>,
    export_dir_va: u32,
    export_dir_size: u32,
) -> Option<ExportIndexData> {
    parse_export_index(raw.len(), export_dir_va, export_dir_size, |rva| {
        layout.data_at(raw, rva)
    })
}

fn parse_export_index<'a>(
    raw_size: usize,
    export_dir_va: u32,
    export_dir_size: u32,
    data_at: impl Fn(u32) -> Option<&'a [u8]>,
) -> Option<ExportIndexData> {
    const DIRECTORY_SIZE: usize = 40;
    const RVA_SIZE: usize = size_of::<u32>();
    const ORDINAL_SIZE: usize = size_of::<u16>();
    if export_dir_va == 0 || export_dir_size < DIRECTORY_SIZE as u32 {
        return None;
    }
    let directory = data_at(export_dir_va)?.get(..DIRECTORY_SIZE)?;
    let base = read_u32_le(directory, 16);
    let functions = read_u32_le(directory, 20)
        .min(MAX_EXPORT_FUNCTIONS)
        .min((raw_size / RVA_SIZE) as u32);
    let names = read_u32_le(directory, 24)
        .min(MAX_EXPORT_NAMES)
        .min((raw_size / RVA_SIZE) as u32);
    let function_rva = read_u32_le(directory, 28);
    if function_rva == 0 {
        return None;
    }
    let function_table = data_at(function_rva)?.get(..functions as usize * RVA_SIZE)?;
    let mut ordinal_targets = Vec::with_capacity(functions as usize);
    for entry in function_table.chunks_exact(RVA_SIZE) {
        let rva = read_u32_le(entry, 0);
        let target = if rva == 0 {
            None
        } else if (rva as u64) >= export_dir_va as u64
            && (rva as u64) < export_dir_va as u64 + export_dir_size as u64
        {
            data_at(rva)
                .map(|data| read_c_string(data, 0))
                .filter(|name| !name.is_empty())
                .map(|value| ExportTarget::Forwarder { value })
        } else {
            Some(ExportTarget::Direct { rva })
        };
        ordinal_targets.push(target);
    }

    let mut named_exports = Vec::with_capacity(names as usize);
    if names != 0 {
        let name_rva = read_u32_le(directory, 32);
        let ordinal_rva = read_u32_le(directory, 36);
        if name_rva == 0 || ordinal_rva == 0 {
            return None;
        }
        let name_table = data_at(name_rva)?.get(..names as usize * RVA_SIZE)?;
        let ordinal_table = data_at(ordinal_rva)?.get(..names as usize * ORDINAL_SIZE)?;
        for (name, ordinal) in name_table
            .chunks_exact(RVA_SIZE)
            .zip(ordinal_table.chunks_exact(ORDINAL_SIZE))
        {
            let ordinal_index = read_u16_le(ordinal, 0) as u32;
            if ordinal_index >= functions {
                continue;
            }
            let name_rva = read_u32_le(name, 0);
            if name_rva == 0 {
                continue;
            }
            let Some(data) = data_at(name_rva) else {
                continue;
            };
            let name = read_c_string(data, 0);
            if !name.is_empty() {
                named_exports.push(NamedExport {
                    name,
                    ordinal_index,
                });
            }
        }
    }
    Some(ExportIndexData {
        export_base: base,
        number_of_functions: functions,
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
            return sect
                .pointer_to_raw_data
                .checked_add(offset_within_section)
                .map(|offset| offset as usize);
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
