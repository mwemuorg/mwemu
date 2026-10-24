use crate::api::windows::export_index::{self, ModuleExportIndex};
use crate::emu::Emu;
use crate::maps::mem64::Permission;
use rs_header::pe::export_index::{ExportIndexData, ExportTarget, NamedExport};

const PAGE: u32 = 0x1000;
const HEADERS_RVA: u32 = 0;
const TEXT_RVA: u32 = PAGE;
const FUNC_SLOT: u32 = 4;

fn align_page(sz: u32) -> u32 {
    (sz + PAGE - 1) & !(PAGE - 1)
}

impl Emu {
    /// Create a minimal in-memory PE32 stub for a system DLL that is missing
    /// from disk. The stub has valid PE headers and an export directory so that
    /// PEB-walking shellcode (ReflectiveLoader, etc.) can resolve API names by
    /// hash against it. The .text section addresses are in lib32 space so
    /// mwemu's WinAPI gateway intercepts calls to them.
    ///
    /// Returns the base address of the stub (0 on failure).
    pub fn create_stub_pe32(&mut self, dll_name: &str, exports: &[&str]) -> u32 {
        let n = exports.len() as u32;

        let map_name = dll_name
            .rsplit('/')
            .next()
            .unwrap_or(dll_name)
            .rsplit('\\')
            .next()
            .unwrap_or(dll_name)
            .trim_end_matches(".dll")
            .trim_end_matches(".DLL")
            .to_string();

        // Compute sizes: .text needs FUNC_SLOT bytes per export,
        // .edata needs the export directory (40 bytes), three tables
        // (N*4 + N*4 + N*2 = N*10), plus all name strings.
        let text_size = align_page(n * FUNC_SLOT);
        let edata_rva = TEXT_RVA + text_size;

        let dll_name_str = format!("{}.dll", map_name);
        let strings_len: u32 =
            dll_name_str.len() as u32 + 1 + exports.iter().map(|s| s.len() as u32 + 1).sum::<u32>();
        let edata_raw = 40 + n * 10 + strings_len;
        let edata_size = align_page(edata_raw);

        let size_of_image = PAGE + text_size + edata_size; // headers + .text + .edata
        let base = match self.maps.lib32_alloc(size_of_image as u64) {
            Some(b) => b as u32,
            None => {
                log::error!("create_stub_pe32: out of lib32 space for {}", dll_name);
                return 0;
            }
        };

        // --- 1. PE headers (.pe map) ---
        let pe_map = match self.maps.create_map(
            &format!("{}.pe", map_name),
            base as u64,
            PAGE as u64,
            Permission::READ_WRITE,
        ) {
            Ok(m) => m,
            Err(e) => {
                log::error!("cannot create stub pe map: {}", e);
                return 0;
            }
        };

        let mut hdr = vec![0u8; PAGE as usize];
        // DOS header
        hdr[0] = 0x4d;
        hdr[1] = 0x5a;
        w32(&mut hdr, 0x3c, 0x80); // e_lfanew

        // PE signature
        w32(&mut hdr, 0x80, 0x0000_4550);

        // IMAGE_FILE_HEADER
        w16(&mut hdr, 0x84, 0x014c); // Machine = i386
        w16(&mut hdr, 0x86, 2); // NumberOfSections
        w16(&mut hdr, 0x94, 0xe0); // SizeOfOptionalHeader
        w16(&mut hdr, 0x96, 0x2102); // DLL | EXEC | 32BIT

        // IMAGE_OPTIONAL_HEADER32
        w16(&mut hdr, 0x98, 0x10b); // Magic = PE32
        w32(&mut hdr, 0x9c, text_size); // SizeOfCode
        w32(&mut hdr, 0xac, TEXT_RVA); // BaseOfCode
        w32(&mut hdr, 0xb0, edata_rva); // BaseOfData
        w32(&mut hdr, 0xb4, base); // ImageBase
        w32(&mut hdr, 0xb8, PAGE); // SectionAlignment
        w32(&mut hdr, 0xbc, 0x200); // FileAlignment
        w32(&mut hdr, 0xd0, size_of_image); // SizeOfImage
        w32(&mut hdr, 0xd4, 0x200); // SizeOfHeaders
        w16(&mut hdr, 0xdc, 3); // Subsystem = CONSOLE
        w32(&mut hdr, 0xf4, 16); // NumberOfRvaAndSizes

        // DataDirectory[0] = Export
        w32(&mut hdr, 0xf8, edata_rva);
        w32(&mut hdr, 0xfc, edata_raw);

        // Section header 0: .text
        let s0 = 0x178;
        copy_name(&mut hdr, s0, b".text\0\0\0");
        w32(&mut hdr, s0 + 8, text_size); // VirtualSize
        w32(&mut hdr, s0 + 12, TEXT_RVA); // VirtualAddress
        w32(&mut hdr, s0 + 16, text_size); // SizeOfRawData
        w32(&mut hdr, s0 + 20, TEXT_RVA); // PointerToRawData (identity-mapped)
        w32(&mut hdr, s0 + 36, 0x6000_0020); // CODE|EXEC|READ

        // Section header 1: .edata
        let s1 = s0 + 40;
        copy_name(&mut hdr, s1, b".edata\0\0");
        w32(&mut hdr, s1 + 8, edata_size); // VirtualSize
        w32(&mut hdr, s1 + 12, edata_rva); // VirtualAddress
        w32(&mut hdr, s1 + 16, edata_size); // SizeOfRawData
        w32(&mut hdr, s1 + 20, edata_rva); // PointerToRawData
        w32(&mut hdr, s1 + 36, 0x4000_0040); // INIT_DATA|READ

        pe_map.memcpy(&hdr, hdr.len());

        // --- 2. .text map: RET stub at each function slot ---
        let text_map = match self.maps.create_map(
            &format!("{}.text", map_name),
            base as u64 + TEXT_RVA as u64,
            text_size as u64,
            Permission::READ_WRITE_EXECUTE,
        ) {
            Ok(m) => m,
            Err(e) => {
                log::error!("cannot create stub text map: {}", e);
                return 0;
            }
        };
        for i in 0..n {
            text_map.write_byte(base as u64 + TEXT_RVA as u64 + (i * FUNC_SLOT) as u64, 0xc3);
        }

        // --- 3. .edata map: export directory ---
        let edata_base = base as u64 + edata_rva as u64;
        let edata_map = match self.maps.create_map(
            &format!("{}.edata", map_name),
            edata_base,
            edata_size as u64,
            Permission::READ_WRITE,
        ) {
            Ok(m) => m,
            Err(e) => {
                log::error!("cannot create stub edata map: {}", e);
                return 0;
            }
        };

        let mut ed = vec![0u8; edata_size as usize];

        let func_tbl_rva = edata_rva + 40;
        let name_tbl_rva = func_tbl_rva + n * 4;
        let ord_tbl_rva = name_tbl_rva + n * 4;
        let str_base_rva = ord_tbl_rva + n * 2;

        // DLL name string
        let mut str_off = (str_base_rva - edata_rva) as usize;
        let dll_name_rva = edata_rva + str_off as u32;
        for b in dll_name_str.bytes() {
            ed[str_off] = b;
            str_off += 1;
        }
        ed[str_off] = 0;
        str_off += 1;

        // Export name strings
        let mut name_rvas = Vec::with_capacity(exports.len());
        for name in exports {
            name_rvas.push(edata_rva + str_off as u32);
            for b in name.bytes() {
                ed[str_off] = b;
                str_off += 1;
            }
            ed[str_off] = 0;
            str_off += 1;
        }

        // Export directory header
        w32(&mut ed, 0x0c, dll_name_rva); // Name
        w32(&mut ed, 0x10, 1); // Base
        w32(&mut ed, 0x14, n); // NumberOfFunctions
        w32(&mut ed, 0x18, n); // NumberOfNames
        w32(&mut ed, 0x1c, func_tbl_rva); // AddressOfFunctions
        w32(&mut ed, 0x20, name_tbl_rva); // AddressOfNames
        w32(&mut ed, 0x24, ord_tbl_rva); // AddressOfNameOrdinals

        // Function RVA table
        let fo = (func_tbl_rva - edata_rva) as usize;
        for i in 0..n as usize {
            w32(&mut ed, fo + i * 4, TEXT_RVA + (i as u32) * FUNC_SLOT);
        }

        // Name RVA table
        let no = (name_tbl_rva - edata_rva) as usize;
        for (i, &rva) in name_rvas.iter().enumerate().take(n as usize) {
            w32(&mut ed, no + i * 4, rva);
        }

        // Ordinal table
        let oo = (ord_tbl_rva - edata_rva) as usize;
        for i in 0..n as usize {
            w16(&mut ed, oo + i * 2, i as u16);
        }

        edata_map.memcpy(&ed, ed.len());

        // --- 4. Host-side export index for gateway dispatch ---
        let mut ordinal_targets = Vec::with_capacity(exports.len());
        let mut named_exports = Vec::with_capacity(exports.len());
        for (i, name) in exports.iter().enumerate() {
            ordinal_targets.push(Some(ExportTarget::Direct {
                rva: TEXT_RVA + (i as u32) * FUNC_SLOT,
            }));
            named_exports.push(NamedExport {
                name: name.to_string(),
                ordinal_index: i as u32,
            });
        }
        let parsed = ExportIndexData {
            export_base: 1,
            number_of_functions: exports.len() as u32,
            ordinal_targets,
            named_exports,
        };
        let index = ModuleExportIndex::from_parsed(
            map_name.clone(),
            export_index::normalize_module_name(&map_name),
            base as u64,
            &parsed,
        );
        self.export_indexes.register(index);

        log::info!(
            "created PE32 stub for {} at 0x{:x} ({} exports)",
            dll_name,
            base,
            exports.len()
        );

        base
    }

    pub fn create_stub_pe64(&mut self, dll_name: &str, exports: &[&str]) -> u64 {
        let n = exports.len() as u32;

        let map_name = dll_name
            .rsplit('/')
            .next()
            .unwrap_or(dll_name)
            .rsplit('\\')
            .next()
            .unwrap_or(dll_name)
            .trim_end_matches(".dll")
            .trim_end_matches(".DLL")
            .to_string();

        let text_size = align_page(n * FUNC_SLOT);
        let edata_rva = TEXT_RVA + text_size;

        let dll_name_str = format!("{}.dll", map_name);
        let strings_len: u32 =
            dll_name_str.len() as u32 + 1 + exports.iter().map(|s| s.len() as u32 + 1).sum::<u32>();
        let edata_raw = 40 + n * 10 + strings_len;
        let edata_size = align_page(edata_raw);

        let size_of_image = PAGE + text_size + edata_size;
        let base = match self.maps.lib64_alloc(size_of_image as u64) {
            Some(b) => b,
            None => {
                log::error!("create_stub_pe64: out of lib64 space for {}", dll_name);
                return 0;
            }
        };

        let pe_map = match self.maps.create_map(
            &format!("{}.pe", map_name),
            base,
            PAGE as u64,
            Permission::READ_WRITE,
        ) {
            Ok(m) => m,
            Err(e) => {
                log::error!("cannot create stub pe64 map: {}", e);
                return 0;
            }
        };

        let mut hdr = vec![0u8; PAGE as usize];
        hdr[0] = 0x4d;
        hdr[1] = 0x5a;
        w32(&mut hdr, 0x3c, 0x80);
        w32(&mut hdr, 0x80, 0x0000_4550);

        w16(&mut hdr, 0x84, 0x8664); // Machine = AMD64
        w16(&mut hdr, 0x86, 2);
        w16(&mut hdr, 0x94, 0xf0); // SizeOfOptionalHeader (PE32+)
        w16(&mut hdr, 0x96, 0x2022); // DLL | EXEC | LARGE_ADDRESS_AWARE

        w16(&mut hdr, 0x98, 0x20b); // Magic = PE32+
        w32(&mut hdr, 0x9c, text_size);
        w32(&mut hdr, 0xac, TEXT_RVA);
        // PE32+ has ImageBase at 0xb0 as u64
        let ib_off = 0xb0;
        hdr[ib_off..ib_off + 8].copy_from_slice(&base.to_le_bytes());
        w32(&mut hdr, 0xb8, PAGE); // SectionAlignment
        w32(&mut hdr, 0xbc, 0x200); // FileAlignment
        w32(&mut hdr, 0xd0, size_of_image); // SizeOfImage
        w32(&mut hdr, 0xd4, 0x200); // SizeOfHeaders
        w16(&mut hdr, 0xdc, 3); // Subsystem = CONSOLE
        w32(&mut hdr, 0x108, 16); // NumberOfRvaAndSizes

        w32(&mut hdr, 0x10c, edata_rva); // DataDirectory[0] Export RVA
        w32(&mut hdr, 0x110, edata_raw); // Export Size

        let s0 = 0x188;
        copy_name(&mut hdr, s0, b".text\0\0\0");
        w32(&mut hdr, s0 + 8, text_size);
        w32(&mut hdr, s0 + 12, TEXT_RVA);
        w32(&mut hdr, s0 + 16, text_size);
        w32(&mut hdr, s0 + 20, TEXT_RVA);
        w32(&mut hdr, s0 + 36, 0x6000_0020);

        let s1 = s0 + 40;
        copy_name(&mut hdr, s1, b".edata\0\0");
        w32(&mut hdr, s1 + 8, edata_size);
        w32(&mut hdr, s1 + 12, edata_rva);
        w32(&mut hdr, s1 + 16, edata_size);
        w32(&mut hdr, s1 + 20, edata_rva);
        w32(&mut hdr, s1 + 36, 0x4000_0040);

        pe_map.memcpy(&hdr, hdr.len());

        let text_map = match self.maps.create_map(
            &format!("{}.text", map_name),
            base + TEXT_RVA as u64,
            text_size as u64,
            Permission::READ_WRITE_EXECUTE,
        ) {
            Ok(m) => m,
            Err(e) => {
                log::error!("cannot create stub text64 map: {}", e);
                return 0;
            }
        };
        for i in 0..n {
            text_map.write_byte(base + TEXT_RVA as u64 + (i * FUNC_SLOT) as u64, 0xc3);
        }

        let edata_base = base + edata_rva as u64;
        let edata_map = match self.maps.create_map(
            &format!("{}.edata", map_name),
            edata_base,
            edata_size as u64,
            Permission::READ_WRITE,
        ) {
            Ok(m) => m,
            Err(e) => {
                log::error!("cannot create stub edata64 map: {}", e);
                return 0;
            }
        };

        let mut ed = vec![0u8; edata_size as usize];

        let func_tbl_rva = edata_rva + 40;
        let name_tbl_rva = func_tbl_rva + n * 4;
        let ord_tbl_rva = name_tbl_rva + n * 4;
        let str_base_rva = ord_tbl_rva + n * 2;

        let mut str_off = (str_base_rva - edata_rva) as usize;
        let dll_name_rva = edata_rva + str_off as u32;
        for b in dll_name_str.bytes() {
            ed[str_off] = b;
            str_off += 1;
        }
        ed[str_off] = 0;
        str_off += 1;

        let mut name_rvas = Vec::with_capacity(exports.len());
        for name in exports {
            name_rvas.push(edata_rva + str_off as u32);
            for b in name.bytes() {
                ed[str_off] = b;
                str_off += 1;
            }
            ed[str_off] = 0;
            str_off += 1;
        }

        w32(&mut ed, 0x0c, dll_name_rva);
        w32(&mut ed, 0x10, 1);
        w32(&mut ed, 0x14, n);
        w32(&mut ed, 0x18, n);
        w32(&mut ed, 0x1c, func_tbl_rva);
        w32(&mut ed, 0x20, name_tbl_rva);
        w32(&mut ed, 0x24, ord_tbl_rva);

        let fo = (func_tbl_rva - edata_rva) as usize;
        for i in 0..n as usize {
            w32(&mut ed, fo + i * 4, TEXT_RVA + (i as u32) * FUNC_SLOT);
        }

        let no = (name_tbl_rva - edata_rva) as usize;
        for (i, &rva) in name_rvas.iter().enumerate().take(n as usize) {
            w32(&mut ed, no + i * 4, rva);
        }

        let oo = (ord_tbl_rva - edata_rva) as usize;
        for i in 0..n as usize {
            w16(&mut ed, oo + i * 2, i as u16);
        }

        edata_map.memcpy(&ed, ed.len());

        let mut ordinal_targets = Vec::with_capacity(exports.len());
        let mut named_exports_vec = Vec::with_capacity(exports.len());
        for (i, name) in exports.iter().enumerate() {
            ordinal_targets.push(Some(ExportTarget::Direct {
                rva: TEXT_RVA + (i as u32) * FUNC_SLOT,
            }));
            named_exports_vec.push(NamedExport {
                name: name.to_string(),
                ordinal_index: i as u32,
            });
        }
        let parsed = ExportIndexData {
            export_base: 1,
            number_of_functions: exports.len() as u32,
            ordinal_targets,
            named_exports: named_exports_vec,
        };
        let index = ModuleExportIndex::from_parsed(
            map_name.clone(),
            export_index::normalize_module_name(&map_name),
            base,
            &parsed,
        );
        self.export_indexes.register(index);

        log::info!(
            "created PE64 stub for {} at 0x{:x} ({} exports)",
            dll_name,
            base,
            exports.len()
        );

        base
    }
}

fn w32(buf: &mut [u8], off: usize, val: u32) {
    buf[off..off + 4].copy_from_slice(&val.to_le_bytes());
}

fn w16(buf: &mut [u8], off: usize, val: u16) {
    buf[off..off + 2].copy_from_slice(&val.to_le_bytes());
}

fn copy_name(buf: &mut [u8], off: usize, name: &[u8; 8]) {
    buf[off..off + 8].copy_from_slice(name);
}
