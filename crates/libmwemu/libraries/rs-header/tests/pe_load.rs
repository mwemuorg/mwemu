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
    /// Module-scoped ordinal resolution: (lowercased module, ordinal) -> address.
    /// Default-zero entry means "unresolved".
    resolved_ordinals: HashMap<(String, u16), u64>,
    /// Records every ordinal lookup so tests can assert module affinity and
    /// cache reuse.
    ordinal_calls: Vec<(String, u16)>,
    /// Every guest address the loader wrote to (per-byte, 1 byte per slot).
    /// Tests use this to prove the binder does not patch unresolved slots,
    /// write past the terminator, or mutate the raw image.
    writes: Vec<u64>,
}

impl Mock {
    fn new() -> Self {
        Mock {
            mem: HashMap::new(),
            next: 0x7000_0000,
            resolved: HashMap::new(),
            resolved_ordinals: HashMap::new(),
            ordinal_calls: Vec::new(),
            writes: Vec::new(),
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
        self.writes.push(addr);
        self.write_bytes(addr, &val.to_le_bytes())
    }
    fn write_qword(&mut self, addr: u64, val: u64) -> bool {
        self.writes.push(addr);
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
    fn resolve_api_ordinal_in_module(&mut self, module: &str, ordinal: u16) -> u64 {
        let key = (module.to_ascii_lowercase(), ordinal);
        self.ordinal_calls.push(key.clone());
        self.resolved_ordinals.get(&key).copied().unwrap_or(0)
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

/// PE64 IMAGE_THUNK_DATA64 high-bit discriminator.
const IMAGE_ORDINAL_FLAG64: u64 = 0x8000_0000_0000_0000;

/// Section raw pointer used by the synthetic PE64 builder below.
const SECTION_RAW_PTR_FOR_TEST: u32 = 0x400;
/// Captured here so the OFT-zero test can assert the raw image is unchanged.
const THUNK7_FOR_TEST: u64 = 0x8000_0000_0000_0007;
/// Encoded thunk for the unresolved ordinal-9 entry.
const THUNK9_FOR_TEST: u64 = 0x8000_0000_0000_0009;
/// Build a minimal PE64 image with a single .idata section and a single
/// ImageImportDescriptor pointing at four 8-byte thunk entries. The image is
/// real enough to parse via `PE64::parse`, but the .idata section bytes are
/// entirely the test's 8-byte thunk tables. `original_first_thunk_rva` controls
/// the split-table walker; setting it to 0 forces the alternative walker.
fn build_synthetic_pe64_with_ordinal_iat(original_first_thunk_rva: u32) -> (PE64, Vec<u8>, u32) {
    // Layout constants. The import directory entry's RVA must point at the
    // raw offset of the ImageImportDescriptor. We place that descriptor at
    // the start of the .idata section so the parser sees it.
    const IMAGE_BASE: u64 = 0x4000_0000;
    const SECTION_ALIGNMENT: u32 = 0x1000;
    const FILE_ALIGNMENT: u32 = 0x200;
    const SIZE_OF_HEADERS: u32 = 0x400;
    const SIZE_OF_OPTIONAL_HEADER: u16 = 0xF0; // PE32+: 112 fixed + 16 * 8
    const NUMBER_OF_RVA_AND_SIZES: u32 = 5; // export, import, resource, exception, security
    const SECTION_RAW_SIZE: u32 = 0x200;
    const SECTION_VIRTUAL_ADDRESS: u32 = 0x1000;
    const IDATA_RVA: u32 = SECTION_VIRTUAL_ADDRESS;
    // FirstThunk (IAT) lives just past the import descriptor and OFT.
    const FIRST_THUNK_RVA: u32 = IDATA_RVA + 0x80;
    const DLL_NAME_RVA: u32 = IDATA_RVA + 0xC0;
    // Thunk entries: resolved 7, unresolved 9, resolved 11, terminator 0.
    const THUNK7: u64 = IMAGE_ORDINAL_FLAG64 | 7;
    const THUNK9: u64 = IMAGE_ORDINAL_FLAG64 | 9;
    const THUNK11: u64 = IMAGE_ORDINAL_FLAG64 | 11;
    const THUNK_TERM: u64 = 0;
    const TOTAL_SIZE: usize = (SIZE_OF_HEADERS + SECTION_RAW_SIZE) as usize;

    let mut raw = vec![0u8; TOTAL_SIZE];

    // DOS header.
    raw[0..2].copy_from_slice(&0x5A4Du16.to_le_bytes());
    const NT_OFF: usize = 0x80;
    raw[60..64].copy_from_slice(&(NT_OFF as u32).to_le_bytes());

    // NT signature.
    raw[NT_OFF..NT_OFF + 4].copy_from_slice(&0x0000_4550u32.to_le_bytes());

    // COFF file header at NT_OFF + 4.
    let fh_off = NT_OFF + 4;
    raw[fh_off..fh_off + 2].copy_from_slice(&0x8664u16.to_le_bytes()); // AMD64
    raw[fh_off + 2..fh_off + 4].copy_from_slice(&1u16.to_le_bytes()); // number_of_sections
    raw[fh_off + 16..fh_off + 18].copy_from_slice(&SIZE_OF_OPTIONAL_HEADER.to_le_bytes());
    raw[fh_off + 18..fh_off + 20].copy_from_slice(&0u16.to_le_bytes()); // characteristics

    // Optional header at NT_OFF + 24.
    let opt_off = NT_OFF + 24;
    raw[opt_off..opt_off + 2].copy_from_slice(&0x020Bu16.to_le_bytes()); // PE32+
    // Per ImageOptionalHeader64::load:
    //   +0   magic (u16)
    //   +2   major_linker_version (u8)
    //   +3   minor_linker_version (u8)
    //   +4   size_of_code (u32)
    //   +8   size_of_initialized_data (u32)
    //   +12  size_of_uninitialized_data (u32)
    //   +16  address_of_entry_point (u32)
    //   +20  base_of_code (u32)
    //   +24  image_base (u64)
    //   +32  section_alignment (u32)
    //   +36  file_alignment (u32)
    //   +40..+52  major/minor OS/subsystem/image versions
    //   +56  size_of_image (u32)
    //   +60  size_of_headers (u32)
    //   +108 number_of_rva_and_sizes (u32)
    //   +112.. data directory
    raw[opt_off + 24..opt_off + 32].copy_from_slice(&IMAGE_BASE.to_le_bytes());
    raw[opt_off + 32..opt_off + 36].copy_from_slice(&SECTION_ALIGNMENT.to_le_bytes());
    raw[opt_off + 36..opt_off + 40].copy_from_slice(&FILE_ALIGNMENT.to_le_bytes());
    raw[opt_off + 60..opt_off + 64].copy_from_slice(&SIZE_OF_HEADERS.to_le_bytes());
    // Data directory starts at opt_off + 112; entry 1 is IMPORT.
    let dd_off = opt_off + 112;
    let import_dir_off = dd_off + 1 * 8;
    // Image import descriptor sits at the very start of .idata.
    raw[import_dir_off..import_dir_off + 4].copy_from_slice(&IDATA_RVA.to_le_bytes());
    raw[import_dir_off + 4..import_dir_off + 8].copy_from_slice(&0x28u32.to_le_bytes()); // size: one descriptor + terminator
    raw[opt_off + 108..opt_off + 112].copy_from_slice(&NUMBER_OF_RVA_AND_SIZES.to_le_bytes());

    // Section header (40 bytes) at opt_off + SIZE_OF_OPTIONAL_HEADER.
    let sect_off = opt_off + SIZE_OF_OPTIONAL_HEADER as usize;
    let name = b".idata\0\0"; // 8 bytes: 6-char name + 2 null padding.
    raw[sect_off..sect_off + 8].copy_from_slice(name);
    let s2 = sect_off + 8;
    raw[s2..s2 + 4].copy_from_slice(&SECTION_RAW_SIZE.to_le_bytes()); // virtual_size == raw_size
    raw[s2 + 4..s2 + 8].copy_from_slice(&SECTION_VIRTUAL_ADDRESS.to_le_bytes());
    raw[s2 + 8..s2 + 12].copy_from_slice(&SECTION_RAW_SIZE.to_le_bytes());
    raw[s2 + 12..s2 + 16].copy_from_slice(&SECTION_RAW_PTR_FOR_TEST.to_le_bytes());
    raw[s2 + 16..s2 + 20].copy_from_slice(&0u32.to_le_bytes());
    raw[s2 + 20..s2 + 24].copy_from_slice(&0u32.to_le_bytes());
    raw[s2 + 24..s2 + 26].copy_from_slice(&0u16.to_le_bytes());
    raw[s2 + 26..s2 + 28].copy_from_slice(&0u16.to_le_bytes());
    // Characteristics: INITIALIZED_DATA | READ | WRITE | EXECUTE
    let chars: u32 = 0xC000_0040;
    raw[s2 + 28..s2 + 32].copy_from_slice(&chars.to_le_bytes());

    // .idata content. Everything is raw-offset by SECTION_RAW_PTR_FOR_TEST.
    let idata_base = SECTION_RAW_PTR_FOR_TEST as usize;

    // ImageImportDescriptor lives at the start of .idata (raw offset 0 of
    // section = IDATA_RVA). OFT | time_date | forwarder | name_ptr | first_thunk.
    let iid_off = idata_base;
    raw[iid_off..iid_off + 4].copy_from_slice(&original_first_thunk_rva.to_le_bytes());
    raw[iid_off + 4..iid_off + 8].copy_from_slice(&0u32.to_le_bytes());
    raw[iid_off + 8..iid_off + 12].copy_from_slice(&0u32.to_le_bytes());
    raw[iid_off + 12..iid_off + 16].copy_from_slice(&DLL_NAME_RVA.to_le_bytes());
    raw[iid_off + 16..iid_off + 20].copy_from_slice(&FIRST_THUNK_RVA.to_le_bytes());

    // DLL name string "ordinal.dll" at DLL_NAME_RVA.
    let dll_name_off = idata_base + (DLL_NAME_RVA - IDATA_RVA) as usize;
    let dll_name = b"ordinal.dll\0";
    raw[dll_name_off..dll_name_off + dll_name.len()].copy_from_slice(dll_name);

    // IAT/lookup thunk tables. The walker reads from the lookup (OFT or FT)
    // and writes the resolved address into the IAT. When OFT == 0 the
    // alternative walker reads from FT directly and writes to the same FT
    // slot, so we keep the same bytes in FT. When OFT != 0 we must populate
    // the OFT table separately so the lookup and destination differ.
    let iat_off = idata_base + (FIRST_THUNK_RVA - IDATA_RVA) as usize;
    for (i, val) in [THUNK7, THUNK9, THUNK11, THUNK_TERM].iter().enumerate() {
        let off = iat_off + i * 8;
        raw[off..off + 8].copy_from_slice(&val.to_le_bytes());
    }
    if original_first_thunk_rva != 0 {
        let oft_off = idata_base + (original_first_thunk_rva - IDATA_RVA) as usize;
        for (i, val) in [THUNK7, THUNK9, THUNK11, THUNK_TERM].iter().enumerate() {
            let off = oft_off + i * 8;
            raw[off..off + 8].copy_from_slice(&val.to_le_bytes());
        }
    }

    let pe = PE64::parse("synthetic.exe", &raw);
    (pe, raw, FIRST_THUNK_RVA)
}
/// the test does not panic when the binder (correctly) leaves a slot alone.
fn read_qword_le_or(mock: &Mock, addr: u64, default: u64) -> u64 {
    let mut buf = [0u8; 8];
    let mut all_missing = true;
    for (i, b) in buf.iter_mut().enumerate() {
        match mock.mem.get(&(addr + i as u64)) {
            Some(v) => {
                *b = *v;
                all_missing = false;
            }
            None => *b = 0,
        }
    }
    if all_missing {
        default
    } else {
        u64::from_le_bytes(buf)
    }
}

fn read_qword_le(mock: &Mock, addr: u64) -> u64 {
    read_qword_le_or(mock, addr, 0)
}

#[test]
fn pe64_iat_binding_ordinals_without_original_first_thunk() {
    let (mut pe, raw, first_thunk) = build_synthetic_pe64_with_ordinal_iat(0);
    let mut mock = Mock::new();
    let base = 0x4000_0000u64;

    // Configure ordinal resolutions. Ordinal 7 and 11 are exported; 9 is not.
    mock.resolved_ordinals
        .insert(("ordinal.dll".to_string(), 7), 0x8000_7000);
    mock.resolved_ordinals
        .insert(("ordinal.dll".to_string(), 11), 0x8000_B000);

    // Fixture sanity: the parser picked up the import descriptor name from
    // the raw DLL_NAME_RVA string, not via any post-parse patch.
    assert_eq!(pe.image_import_descriptor[0].name, "ordinal.dll");

    pe.iat_binding(&raw, &mut mock, base);

    let slot7 = read_qword_le(&mock, base + first_thunk as u64);
    let slot9 = read_qword_le_or(&mock, base + first_thunk as u64 + 8, THUNK9_FOR_TEST);
    let slot11 = read_qword_le(&mock, base + first_thunk as u64 + 16);
    let slot_term = read_qword_le_or(&mock, base + first_thunk as u64 + 24, 0);
    assert_eq!(slot7, 0x8000_7000, "ordinal 7 not patched");
    assert_eq!(
        slot9, THUNK9_FOR_TEST,
        "unresolved ordinal 9 must not be patched"
    );
    assert_eq!(slot11, 0x8000_B000, "ordinal 11 not patched");
    assert_eq!(slot_term, 0, "zero terminator should not be patched");
    // The binder must write exactly the two resolved IAT slots and nothing
    // else (no writes for the unresolved slot, the terminator, or anything
    // past it).
    let iat_base = base + first_thunk as u64;
    let mut expected_writes: Vec<u64> = vec![iat_base, iat_base + 16];
    expected_writes.sort();
    let mut actual_writes = mock.writes.clone();
    actual_writes.sort();
    assert_eq!(actual_writes, expected_writes, "unexpected writes");
    // The mock saw three ordinal lookups; ordinal 9 was attempted.
    assert_eq!(mock.ordinal_calls.len(), 3);
    assert!(mock.ordinal_calls.contains(&("ordinal.dll".to_string(), 7)));
    assert!(mock.ordinal_calls.contains(&("ordinal.dll".to_string(), 9)));
    assert!(
        mock.ordinal_calls
            .contains(&("ordinal.dll".to_string(), 11))
    );

    // iat_names: resolved entries keyed on resolved VA, unresolved on the
    // encoded thunk.
    assert_eq!(
        pe.iat_names.get(&0x8000_7000).map(|s| s.as_str()),
        Some("ordinal.dll!#7")
    );
    assert_eq!(
        pe.iat_names.get(&0x8000_B000).map(|s| s.as_str()),
        Some("ordinal.dll!#11")
    );
    assert_eq!(
        pe.iat_names.get(&THUNK9_FOR_TEST).map(|s| s.as_str()),
        Some("ordinal.dll!#9")
    );

    // import_addr_to_name strips the dll! prefix; import_addr_to_dll_and_name
    // keeps it.
    assert_eq!(pe.import_addr_to_name(0x8000_7000), "#7");
    assert_eq!(
        pe.import_addr_to_dll_and_name(0x8000_7000),
        "ordinal.dll!#7"
    );

    // The raw image is caller-owned and must not be mutated.
    let idata_base = SECTION_RAW_PTR_FOR_TEST as usize;
    let thunk_base = idata_base + (first_thunk - 0x1000) as usize;
    let stored7 = u64::from_le_bytes(raw[thunk_base..thunk_base + 8].try_into().unwrap());
    assert_eq!(stored7, THUNK7_FOR_TEST, "raw image was mutated by binding");
}

#[test]
fn pe64_iat_binding_ordinals_with_original_first_thunk() {
    // OFT differs from the IAT. The walker should read from OFT and patch FT.
    let oft_rva: u32 = 0x1100;
    let (mut pe, raw, first_thunk) = build_synthetic_pe64_with_ordinal_iat(oft_rva);
    let mut mock = Mock::new();
    let base = 0x4000_0000u64;
    mock.resolved_ordinals
        .insert(("ordinal.dll".to_string(), 7), 0x8000_7000);
    mock.resolved_ordinals
        .insert(("ordinal.dll".to_string(), 11), 0x8000_B000);

    assert_eq!(pe.image_import_descriptor[0].name, "ordinal.dll");

    pe.iat_binding(&raw, &mut mock, base);
    let slot7 = read_qword_le(&mock, base + first_thunk as u64);
    let slot11 = read_qword_le(&mock, base + first_thunk as u64 + 16);
    assert_eq!(slot7, 0x8000_7000, "ordinal 7 not patched in split table");
    assert_eq!(slot11, 0x8000_B000, "ordinal 11 not patched in split table");

    // Mock saw three lookups in module-scoped form.
    assert_eq!(mock.ordinal_calls.len(), 3);
    assert!(mock.ordinal_calls.contains(&("ordinal.dll".to_string(), 7)));
    assert!(mock.ordinal_calls.contains(&("ordinal.dll".to_string(), 9)));
    assert!(
        mock.ordinal_calls
            .contains(&("ordinal.dll".to_string(), 11))
    );

    // The split-table walker must patch exactly the two resolved IAT slots
    // and leave the unresolved slot and the OFT table alone.
    let iat_base = base + first_thunk as u64;
    let mut expected_writes: Vec<u64> = vec![iat_base, iat_base + 16];
    expected_writes.sort();
    let mut actual_writes = mock.writes.clone();
    actual_writes.sort();
    assert_eq!(
        actual_writes, expected_writes,
        "unexpected writes in split table"
    );
    assert_eq!(
        pe.iat_names.get(&0x8000_7000).map(|s| s.as_str()),
        Some("ordinal.dll!#7")
    );
    assert_eq!(
        pe.iat_names.get(&0x8000_B000).map(|s| s.as_str()),
        Some("ordinal.dll!#11")
    );
    assert_eq!(
        pe.iat_names.get(&THUNK9_FOR_TEST).map(|s| s.as_str()),
        Some("ordinal.dll!#9")
    );
    assert_eq!(pe.import_addr_to_name(0x8000_7000), "#7");
    assert_eq!(
        pe.import_addr_to_dll_and_name(0x8000_B000),
        "ordinal.dll!#11"
    );

    // The raw image is caller-owned: lookup and destination tables must be
    // unchanged so a later re-bind of the same descriptor still works.
    let idata_base = SECTION_RAW_PTR_FOR_TEST as usize;
    let iat_off = idata_base + (first_thunk - 0x1000) as usize;
    let oft_off = idata_base + (oft_rva - 0x1000) as usize;
    let stored_iat7 = u64::from_le_bytes(raw[iat_off..iat_off + 8].try_into().unwrap());
    let stored_oft9 = u64::from_le_bytes(raw[oft_off + 8..oft_off + 16].try_into().unwrap());
    assert_eq!(
        stored_iat7, THUNK7_FOR_TEST,
        "raw IAT was mutated by binding"
    );
    assert_eq!(
        stored_oft9, THUNK9_FOR_TEST,
        "raw OFT was mutated by binding"
    );
}

/// PE32 IMAGE_THUNK_DATA32 high-bit discriminator.
const IMAGE_ORDINAL_FLAG32_PE32: u32 = 0x8000_0000;

/// Section raw pointer for the PE32 fixture.
const SECTION_RAW_PTR_PE32: u32 = 0x200;
/// Encoded thunk for the resolved ordinal-7 entry.
const THUNK7_PE32: u32 = IMAGE_ORDINAL_FLAG32_PE32 | 7;
/// Encoded thunk for the unresolved ordinal-9 entry.
const THUNK9_PE32: u32 = IMAGE_ORDINAL_FLAG32_PE32 | 9;
/// Encoded thunk for the resolved ordinal-11 entry.
const THUNK11_PE32: u32 = IMAGE_ORDINAL_FLAG32_PE32 | 11;

/// Build a minimal PE32 image with one `.idata` section containing a single
/// ImageImportDescriptor pointing at four 4-byte thunk entries. `original_first_thunk`
/// controls the split-table walker (OFT != 0); setting it to 0 forces the
/// alternative walker that reads from FT directly.
fn build_synthetic_pe32_with_ordinal_iat(original_first_thunk_rva: u32) -> (PE32, Vec<u8>, u32) {
    const IMAGE_BASE: u32 = 0x0040_0000;
    const SECTION_ALIGNMENT: u32 = 0x1000;
    const FILE_ALIGNMENT: u32 = 0x200;
    const SIZE_OF_HEADERS: u32 = 0x200;
    const SIZE_OF_OPTIONAL_HEADER: u16 = 0xE0; // PE32: 96 fixed + 16 dirs * 8
    const NUMBER_OF_RVA_AND_SIZES: u32 = 5; // export, import, resource, exception, security
    const SECTION_RAW_SIZE: u32 = 0x200;
    const SECTION_VIRTUAL_ADDRESS: u32 = 0x2000;
    const IDATA_RVA: u32 = SECTION_VIRTUAL_ADDRESS;
    const FIRST_THUNK_RVA: u32 = IDATA_RVA + 0x80;
    const DLL_NAME_RVA: u32 = IDATA_RVA + 0xC0;
    const TOTAL_SIZE: usize = (SIZE_OF_HEADERS + SECTION_RAW_SIZE) as usize;

    let mut raw = vec![0u8; TOTAL_SIZE];

    // DOS header.
    raw[0..2].copy_from_slice(&0x5A4Du16.to_le_bytes());
    raw[60..64].copy_from_slice(&0x80u32.to_le_bytes());
    let nt_off = 0x80usize;

    // NT signature.
    raw[nt_off..nt_off + 4].copy_from_slice(&0x0000_4550u32.to_le_bytes());

    // COFF file header at nt_off + 4.
    let fh_off = nt_off + 4;
    raw[fh_off..fh_off + 2].copy_from_slice(&0x014Cu16.to_le_bytes()); // i386
    raw[fh_off + 2..fh_off + 4].copy_from_slice(&1u16.to_le_bytes());
    raw[fh_off + 16..fh_off + 18].copy_from_slice(&SIZE_OF_OPTIONAL_HEADER.to_le_bytes());

    // Optional header at nt_off + 24.
    let opt_off = nt_off + 24;
    raw[opt_off..opt_off + 2].copy_from_slice(&0x010Bu16.to_le_bytes()); // PE32
    // Per ImageOptionalHeader (PE32) layout: image_base at +28, section_alignment +32,
    // file_alignment +36, size_of_headers +60, number_of_rva_and_sizes +92,
    // data_directory +96.
    raw[opt_off + 28..opt_off + 32].copy_from_slice(&IMAGE_BASE.to_le_bytes());
    raw[opt_off + 32..opt_off + 36].copy_from_slice(&SECTION_ALIGNMENT.to_le_bytes());
    raw[opt_off + 36..opt_off + 40].copy_from_slice(&FILE_ALIGNMENT.to_le_bytes());
    raw[opt_off + 60..opt_off + 64].copy_from_slice(&SIZE_OF_HEADERS.to_le_bytes());
    raw[opt_off + 92..opt_off + 96].copy_from_slice(&NUMBER_OF_RVA_AND_SIZES.to_le_bytes());

    // Data directory: entry 1 is IMPORT.
    let dd_off = opt_off + 96;
    let import_dir_off = dd_off + 1 * 8;
    raw[import_dir_off..import_dir_off + 4].copy_from_slice(&IDATA_RVA.to_le_bytes());
    raw[import_dir_off + 4..import_dir_off + 8].copy_from_slice(&0x28u32.to_le_bytes());

    // Section header at opt_off + SIZE_OF_OPTIONAL_HEADER.
    let sect_off = opt_off + SIZE_OF_OPTIONAL_HEADER as usize;
    let name = b".idata\0\0";
    raw[sect_off..sect_off + 8].copy_from_slice(name);
    let s2 = sect_off + 8;
    raw[s2..s2 + 4].copy_from_slice(&SECTION_RAW_SIZE.to_le_bytes()); // virtual_size == raw_size
    raw[s2 + 4..s2 + 8].copy_from_slice(&SECTION_VIRTUAL_ADDRESS.to_le_bytes());
    raw[s2 + 8..s2 + 12].copy_from_slice(&SECTION_RAW_SIZE.to_le_bytes());
    raw[s2 + 12..s2 + 16].copy_from_slice(&SECTION_RAW_PTR_PE32.to_le_bytes());

    // .idata content.
    let idata_base = SECTION_RAW_PTR_PE32 as usize;

    // ImageImportDescriptor lives at the start of .idata. OFT/date/fwd/name/FT.
    let iid_off = idata_base;
    raw[iid_off..iid_off + 4].copy_from_slice(&original_first_thunk_rva.to_le_bytes());
    raw[iid_off + 4..iid_off + 8].copy_from_slice(&0u32.to_le_bytes());
    raw[iid_off + 8..iid_off + 12].copy_from_slice(&0u32.to_le_bytes());
    raw[iid_off + 12..iid_off + 16].copy_from_slice(&DLL_NAME_RVA.to_le_bytes());
    raw[iid_off + 16..iid_off + 20].copy_from_slice(&FIRST_THUNK_RVA.to_le_bytes());

    // DLL name string.
    let dll_name_off = idata_base + (DLL_NAME_RVA - IDATA_RVA) as usize;
    let dll_name = b"ordinal.dll\0";
    raw[dll_name_off..dll_name_off + dll_name.len()].copy_from_slice(dll_name);

    // IAT (and OFT) thunk tables: resolved 7, unresolved 9, resolved 11, terminator 0.
    let iat_off = idata_base + (FIRST_THUNK_RVA - IDATA_RVA) as usize;
    for (i, val) in [THUNK7_PE32, THUNK9_PE32, THUNK11_PE32, 0u32]
        .iter()
        .enumerate()
    {
        let off = iat_off + i * 4;
        raw[off..off + 4].copy_from_slice(&val.to_le_bytes());
    }
    if original_first_thunk_rva != 0 {
        let oft_off = idata_base + (original_first_thunk_rva - IDATA_RVA) as usize;
        for (i, val) in [THUNK7_PE32, THUNK9_PE32, THUNK11_PE32, 0u32]
            .iter()
            .enumerate()
        {
            let off = oft_off + i * 4;
            raw[off..off + 4].copy_from_slice(&val.to_le_bytes());
        }
    }

    let pe = PE32::parse("synthetic.exe", &raw);
    (pe, raw, FIRST_THUNK_RVA)
}

#[test]
fn pe32_iat_binding_ordinals_without_original_first_thunk() {
    let (mut pe, raw, first_thunk) = build_synthetic_pe32_with_ordinal_iat(0);
    let mut mock = Mock::new();
    let base = 0x0050_0000u32;

    mock.resolved_ordinals
        .insert(("ordinal.dll".to_string(), 7), 0x8000_7000);
    mock.resolved_ordinals
        .insert(("ordinal.dll".to_string(), 11), 0x8000_B000);

    // Fixture sanity: parser read the import descriptor name from the raw
    // DLL_NAME_RVA string.
    assert_eq!(pe.image_import_descriptor[0].name, "ordinal.dll");

    pe.iat_binding(&raw, &mut mock, base);

    // Resolved slots patched; unresolved slot left alone; terminator untouched.
    let slot7 = read_dword_le_or(&mock, base as u64 + first_thunk as u64, 0);
    let slot9 = read_dword_le_or(&mock, base as u64 + first_thunk as u64 + 4, THUNK9_PE32);
    let slot11 = read_dword_le_or(&mock, base as u64 + first_thunk as u64 + 8, 0);
    let slot_term = read_dword_le_or(&mock, base as u64 + first_thunk as u64 + 12, 0);
    assert_eq!(slot7, 0x8000_7000, "ordinal 7 not patched");
    assert_eq!(
        slot9, THUNK9_PE32,
        "unresolved ordinal 9 must not be patched"
    );
    assert_eq!(slot11, 0x8000_B000, "ordinal 11 not patched");
    assert_eq!(slot_term, 0, "zero terminator should not be patched");

    // Binder writes exactly the two resolved IAT slots — not the unresolved
    // one, not the terminator, not anything past it.
    let iat_base = base as u64 + first_thunk as u64;
    let mut expected_writes: Vec<u64> = vec![iat_base, iat_base + 8];
    expected_writes.sort();
    let mut actual_writes = mock.writes.clone();
    actual_writes.sort();
    assert_eq!(
        actual_writes, expected_writes,
        "unexpected writes in PE32 IAT"
    );

    // Three ordinal lookups in module-scoped form.
    assert_eq!(mock.ordinal_calls.len(), 3);
    assert!(mock.ordinal_calls.contains(&("ordinal.dll".to_string(), 7)));
    assert!(mock.ordinal_calls.contains(&("ordinal.dll".to_string(), 9)));
    assert!(
        mock.ordinal_calls
            .contains(&("ordinal.dll".to_string(), 11))
    );

    // iat_names entries.
    assert_eq!(
        pe.iat_names.get(&0x8000_7000).map(|s| s.as_str()),
        Some("ordinal.dll!#7")
    );
    assert_eq!(
        pe.iat_names.get(&0x8000_B000).map(|s| s.as_str()),
        Some("ordinal.dll!#11")
    );
    assert_eq!(
        pe.iat_names.get(&THUNK9_PE32).map(|s| s.as_str()),
        Some("ordinal.dll!#9")
    );
    assert_eq!(pe.import_addr_to_name(0x8000_7000), "#7");
    assert_eq!(
        pe.import_addr_to_dll_and_name(0x8000_7000),
        "ordinal.dll!#7"
    );

    // Raw image unchanged: caller owns it.
    let idata_base = SECTION_RAW_PTR_PE32 as usize;
    let iat_off = idata_base + (first_thunk - 0x2000) as usize;
    let stored7 = u32::from_le_bytes(raw[iat_off..iat_off + 4].try_into().unwrap());
    assert_eq!(stored7, THUNK7_PE32, "raw image was mutated by binding");
}

#[test]
fn pe32_iat_binding_ordinals_with_original_first_thunk() {
    // OFT differs from the IAT. The walker should read from OFT and patch FT.
    let oft_rva: u32 = 0x2100;
    let (mut pe, raw, first_thunk) = build_synthetic_pe32_with_ordinal_iat(oft_rva);
    let mut mock = Mock::new();
    let base = 0x0050_0000u32;
    mock.resolved_ordinals
        .insert(("ordinal.dll".to_string(), 7), 0x8000_7000);
    mock.resolved_ordinals
        .insert(("ordinal.dll".to_string(), 11), 0x8000_B000);

    assert_eq!(pe.image_import_descriptor[0].name, "ordinal.dll");

    pe.iat_binding(&raw, &mut mock, base);

    // Resolved slots patched.
    let slot7 = read_dword_le_or(&mock, base as u64 + first_thunk as u64, 0);
    let slot11 = read_dword_le_or(&mock, base as u64 + first_thunk as u64 + 8, 0);
    assert_eq!(slot7, 0x8000_7000, "ordinal 7 not patched in split table");
    assert_eq!(slot11, 0x8000_B000, "ordinal 11 not patched in split table");

    // Unresolved slot left alone.
    let slot9 = read_dword_le_or(&mock, base as u64 + first_thunk as u64 + 4, THUNK9_PE32);
    assert_eq!(
        slot9, THUNK9_PE32,
        "unresolved ordinal 9 must not be patched"
    );

    let iat_base = base as u64 + first_thunk as u64;
    let mut expected_writes: Vec<u64> = vec![iat_base, iat_base + 8];
    expected_writes.sort();
    let mut actual_writes = mock.writes.clone();
    actual_writes.sort();
    assert_eq!(
        actual_writes, expected_writes,
        "unexpected writes in PE32 split table"
    );

    // iat_names.
    assert_eq!(
        pe.iat_names.get(&0x8000_7000).map(|s| s.as_str()),
        Some("ordinal.dll!#7")
    );
    assert_eq!(
        pe.iat_names.get(&0x8000_B000).map(|s| s.as_str()),
        Some("ordinal.dll!#11")
    );
    assert_eq!(
        pe.iat_names.get(&THUNK9_PE32).map(|s| s.as_str()),
        Some("ordinal.dll!#9")
    );

    // Raw image unchanged: both lookup and destination tables.
    let idata_base = SECTION_RAW_PTR_PE32 as usize;
    let iat_off = idata_base + (first_thunk - 0x2000) as usize;
    let oft_off = idata_base + (oft_rva - 0x2000) as usize;
    let stored_iat7 = u32::from_le_bytes(raw[iat_off..iat_off + 4].try_into().unwrap());
    let stored_oft9 = u32::from_le_bytes(raw[oft_off + 4..oft_off + 8].try_into().unwrap());
    assert_eq!(stored_iat7, THUNK7_PE32, "raw IAT was mutated by binding");
    assert_eq!(stored_oft9, THUNK9_PE32, "raw OFT was mutated by binding");
}

/// Read a 32-bit little-endian value from the mock or `default` if the slot
/// was never written. Mirrors `read_qword_le_or` for the PE32 binder.
fn read_dword_le_or(mock: &Mock, addr: u64, default: u32) -> u32 {
    let mut buf = [0u8; 4];
    let mut all_missing = true;
    for (i, b) in buf.iter_mut().enumerate() {
        match mock.mem.get(&(addr + i as u64)) {
            Some(v) => {
                *b = *v;
                all_missing = false;
            }
            None => *b = 0,
        }
    }
    if all_missing {
        default
    } else {
        u32::from_le_bytes(buf)
    }
}

/// the IAT, OFT, import descriptor, and the delay-load thunk tables so the
/// delay parser can map every RVA it sees.
const SECTION_RAW_PTR_PE32_DELAY: u32 = 0x200;
const SECTION_VIRTUAL_ADDRESS_PE32_DELAY: u32 = 0x2000;
const DELAY_DIR_RVA: u32 = 0x2080;
const DELAY_DLL_NAME_RVA: u32 = 0x20C0;
const DELAY_NAME_TABLE_RVA: u32 = 0x2100;
const DELAY_ADDRESS_TABLE_RVA: u32 = 0x2180;

/// Build a synthetic PE32 image with one `.idata` section containing:
///   - one ImageImportDescriptor with a 4-byte terminator,
///   - one DelayLoadDirectory pointing at separate name/address tables.
/// The delay tables use ordinal thunks (resolved 7, unresolved 9, resolved 11,
/// terminator 0). `name_table` holds the thunks; `address_table` is the
/// delay-IAT that the binder must patch. `bound_delay_import_table` is left
/// zero so we exercise the unbound descriptor path the previous fix
/// regressed.
fn build_synthetic_pe32_with_delay_ordinal_iat() -> (PE32, Vec<u8>, u32) {
    const IMAGE_BASE: u32 = 0x0040_0000;
    const SECTION_ALIGNMENT: u32 = 0x1000;
    const FILE_ALIGNMENT: u32 = 0x200;
    const SIZE_OF_HEADERS: u32 = 0x300;
    const SIZE_OF_OPTIONAL_HEADER: u16 = 0xE0; // PE32: 96 fixed + 16 dirs * 8
    const NUMBER_OF_RVA_AND_SIZES: u32 = 14; // include DELAY_LOAD (index 13)
    const SECTION_RAW_SIZE: u32 = 0x300;
    const TOTAL_SIZE: usize = (SIZE_OF_HEADERS + SECTION_RAW_SIZE) as usize;
    const IDATA_RVA: u32 = SECTION_VIRTUAL_ADDRESS_PE32_DELAY;

    let mut raw = vec![0u8; TOTAL_SIZE];

    // DOS header.
    raw[0..2].copy_from_slice(&0x5A4Du16.to_le_bytes());
    raw[60..64].copy_from_slice(&0x80u32.to_le_bytes());
    let nt_off = 0x80usize;

    // NT signature.
    raw[nt_off..nt_off + 4].copy_from_slice(&0x0000_4550u32.to_le_bytes());

    // COFF file header at nt_off + 4.
    let fh_off = nt_off + 4;
    raw[fh_off..fh_off + 2].copy_from_slice(&0x014Cu16.to_le_bytes()); // i386
    raw[fh_off + 2..fh_off + 4].copy_from_slice(&1u16.to_le_bytes());
    raw[fh_off + 16..fh_off + 18].copy_from_slice(&SIZE_OF_OPTIONAL_HEADER.to_le_bytes());

    // Optional header at nt_off + 24.
    let opt_off = nt_off + 24;
    raw[opt_off..opt_off + 2].copy_from_slice(&0x010Bu16.to_le_bytes()); // PE32
    raw[opt_off + 28..opt_off + 32].copy_from_slice(&IMAGE_BASE.to_le_bytes());
    raw[opt_off + 32..opt_off + 36].copy_from_slice(&SECTION_ALIGNMENT.to_le_bytes());
    raw[opt_off + 36..opt_off + 40].copy_from_slice(&FILE_ALIGNMENT.to_le_bytes());
    raw[opt_off + 60..opt_off + 64].copy_from_slice(&SIZE_OF_HEADERS.to_le_bytes());
    raw[opt_off + 92..opt_off + 96].copy_from_slice(&NUMBER_OF_RVA_AND_SIZES.to_le_bytes());

    // Data directory: entry 1 is IMPORT (empty), entry 13 is DELAY_LOAD.
    let dd_off = opt_off + 96;
    let delay_dir_off = dd_off + 13 * 8;
    raw[delay_dir_off..delay_dir_off + 4].copy_from_slice(&DELAY_DIR_RVA.to_le_bytes());
    raw[delay_dir_off + 4..delay_dir_off + 8].copy_from_slice(&0x20u32.to_le_bytes()); // one descriptor

    // Section header at opt_off + SIZE_OF_OPTIONAL_HEADER.
    let sect_off = opt_off + SIZE_OF_OPTIONAL_HEADER as usize;
    raw[sect_off..sect_off + 8].copy_from_slice(b".idata\0\0");
    let s2 = sect_off + 8;
    raw[s2..s2 + 4].copy_from_slice(&SECTION_RAW_SIZE.to_le_bytes()); // virtual_size
    raw[s2 + 4..s2 + 8].copy_from_slice(&SECTION_VIRTUAL_ADDRESS_PE32_DELAY.to_le_bytes());
    raw[s2 + 8..s2 + 12].copy_from_slice(&SECTION_RAW_SIZE.to_le_bytes());
    raw[s2 + 12..s2 + 16].copy_from_slice(&SECTION_RAW_PTR_PE32_DELAY.to_le_bytes());

    // .idata content.
    let idata_base = SECTION_RAW_PTR_PE32_DELAY as usize;

    // ImageImportDescriptor lives at the start of .idata. Empty import
    // descriptor + terminator, so iat_binding is a no-op and delay_load_binding
    // is the focus of the test.
    let iid_off = idata_base + (IDATA_RVA - SECTION_VIRTUAL_ADDRESS_PE32_DELAY) as usize;
    raw[iid_off..iid_off + 4].copy_from_slice(&0u32.to_le_bytes());
    raw[iid_off + 4..iid_off + 8].copy_from_slice(&0u32.to_le_bytes());
    raw[iid_off + 8..iid_off + 12].copy_from_slice(&0u32.to_le_bytes());
    raw[iid_off + 12..iid_off + 16].copy_from_slice(&0u32.to_le_bytes());
    raw[iid_off + 16..iid_off + 20].copy_from_slice(&0u32.to_le_bytes());
    // Terminator descriptor at iid_off + 20.
    raw[iid_off + 20..iid_off + 24].copy_from_slice(&0u32.to_le_bytes());

    // DelayLoadDirectory at DELAY_DIR_RVA.
    let dld_off = idata_base + (DELAY_DIR_RVA - SECTION_VIRTUAL_ADDRESS_PE32_DELAY) as usize;
    raw[dld_off..dld_off + 4].copy_from_slice(&0u32.to_le_bytes()); // attributes
    raw[dld_off + 4..dld_off + 8].copy_from_slice(&DELAY_DLL_NAME_RVA.to_le_bytes()); // name_ptr
    raw[dld_off + 8..dld_off + 12].copy_from_slice(&1u32.to_le_bytes()); // handle (must be non-zero)
    raw[dld_off + 12..dld_off + 16].copy_from_slice(&DELAY_ADDRESS_TABLE_RVA.to_le_bytes()); // address_table (delay IAT)
    raw[dld_off + 16..dld_off + 20].copy_from_slice(&DELAY_NAME_TABLE_RVA.to_le_bytes()); // name_table
    raw[dld_off + 20..dld_off + 24].copy_from_slice(&0u32.to_le_bytes()); // bound_delay_import_table (must be zero)
    raw[dld_off + 24..dld_off + 28].copy_from_slice(&0u32.to_le_bytes()); // unload
    raw[dld_off + 28..dld_off + 32].copy_from_slice(&0u32.to_le_bytes()); // tstamp

    // DLL name string.
    let dll_name_off =
        idata_base + (DELAY_DLL_NAME_RVA - SECTION_VIRTUAL_ADDRESS_PE32_DELAY) as usize;
    let dll_name = b"ordinal.dll\0";
    raw[dll_name_off..dll_name_off + dll_name.len()].copy_from_slice(dll_name);

    // Name-table thunks (input).
    let name_off =
        idata_base + (DELAY_NAME_TABLE_RVA - SECTION_VIRTUAL_ADDRESS_PE32_DELAY) as usize;
    for (i, val) in [THUNK7_PE32, THUNK9_PE32, THUNK11_PE32, 0u32]
        .iter()
        .enumerate()
    {
        let off = name_off + i * 4;
        raw[off..off + 4].copy_from_slice(&val.to_le_bytes());
    }

    // Delay-IAT slots (output). Filled with sentinel zeros so a successful
    // patch is detectable.
    let iat_off =
        idata_base + (DELAY_ADDRESS_TABLE_RVA - SECTION_VIRTUAL_ADDRESS_PE32_DELAY) as usize;
    for i in 0..4 {
        raw[iat_off + i * 4..iat_off + i * 4 + 4].copy_from_slice(&0xDEAD_BEEFu32.to_le_bytes());
    }

    let pe = PE32::parse("synthetic.exe", &raw);
    (pe, raw, DELAY_ADDRESS_TABLE_RVA)
}

#[test]
fn pe32_delay_load_binding_uses_address_table_for_unbound_descriptor() {
    // bound_delay_import_table is zero (unbound); the binder must still patch
    // the delay-IAT (address_table) for resolved ordinals. This guards the
    // P1 regression where unbound descriptors were skipped entirely.
    let (mut pe, raw, address_table_rva) = build_synthetic_pe32_with_delay_ordinal_iat();
    let mut mock = Mock::new();
    let base = 0x0060_0000u32;

    mock.resolved_ordinals
        .insert(("ordinal.dll".to_string(), 7), 0x9000_7000);
    mock.resolved_ordinals
        .insert(("ordinal.dll".to_string(), 11), 0x9000_B000);
    assert_eq!(pe.delay_load_dir.len(), 1, "expected one delay descriptor");
    assert_eq!(pe.delay_load_dir[0].name, "ordinal.dll");
    assert_eq!(pe.delay_load_dir[0].address_table, DELAY_ADDRESS_TABLE_RVA);
    assert_eq!(pe.delay_load_dir[0].name_table, DELAY_NAME_TABLE_RVA);
    assert_eq!(pe.delay_load_dir[0].bound_delay_import_table, 0);

    // Resolved slots patched in the delay-IAT; unresolved and terminator left
    // alone. The binder must NOT touch the bound_delay_import_table slot
    // (which is zero and unmapped here).
    pe.delay_load_binding(&raw, &mut mock, base);
    let iat_base = base as u64 + address_table_rva as u64;
    let slot7 = read_dword_le_or(&mock, iat_base, 0xDEAD_BEEF);
    let slot9 = read_dword_le_or(&mock, iat_base + 4, 0xDEAD_BEEF);
    let slot11 = read_dword_le_or(&mock, iat_base + 8, 0xDEAD_BEEF);
    let slot_term = read_dword_le_or(&mock, iat_base + 12, 0xDEAD_BEEF);
    assert_eq!(slot7, 0x9000_7000, "delay ordinal 7 not patched");
    assert_eq!(
        slot9, 0xDEAD_BEEF,
        "unresolved delay ordinal 9 must not be patched"
    );
    assert_eq!(slot11, 0x9000_B000, "delay ordinal 11 not patched");
    assert_eq!(
        slot_term, 0xDEAD_BEEF,
        "delay terminator must not be patched"
    );

    // Exactly two writes: the resolved delay-IAT slots. The name table is
    // read-only input and must not be written.
    let mut expected_writes: Vec<u64> = vec![iat_base, iat_base + 8];
    expected_writes.sort();
    let mut actual_writes = mock.writes.clone();
    actual_writes.sort();
    assert_eq!(actual_writes, expected_writes, "unexpected delay writes");
    // Three ordinal lookups in module-scoped form.
    assert_eq!(mock.ordinal_calls.len(), 3);
    assert!(mock.ordinal_calls.contains(&("ordinal.dll".to_string(), 7)));
    assert!(mock.ordinal_calls.contains(&("ordinal.dll".to_string(), 9)));
    assert!(
        mock.ordinal_calls
            .contains(&("ordinal.dll".to_string(), 11))
    );

    // iat_names entries for resolved and unresolved ordinals.
    assert_eq!(
        pe.iat_names.get(&0x9000_7000).map(|s| s.as_str()),
        Some("ordinal.dll!#7")
    );
    assert_eq!(
        pe.iat_names.get(&0x9000_B000).map(|s| s.as_str()),
        Some("ordinal.dll!#11")
    );
    assert_eq!(
        pe.iat_names.get(&THUNK9_PE32).map(|s| s.as_str()),
        Some("ordinal.dll!#9")
    );

    // Name-table raw bytes are unchanged.
    let idata_base = SECTION_RAW_PTR_PE32_DELAY as usize;
    let name_off =
        idata_base + (DELAY_NAME_TABLE_RVA - SECTION_VIRTUAL_ADDRESS_PE32_DELAY) as usize;
    let stored7 = u32::from_le_bytes(raw[name_off..name_off + 4].try_into().unwrap());
    assert_eq!(
        stored7, THUNK7_PE32,
        "raw name table was mutated by delay binding"
    );
}
