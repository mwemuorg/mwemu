//! End-to-end loader test: bind a real PE's imports through a mock `PeLoader`
//! (an in-memory backend), proving the generic loader works without an
//! emulator and that `import_addr_to_name` is reversible from `iat_names`.

use std::collections::HashMap;

use rs_header::pe::PeLoader;
use rs_header::pe::pe32::PE32;
use rs_header::pe::pe64::PE64;

/// Minimal in-memory PeLoader: records writes and hands out fake, stable
/// addresses for resolved imports.
struct Mock {
    mem: HashMap<u64, u8>,
    next: u64,
    resolved: HashMap<String, u64>,
}

impl Mock {
    fn new() -> Self {
        Mock {
            mem: HashMap::new(),
            next: 0x7000_0000,
            resolved: HashMap::new(),
        }
    }
}

impl PeLoader for Mock {
    fn is_mapped(&self, _addr: u64) -> bool {
        true
    }
    fn write_bytes(&mut self, addr: u64, data: &[u8]) -> bool {
        for (i, b) in data.iter().enumerate() {
            self.mem.insert(addr + i as u64, *b);
        }
        true
    }
    fn write_dword(&mut self, addr: u64, val: u32) -> bool {
        self.write_bytes(addr, &val.to_le_bytes())
    }
    fn write_qword(&mut self, addr: u64, val: u64) -> bool {
        self.write_bytes(addr, &val.to_le_bytes())
    }
    fn load_library(&mut self, _libname: &str) -> u64 {
        0x1000 // non-zero = "loaded"
    }
    fn resolve_api_name(&mut self, name: &str) -> u64 {
        self.resolve_api_name_in_module("?", name)
    }
    fn resolve_api_name_in_module(&mut self, _module: &str, name: &str) -> u64 {
        if let Some(a) = self.resolved.get(name) {
            return *a;
        }
        let a = self.next;
        self.next += 0x10;
        self.resolved.insert(name.to_string(), a);
        a
    }
    fn search_api_name(&mut self, name: &str) -> (u64, String, String) {
        (
            self.resolve_api_name(name),
            "mock".to_string(),
            name.to_string(),
        )
    }
}

static LOADER64: &[u8] = include_bytes!("fixtures/loader64.exe");

/// Build a minimal synthetic PE32 image with a single HIGHLOW relocation
/// entry at RVA 0x2030 pointing into a single mapped section at RVA 0x2000.
/// The original DWORD at that RVA is `ORIGINAL_VAL`, so a relocation with
/// delta `D = base_addr - image_base` is expected to write
/// `ORIGINAL_VAL.wrapping_add(D)` to guest address `base_addr + 0x2030`.
fn build_synthetic_pe32_with_reloc(original_val: u32) -> Vec<u8> {
    const IMAGE_BASE: u32 = 0x0040_0000;
    const SECTION_ALIGNMENT: u32 = 0x1000;
    const FILE_ALIGNMENT: u32 = 0x200;
    const SIZE_OF_HEADERS: u32 = 0x200;
    const SIZE_OF_OPTIONAL_HEADER: u16 = 0xE0; // PE32: 96 fixed + 16 data dirs * 8
    const NUMBER_OF_RVA_AND_SIZES: u32 = 6; // import, delay, basereloc, ...
    const SECTION_RAW_PTR: u32 = 0x200;
    const SECTION_RAW_SIZE: u32 = 0x200;
    const SECTION_VIRTUAL_ADDRESS: u32 = 0x2000;
    const SECTION_VIRTUAL_SIZE: u32 = 0x1000;
    const RELOC_RVA: u32 = 0x2000;
    const RELOC_BLOCK_SIZE: u32 = 12; // 8 header + 2 entries * 2 bytes
    const RELOC_TARGET_RVA: u32 = 0x2030;
    const RELOC_TARGET_RAW: usize =
        SECTION_RAW_PTR as usize + (RELOC_TARGET_RVA - SECTION_VIRTUAL_ADDRESS) as usize;
    const TOTAL_SIZE: usize = (SECTION_RAW_PTR + SECTION_RAW_SIZE) as usize;

    assert!(TOTAL_SIZE > RELOC_TARGET_RAW + 4);
    // Headers (0..SIZE_OF_HEADERS) and section raw data
    // (SECTION_RAW_PTR..SECTION_RAW_PTR + SECTION_RAW_SIZE) must not overlap.
    assert!(SIZE_OF_HEADERS as usize <= SECTION_RAW_PTR as usize);

    let mut raw = vec![0u8; TOTAL_SIZE];

    // DOS header (64 bytes)
    raw[0..2].copy_from_slice(&0x5A4Du16.to_le_bytes()); // e_magic = "MZ"
    raw[60..64].copy_from_slice(&0x80u32.to_le_bytes()); // e_lfanew

    let nt_off = 0x80usize;

    // NT signature
    raw[nt_off..nt_off + 4].copy_from_slice(&0x0000_4550u32.to_le_bytes());

    // COFF file header (20 bytes) at nt_off + 4
    let fh_off = nt_off + 4;
    raw[fh_off..fh_off + 2].copy_from_slice(&0x014Cu16.to_le_bytes()); // machine = i386
    raw[fh_off + 2..fh_off + 4].copy_from_slice(&1u16.to_le_bytes()); // number_of_sections
    raw[fh_off + 4..fh_off + 8].copy_from_slice(&0u32.to_le_bytes()); // time_date_stamp
    raw[fh_off + 8..fh_off + 12].copy_from_slice(&0u32.to_le_bytes()); // pointer_to_symbol_table
    raw[fh_off + 12..fh_off + 16].copy_from_slice(&0u32.to_le_bytes()); // number_of_symbols
    raw[fh_off + 16..fh_off + 18].copy_from_slice(&SIZE_OF_OPTIONAL_HEADER.to_le_bytes()); // size_of_optional_header
    raw[fh_off + 18..fh_off + 20].copy_from_slice(&0u16.to_le_bytes()); // characteristics

    // Optional header at nt_off + 24, size SIZE_OF_OPTIONAL_HEADER
    let opt_off = nt_off + 24;
    raw[opt_off..opt_off + 2].copy_from_slice(&0x010Bu16.to_le_bytes()); // magic = PE32
    raw[opt_off + 28..opt_off + 32].copy_from_slice(&IMAGE_BASE.to_le_bytes()); // image_base
    raw[opt_off + 32..opt_off + 36].copy_from_slice(&SECTION_ALIGNMENT.to_le_bytes()); // section_alignment
    raw[opt_off + 36..opt_off + 40].copy_from_slice(&FILE_ALIGNMENT.to_le_bytes()); // file_alignment
    raw[opt_off + 60..opt_off + 64].copy_from_slice(&SIZE_OF_HEADERS.to_le_bytes()); // size_of_headers
    raw[opt_off + 92..opt_off + 96].copy_from_slice(&NUMBER_OF_RVA_AND_SIZES.to_le_bytes()); // number_of_rva_and_sizes

    // Data directories start at opt_off + 96; entry 5 is BASERELOC.
    let dd_off = opt_off + 96;
    let basereloc_off = dd_off + 5 * 8;
    raw[basereloc_off..basereloc_off + 4].copy_from_slice(&RELOC_RVA.to_le_bytes());
    raw[basereloc_off + 4..basereloc_off + 8].copy_from_slice(&RELOC_BLOCK_SIZE.to_le_bytes());

    // Single section header (40 bytes) at opt_off + SIZE_OF_OPTIONAL_HEADER
    let sect_off = opt_off + SIZE_OF_OPTIONAL_HEADER as usize;
    let name = b".text\0\0\0";
    raw[sect_off..sect_off + 8].copy_from_slice(name);
    let s2 = sect_off + 8;
    raw[s2..s2 + 4].copy_from_slice(&SECTION_VIRTUAL_SIZE.to_le_bytes());
    raw[s2 + 4..s2 + 8].copy_from_slice(&SECTION_VIRTUAL_ADDRESS.to_le_bytes());
    raw[s2 + 8..s2 + 12].copy_from_slice(&SECTION_RAW_SIZE.to_le_bytes());
    raw[s2 + 12..s2 + 16].copy_from_slice(&SECTION_RAW_PTR.to_le_bytes());
    raw[s2 + 16..s2 + 20].copy_from_slice(&0u32.to_le_bytes()); // pointer_to_relocations
    raw[s2 + 20..s2 + 24].copy_from_slice(&0u32.to_le_bytes()); // pointer_to_linenumbers
    raw[s2 + 24..s2 + 26].copy_from_slice(&0u16.to_le_bytes()); // number_of_relocations
    raw[s2 + 26..s2 + 28].copy_from_slice(&0u16.to_le_bytes()); // number_of_linenumbers
    raw[s2 + 28..s2 + 32].copy_from_slice(&0u32.to_le_bytes()); // characteristics

    // Relocation block at SECTION_RAW_PTR
    raw[SECTION_RAW_PTR as usize..SECTION_RAW_PTR as usize + 4]
        .copy_from_slice(&RELOC_RVA.to_le_bytes());
    raw[SECTION_RAW_PTR as usize + 4..SECTION_RAW_PTR as usize + 8]
        .copy_from_slice(&RELOC_BLOCK_SIZE.to_le_bytes());
    // entry 0: HIGHLOW at offset 0x30 of page 0x2000 -> target RVA 0x2030
    raw[SECTION_RAW_PTR as usize + 8..SECTION_RAW_PTR as usize + 10]
        .copy_from_slice(&0x3030u16.to_le_bytes());
    // entry 1: ABSOLUTE padding (must be skipped)
    raw[SECTION_RAW_PTR as usize + 10..SECTION_RAW_PTR as usize + 12]
        .copy_from_slice(&0x0000u16.to_le_bytes());

    // Original DWORD at the relocation target
    raw[RELOC_TARGET_RAW..RELOC_TARGET_RAW + 4].copy_from_slice(&original_val.to_le_bytes());

    raw
}

#[test]
fn iat_binding_generic_roundtrip() {
    let mut pe = PE64::parse("loader64.exe", LOADER64);
    let mut mock = Mock::new();
    let base = 0x4000_0000u64;

    // The generic loader runs against the mock backend without any emulator.
    pe.iat_binding(LOADER64, &mut mock, base);
    pe.delay_load_binding(LOADER64, &mut mock, base);
    pe.apply_relocations(LOADER64, &mut mock, base); // must not panic

    // Every recorded import is reversible by address — no file bytes needed.
    for (&addr, full) in pe.iat_names.iter() {
        let name = full.split_once('!').expect("dll!name").1;
        assert_eq!(pe.import_addr_to_name(addr), name);
        assert_eq!(pe.import_addr_to_dll_and_name(addr), *full);
    }

    // Resolving a missing address yields empty (not a panic).
    assert_eq!(pe.import_addr_to_name(0xdead_beef), "");
}

fn read_dword_le(mock: &Mock, addr: u64) -> u32 {
    let b0 = *mock.mem.get(&addr).expect("byte 0 written");
    let b1 = *mock.mem.get(&(addr + 1)).expect("byte 1 written");
    let b2 = *mock.mem.get(&(addr + 2)).expect("byte 2 written");
    let b3 = *mock.mem.get(&(addr + 3)).expect("byte 3 written");
    u32::from_le_bytes([b0, b1, b2, b3])
}

#[test]
fn pe32_apply_relocations_highlow_patches_target() {
    // ImageBase = 0x0040_0000, original DWORD at RVA 0x2030 = 0x0040_1234.
    // Loaded at 0x0050_0000 -> delta = 0x0010_0000, expected patched value
    // = 0x0050_1234, written to guest address 0x0050_2030.
    const IMAGE_BASE: u32 = 0x0040_0000;
    const ORIGINAL_VAL: u32 = 0x0040_1234;
    const BASE_ADDR: u32 = 0x0050_0000;
    const EXPECTED_ADDR: u64 = BASE_ADDR as u64 + 0x2030;
    const EXPECTED_VAL: u32 = ORIGINAL_VAL.wrapping_add(BASE_ADDR - IMAGE_BASE);

    let raw = build_synthetic_pe32_with_reloc(ORIGINAL_VAL);
    let pe = PE32::parse("synthetic.exe", &raw);
    let mut mock = Mock::new();

    pe.apply_relocations(&raw, &mut mock, BASE_ADDR);

    assert_eq!(read_dword_le(&mock, EXPECTED_ADDR), EXPECTED_VAL);
    // ABSOLUTE padding entry must not produce any write.
    // Sanity: there is exactly one 4-byte patched DWORD, plus no other writes.
    assert_eq!(
        mock.mem.len(),
        4,
        "only the HIGHLOW target should be written"
    );
}

#[test]
fn pe32_apply_relocations_noop_when_base_matches_image_base() {
    const IMAGE_BASE: u32 = 0x0040_0000;
    let raw = build_synthetic_pe32_with_reloc(0x0040_1234);
    let pe = PE32::parse("synthetic.exe", &raw);
    let mut mock = Mock::new();

    pe.apply_relocations(&raw, &mut mock, IMAGE_BASE);

    assert!(
        mock.mem.is_empty(),
        "zero-delta reload must not write anything"
    );
}
