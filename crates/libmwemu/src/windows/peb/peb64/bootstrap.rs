use crate::emu;
use crate::maps::mem64::Permission;
use crate::windows::peb::peb64::ldr::create_ldr_entry;
use crate::windows::structures::PEB64;
use crate::windows::structures::PebLdrData64;
use crate::windows::structures::RtlUserProcessParameters64;
use crate::windows::structures::TEB64;

/// `PEB+0x90` (`system_dependent_07`): ntdll reads this as a pointer in loader / API-set helpers.
/// Real CSRSS/kernel fills it; under emulation `LdrInitializeThunk` (and related code) often leaves
/// this NULL, which makes loader code fault. If the slot is NULL or points at unmapped memory,
/// install or reuse a dedicated zero-filled page and write its base to `PEB+0x90`.
pub fn ensure_peb_system_dependent_07(emu: &mut emu::Emu) {
    if !emu.cfg.arch.is_64bits() {
        return;
    }
    let peb_base = match emu.maps.get_map_by_name("peb") {
        Some(m) => m.get_base(),
        None => return,
    };
    let cur = emu.maps.read_qword(peb_base + 0x90).unwrap_or(0);
    if cur != 0 && emu.maps.is_mapped(cur) {
        return;
    }

    const SD07_NAME: &str = "peb_system_dependent_07";
    const SD07_SZ: u64 = 0x1000;
    let sd07 = if let Some(m) = emu.maps.get_map_by_name(SD07_NAME) {
        m.get_base()
    } else {
        let base = emu.maps.map(SD07_NAME, SD07_SZ, Permission::READ_WRITE);
        emu.maps.memset(base, 0, SD07_SZ as usize);
        base
    };
    emu.maps.write_qword(peb_base + 0x90, sd07);
}

/// `TEB+0x2C8` points at the thread activation-context stack.
fn ensure_teb_activation_context_stack(emu: &mut emu::Emu) {
    if !emu.cfg.arch.is_64bits() {
        return;
    }
    let teb_base = match emu.maps.get_map_by_name("teb") {
        Some(m) => m.get_base(),
        None => return,
    };
    let cur = emu.maps.read_qword(teb_base + 0x2c8).unwrap_or(0);
    if cur != 0 && emu.maps.is_mapped(cur) {
        return;
    }

    const ACTCTX_NAME: &str = "teb_activation_context_stack";
    const ACTCTX_SZ: u64 = 0x1000;
    let actctx = if let Some(m) = emu.maps.get_map_by_name(ACTCTX_NAME) {
        m.get_base()
    } else {
        let base = emu.maps.map(ACTCTX_NAME, ACTCTX_SZ, Permission::READ_WRITE);
        emu.maps.memset(base, 0, ACTCTX_SZ as usize);
        emu.maps.write_qword(base + 0x08, base + 0x08);
        emu.maps.write_qword(base + 0x10, base + 0x08);
        base
    };
    emu.maps.write_qword(teb_base + 0x2c8, actctx);
}

/// Compressed Win11 (24H2) `API_SET_NAMESPACE` blob, sourced from sogen's
/// `default_apiset.hpp`. Decompresses to ~128 KB containing the full
/// virtual-DLL → host-DLL redirect schema for hundreds of `api-ms-*` and
/// `ext-ms-*` names. Embedding this avoids shipping the ~115 individual
/// stub DLLs that would otherwise be needed to satisfy ntdll's loader.
const APISET_W11_ZLIB: &[u8] = include_bytes!("../../../../data/apiset_w11.zlib"); // TODO: rebuild

/// `PEB+0x68` = `ApiSetMap` on x64 Windows 10+ ntdll. The loader reads it
/// during `LdrpInitializeProcess` to redirect virtual API-set names
/// (`api-ms-*`, `ext-ms-*`) to real DLLs. Without a real schema every
/// dependency lookup fails: KnownDll has no entry, disk lookup fails, and
/// the loader terminates with `STATUS_DLL_NOT_FOUND`.
///
/// Decompress the embedded Win11 schema and map it at a stable virtual
/// address; the schema's `EntryOffset`, `HashOffset`, and per-entry name
/// offsets are all *relative* to the namespace base, so we can map the raw
/// blob unchanged and ntdll's resolver walks it correctly.
fn ensure_peb_system_dependent_06(emu: &mut emu::Emu) {
    if !emu.cfg.arch.is_64bits() {
        return;
    }
    let peb_base = match emu.maps.get_map_by_name("peb") {
        Some(m) => m.get_base(),
        None => return,
    };
    let cur = emu.maps.read_qword(peb_base + 0x68).unwrap_or(0);
    if cur != 0 && emu.maps.is_mapped(cur) {
        return;
    }

    const SD06_NAME: &str = "peb_apiset_map";
    let sd06 = if let Some(m) = emu.maps.get_map_by_name(SD06_NAME) {
        m.get_base()
    } else {
        // Decompress the zlib-compressed schema from the embedded blob.
        use flate2::read::ZlibDecoder;
        use std::io::Read;
        let mut decoder = ZlibDecoder::new(APISET_W11_ZLIB);
        let mut decompressed = Vec::with_capacity(0x20000);
        if decoder.read_to_end(&mut decompressed).is_err() {
            log::warn!(
                "ensure_peb_system_dependent_06: failed to decompress embedded apiset, falling back to empty schema"
            );
            return;
        }

        let sz = ((decompressed.len() + 0xFFF) & !0xFFF) as u64;
        let base = emu.maps.map(SD06_NAME, sz, Permission::READ_WRITE);
        emu.maps.memset(base, 0, sz as usize);
        if let Some(mem) = emu.maps.get_mem_by_addr_mut(base) {
            mem.memcpy(&decompressed, decompressed.len());
            mem.set_permission(Permission::READ);
        }
        log::trace!(
            "ensure_peb_system_dependent_06: ApiSetMap (Win11 schema, {} bytes) mapped at 0x{:x}",
            decompressed.len(),
            base
        );
        base
    };
    emu.maps.write_qword(peb_base + 0x68, sd06);
}

/// Map the three NLS code-page sections that `RtlInitNlsTables` reads via
/// `PEB.AnsiCodePageData` / `OemCodePageData` / `UnicodeCaseTableData`. In
/// real Windows the kernel hands ntdll already-populated section views from
/// `C:\Windows\System32\C_1252.NLS`, `C_437.NLS` and `locale.nls`; we mirror
/// that by reading those files from `cfg.maps_folder`. Without real NLS
/// content every byte→wide conversion in the loader produces zeros and DLL
/// dependency lookups fail with `STATUS_DLL_NOT_FOUND`.
pub fn ensure_peb_nls_tables(emu: &mut emu::Emu) {
    if !emu.cfg.arch.is_64bits() {
        return;
    }
    let peb_base = match emu.maps.get_map_by_name("peb") {
        Some(m) => m.get_base(),
        None => return,
    };

    // Fallback size if the file is missing — ntdll then sees a zero buffer
    // (same as before this fix), but at least it's mapped so reads don't crash.
    const FALLBACK_SZ: u64 = 0x1000;
    // (PEB slot, map name, NLS filename)
    let slots: &[(&str, u64, &str)] = &[
        ("peb_nls_ansi", peb_base + 0xA0, "C_1252.NLS"),
        ("peb_nls_oem", peb_base + 0xA8, "C_437.NLS"),
        ("peb_nls_unicode", peb_base + 0xB0, "locale.nls"),
    ];
    for (name, slot_addr, nls_filename) in slots {
        let cur = emu.maps.read_qword(*slot_addr).unwrap_or(0);
        let page = cur & !0xFFF;
        if cur != 0 && page != 0 && emu.maps.is_mapped(page) {
            continue;
        }
        let base = if let Some(m) = emu.maps.get_map_by_name(name) {
            m.get_base()
        } else {
            let path = emu.cfg.get_maps_folder(nls_filename);
            match std::fs::read(&path) {
                Ok(bytes) => {
                    let sz = ((bytes.len() + 0xFFF) & !0xFFF) as u64;
                    let b = emu.maps.map(name, sz, Permission::READ_WRITE);
                    emu.maps.memset(b, 0, sz as usize);
                    if let Some(mem) = emu.maps.get_mem_by_addr_mut(b) {
                        mem.memcpy(&bytes, bytes.len());
                    }
                    log::trace!(
                        "ensure_peb_nls_tables: loaded {} ({} bytes) at 0x{:x}",
                        nls_filename,
                        bytes.len(),
                        b
                    );
                    b
                }
                Err(_) => {
                    log::warn!(
                        "ensure_peb_nls_tables: missing {} — leaving {} zeroed",
                        path,
                        name
                    );
                    let b = emu.maps.map(name, FALLBACK_SZ, Permission::READ_WRITE);
                    emu.maps.memset(b, 0, FALLBACK_SZ as usize);
                    b
                }
            }
        };
        emu.maps.write_qword(*slot_addr, base);
    }
}

pub fn init_ldr(emu: &mut emu::Emu) -> u64 {
    let ldr_sz = PebLdrData64::size() + 100;
    let ldr_addr = emu
        .maps
        .lib64_alloc(ldr_sz as u64)
        .expect("cannot alloc the LDR");
    emu.maps
        .create_map("ldr", ldr_addr, ldr_sz as u64, Permission::READ_WRITE)
        .expect("cannot create ldr map");
    let exe_name = emu.cfg.exe_name.clone();
    let module_entry = create_ldr_entry(emu, 0, 0, &exe_name, 0, 0);
    let mut ldr = PebLdrData64::new();
    ldr.initializated = 1;
    ldr.in_load_order_module_list.flink = module_entry;
    ldr.in_load_order_module_list.blink = module_entry;
    ldr.in_memory_order_module_list.flink = module_entry + 0x10;
    ldr.in_memory_order_module_list.blink = module_entry + 0x10;
    ldr.in_initialization_order_module_list.flink = module_entry + 0x20;
    ldr.in_initialization_order_module_list.blink = module_entry + 0x20;
    ldr.entry_in_progress.flink = module_entry;
    ldr.entry_in_progress.blink = module_entry;
    ldr.save(ldr_addr, &mut emu.maps);

    ldr_addr
}

/// Peek the PE Characteristics field of `filename` to tell whether it is a DLL
/// or an EXE without doing the full PE load. Returns `Some(true)` for a DLL
/// (`IMAGE_FILE_DLL = 0x2000`), `Some(false)` for an EXE, or `None` if the
/// file cannot be parsed.
pub fn pe_is_dll(filename: &str) -> Option<bool> {
    use std::io::Read;
    if filename.is_empty() {
        return None;
    }
    let mut f = std::fs::File::open(filename).ok()?;
    let mut hdr = [0u8; 0x140];
    let _ = f.read(&mut hdr).ok()?;
    if hdr[0] != b'M' || hdr[1] != b'Z' {
        return None;
    }
    let e_lfanew = u32::from_le_bytes([hdr[0x3c], hdr[0x3d], hdr[0x3e], hdr[0x3f]]) as usize;
    // COFF FileHeader.Characteristics is at e_lfanew+4 (Signature) +18.
    let char_off = e_lfanew + 4 + 18;
    if char_off + 2 > hdr.len() {
        // Header straddles past the slurped chunk — fetch the bytes directly.
        use std::io::Seek;
        let mut f = std::fs::File::open(filename).ok()?;
        f.seek(std::io::SeekFrom::Start(char_off as u64)).ok()?;
        let mut buf = [0u8; 2];
        f.read_exact(&mut buf).ok()?;
        let ch = u16::from_le_bytes(buf);
        return Some((ch & 0x2000) != 0);
    }
    let ch = u16::from_le_bytes([hdr[char_off], hdr[char_off + 1]]);
    Some((ch & 0x2000) != 0)
}

/// If `cfg.filename` is an EXE (not a DLL), set `cfg.exe_name` to its
/// basename so PEB/LDR setup uses the real image name instead of the
/// generic "loader.exe" placeholder. Idempotent; safe to call before
/// PEB init. Leaves the default in place for DLLs and shellcode.
pub fn refresh_exe_name_from_filename(emu: &mut emu::Emu) {
    let basename: String = std::path::Path::new(&emu.cfg.filename)
        .file_name()
        .and_then(|s| s.to_str())
        .map(|s| s.to_string())
        .unwrap_or_default();
    if basename.is_empty() {
        return;
    }
    if matches!(pe_is_dll(&emu.cfg.filename), Some(false)) {
        emu.cfg.exe_name = basename;
    }
}

pub fn init_arguments(emu: &mut emu::Emu) -> u64 {
    let addr = emu.maps.map(
        "RtlUserProcessParameters64",
        RtlUserProcessParameters64::size() as u64,
        Permission::READ_WRITE_EXECUTE,
    );
    let mut params_struct = RtlUserProcessParameters64::new();

    // `cfg.exe_name` is set to the real EXE basename by
    // `refresh_exe_name_from_filename` BEFORE `init_ldr` runs, so the LDR
    // entry created early shares the name `init_arguments` writes into
    // ImagePathName. Present the path Windows-canonical (`C:\<name>`) so
    // newer ntdll (`LdrpInitializeProcess` → `RtlGetFullPathName_UEx`)
    // can derive FullDllName / BaseDllName for the EXE's
    // LDR_DATA_TABLE_ENTRY — unix-style paths like `test/foo.bin` make
    // the parser leave the freshly-allocated BaseDllName.Buffer untouched
    // and the next `RtlHashUnicodeString` faults inside ntdll.
    let image_path = format!("C:\\{}", emu.cfg.exe_name);
    let mut cmdline_str = image_path.clone();
    cmdline_str.push_str(&emu.cfg.arguments);

    // UNICODE_STRING.Length is bytes WITHOUT trailing NUL; MaximumLength
    // is the buffer capacity INCLUDING space for the NUL terminator.
    let image_bytes = image_path.len() as u64 * 2;
    let image_maxlen = image_bytes + 2;
    let cmd_bytes = cmdline_str.len() as u64 * 2;
    let cmd_maxlen = cmd_bytes + 2;

    let filename = emu
        .maps
        .map("file_name", image_maxlen, Permission::READ_WRITE);
    let cmdline = emu
        .maps
        .map("command_line", cmd_maxlen, Permission::READ_WRITE);

    let dll_path_buf = emu.maps.map("dll_path", 4, Permission::READ_WRITE);
    emu.maps.write_wide_string(dll_path_buf, "");

    params_struct.image_path_name.length = image_bytes as u16;
    params_struct.image_path_name.maximum_length = image_maxlen as u16;
    params_struct.image_path_name.buffer = filename;

    params_struct.command_line.length = cmd_bytes as u16;
    params_struct.command_line.maximum_length = cmd_maxlen as u16;
    params_struct.command_line.buffer = cmdline;

    params_struct.dll_path.length = 0;
    params_struct.dll_path.maximum_length = 4;
    params_struct.dll_path.buffer = dll_path_buf;

    emu.maps.write_wide_string(filename, &image_path);
    emu.maps.write_wide_string(cmdline, &cmdline_str);

    params_struct.save(addr, &mut emu.maps);

    addr
}

pub fn init_peb(emu: &mut emu::Emu) {
    let ldr = init_ldr(emu);
    let params_addr = init_arguments(emu);

    let peb_addr = emu
        .maps
        .lib64_alloc(PEB64::size() as u64)
        .expect("cannot alloc the PEB64");
    let peb_map = emu
        .maps
        .create_map(
            "peb",
            peb_addr,
            PEB64::size() as u64,
            Permission::READ_WRITE,
        )
        .expect("cannot create peb map");
    let peb = PEB64::new(0, ldr, params_addr);
    peb.save(peb_map);
    emu.maps.write_byte(peb_addr + 2, 0);

    let teb_addr = emu
        .maps
        .lib64_alloc(TEB64::size() as u64)
        .expect("cannot alloc the TEB64");
    let teb_map = emu
        .maps
        .create_map(
            "teb",
            teb_addr,
            TEB64::size() as u64,
            Permission::READ_WRITE,
        )
        .expect("cannot create teb map");
    let teb = TEB64::new(peb_addr);
    teb.save(teb_map);

    ensure_teb_activation_context_stack(emu);
    ensure_peb_system_dependent_07(emu);
}

/// Allocate and initialize a minimal Windows `_HEAP` (x64) structure.
fn init_process_heap(emu: &mut emu::Emu) -> u64 {
    let heap_sz: u64 = 0x1000;
    let h = emu
        .maps
        .lib64_alloc(heap_sz)
        .expect("cannot alloc fake ProcessHeap");
    let _heap_map = emu
        .maps
        .create_map("process_heap", h, heap_sz, Permission::READ_WRITE)
        .expect("cannot create process_heap map");
    emu.maps.memset(h, 0, heap_sz as usize);

    let self_list = |maps: &mut crate::maps::Maps, addr: u64| {
        maps.write_qword(addr, addr);
        maps.write_qword(addr + 8, addr);
    };

    emu.maps.write_dword(h + 0x010, 0xFEED_FEED);
    self_list(&mut emu.maps, h + 0x018);
    emu.maps.write_qword(h + 0x028, h);
    emu.maps.write_qword(h + 0x030, h);
    emu.maps.write_dword(h + 0x038, (heap_sz >> 12) as u32);
    emu.maps.write_qword(h + 0x040, h + 0x200);
    emu.maps.write_qword(h + 0x048, h + heap_sz);
    self_list(&mut emu.maps, h + 0x060);

    emu.maps.write_dword(h + 0x070, 0x0000_0002);
    emu.maps.write_dword(h + 0x098, 0xEEFF_EEFF);
    emu.maps.write_qword(h + 0x0C8, 0x7FFF_EFFF_FFFF);

    self_list(&mut emu.maps, h + 0x0E8);
    self_list(&mut emu.maps, h + 0x108);
    self_list(&mut emu.maps, h + 0x118);
    self_list(&mut emu.maps, h + 0x148);

    let ucr_stub = h + 0x300;
    self_list(&mut emu.maps, ucr_stub);
    emu.maps.write_qword(h + 0x138, ucr_stub);

    emu.maps.write_word(h + 0x1B0, 0);

    let lock_addr = h + 0x400;
    emu.maps.write_dword(lock_addr + 0x08, 0xFFFF_FFFF);
    emu.maps.write_qword(h + 0x158, lock_addr);
    emu.maps.write_qword(h + 0x160, lock_addr);

    h
}

/// Minimal PEB/TEB for `--init` / `ssdt_use_ldr_initialize_thunk`.
pub fn init_peb_teb_empty(emu: &mut emu::Emu) {
    let ldr_addr = init_ldr(emu);
    let params_addr = init_arguments(emu);

    let peb_addr = emu
        .maps
        .lib64_alloc(PEB64::size() as u64)
        .expect("cannot alloc the PEB64");
    let _peb_map = emu
        .maps
        .create_map(
            "peb",
            peb_addr,
            PEB64::size() as u64,
            Permission::READ_WRITE,
        )
        .expect("cannot create peb map");
    let heap_addr = init_process_heap(emu);
    let peb = PEB64::new(0, ldr_addr, params_addr);
    peb.save(emu.maps.get_mem_mut("peb"));
    emu.maps.write_byte(peb_addr + 2, 0);
    emu.maps.write_qword(peb_addr + 0x30, heap_addr);

    let teb_addr = emu
        .maps
        .lib64_alloc(TEB64::size() as u64)
        .expect("cannot alloc the TEB64");
    let _teb_map = emu
        .maps
        .create_map(
            "teb",
            teb_addr,
            TEB64::size() as u64,
            Permission::READ_WRITE,
        )
        .expect("cannot create teb map");
    let teb = TEB64::new(peb_addr);
    teb.save(emu.maps.get_mem_mut("teb"));

    ensure_teb_activation_context_stack(emu);
    ensure_peb_system_dependent_06(emu);
    ensure_peb_system_dependent_07(emu);
    ensure_peb_nls_tables(emu);
}

pub fn update_peb_image_base(emu: &mut emu::Emu, base: u64) {
    let peb = emu.maps.get_mem("peb");
    let peb_base = peb.get_base();
    emu.maps.write_qword(peb_base + 0x10, base);
}
