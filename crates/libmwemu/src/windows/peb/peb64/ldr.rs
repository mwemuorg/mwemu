use crate::emu;
use crate::maps::mem64::Permission;
use crate::winapi::common::modules::main_module_path;
use crate::windows::peb::peb64::bootstrap::ensure_peb_system_dependent_07;
use crate::windows::peb::{
    ENTRY_POINT_RVA_OFFSET, EXPORT_DIRECTORY_OFFSET64, IMAGE_SIZE_OFFSET64, LDR_NAME_RESERVE,
    LdrNames, unlink_dynamic_entry,
};
use crate::windows::structures::{
    ImageExportDirectory, LdrDataTableEntry64, ListEntry64, OrdinalTable, Peb64, PebLdrData64,
    UnicodeString64,
};
use goblin::pe::header::PE_POINTER_OFFSET;
use std::sync::atomic::{AtomicU32, Ordering};

const LDR_HASH_BUCKET_COUNT: u64 = 32;
const LDR_HASH_MULTIPLIER: u32 = 65_599;
const LDR_SYSTEM_MODULE_FLAGS: u32 = 0x000c_4004;

const DDAG_NODE_SIZE: u64 = 0x60;
const DDAG_LOAD_COUNT_OFFSET: u64 = 0x18;
const DDAG_STATE_OFFSET: u64 = 0x38;
const LDR_MODULES_READY_TO_RUN: u32 = 9;

const RB_TREE_MIN_OFFSET: u64 = 0x08;
const BALANCED_NODE_RIGHT_OFFSET: u64 = 0x08;
const BALANCED_NODE_PARENT_OFFSET: u64 = 0x10;
const CRITICAL_SECTION_LOCK_COUNT_OFFSET: u64 = 0x08;

// Private RVAs in the ntdll layout used by this loader bootstrap.
const NTDLL_LDRP_HASH_TABLE_RVA64: u64 = 0x1d3120;
const NTDLL_LDRP_GLOBAL_2680_RVA64: u64 = 0x1d2680;
const NTDLL_LDRP_GLOBAL_26C0_RVA64: u64 = 0x1d26c0;
const NTDLL_LDRP_GLOBAL_26F0_RVA64: u64 = 0x1d26f0;

// RTL_RB_TREE `LdrpModuleBaseAddressIndex` lives at ntdll + this RVA.
const NTDLL_LDRP_MODULE_BASE_ADDRESS_INDEX_RVA: u64 = 0x16b458;

// RVA of the loader lock RTL_CRITICAL_SECTION inside ntdll .data.
const NTDLL_LOADER_LOCK_CS_RVA: u64 = 0x16abb8;

// ntdll global pointing to the 256-entry u16 ANSI→Unicode upcase translation
// table used by LdrpUnicodeStringToHash / RtlpUpcaseUnicodeStringTo... (case-
// insensitive name hashing during GetProcAddress). Real ntdll initialises this
// during NLS table mapping; under --ssdt the memory is allocated but stays
// zeroed, so every input byte upcases to U+0000 and LdrGetProcedureAddress
// returns STATUS_ENTRYPOINT_NOT_FOUND for every export lookup.
const NTDLL_NLS_UPCASE_TABLE_PTR_RVA: u64 = 0x16b6f0;

#[derive(Debug)]
pub struct Flink {
    flink_addr: u64,
    pub mod_base: u64,
    pub mod_name: String,
    pub pe_hdr: u64,
    pub export_table_rva: u64,
    pub export_dir_size: u64,
    pub export_table: u64,
    pub num_of_funcs: u64,
    pub func_name_tbl_rva: u64,
    pub func_name_tbl: u64,
}

impl Flink {
    pub fn new(emu: &mut emu::Emu) -> Flink {
        let peb = emu.maps.get_mem("peb");
        let peb_base = peb.get_base();
        let ldr = peb.read_qword(peb_base + Peb64::LDR_OFFSET);
        let flink = emu
            .maps
            .read_qword(ldr + PebLdrData64::IN_LOAD_ORDER_MODULE_LIST_OFFSET)
            .unwrap_or(0);

        Flink {
            flink_addr: flink,
            mod_base: 0,
            mod_name: String::new(),
            pe_hdr: 0,
            export_table_rva: 0,
            export_dir_size: 0,
            export_table: 0,
            num_of_funcs: 0,
            func_name_tbl_rva: 0,
            func_name_tbl: 0,
        }
    }

    pub fn print(&self) {
        log::trace!("{:#x?}", self);
    }

    pub fn get_ptr(&self) -> u64 {
        self.flink_addr
    }

    pub fn set_ptr(&mut self, addr: u64) {
        self.flink_addr = addr;
    }

    pub fn load(&mut self, emu: &mut emu::Emu) {
        self.get_mod_base(emu);
        self.get_mod_name(emu);
        self.get_pe_hdr(emu);
        self.get_export_table(emu);
    }

    pub fn get_mod_base(&mut self, emu: &mut emu::Emu) {
        self.mod_base = emu
            .maps
            .read_qword(self.flink_addr + LdrDataTableEntry64::DLL_BASE_OFFSET)
            .unwrap_or(0);
    }

    pub fn set_mod_base(&mut self, base: u64, emu: &mut emu::Emu) {
        self.mod_base = base;
        emu.maps
            .write_qword(self.flink_addr + LdrDataTableEntry64::DLL_BASE_OFFSET, base);
    }

    pub fn get_mod_name(&mut self, emu: &mut emu::Emu) {
        let mod_name_ptr = emu
            .maps
            .read_qword(
                self.flink_addr
                    + LdrDataTableEntry64::BASE_DLL_NAME_OFFSET
                    + UnicodeString64::BUFFER_OFFSET,
            )
            .unwrap_or(0);
        self.mod_name = emu.maps.read_wide_string(mod_name_ptr);
    }

    pub fn has_module(&self) -> bool {
        self.mod_base != 0 && self.flink_addr != 0
    }

    pub fn get_pe_hdr(&mut self, emu: &mut emu::Emu) {
        self.pe_hdr = match emu
            .maps
            .read_dword(self.mod_base + PE_POINTER_OFFSET as u64)
        {
            Some(hdr) => hdr as u64,
            None => 0,
        };
    }

    pub fn get_export_table(&mut self, emu: &mut emu::Emu) {
        if self.pe_hdr == 0 {
            return;
        }

        self.export_table_rva = match emu
            .maps
            .read_dword(self.mod_base + self.pe_hdr + EXPORT_DIRECTORY_OFFSET64)
        {
            Some(rva) => rva as u64,
            None => 0,
        };

        self.export_dir_size = emu
            .maps
            .read_dword(
                self.mod_base + self.pe_hdr + EXPORT_DIRECTORY_OFFSET64 + size_of::<u32>() as u64,
            )
            .unwrap_or(0) as u64;

        if self.export_table_rva == 0 {
            return;
        }

        self.export_table = self.export_table_rva + self.mod_base;
        self.num_of_funcs = emu
            .maps
            .read_dword(self.export_table + ImageExportDirectory::NUMBER_OF_NAMES_OFFSET)
            .unwrap_or(0) as u64;
        self.func_name_tbl_rva = emu
            .maps
            .read_dword(self.export_table + ImageExportDirectory::ADDRESS_OF_NAMES_OFFSET)
            .unwrap_or(0) as u64;
        self.func_name_tbl = self.func_name_tbl_rva + self.mod_base;
    }

    pub fn get_function_ordinal(&self, emu: &mut emu::Emu, function_id: u64) -> OrdinalTable {
        self.get_function_ordinal_depth(emu, function_id, 0)
    }

    pub fn get_function_ordinal_depth(
        &self,
        emu: &mut emu::Emu,
        function_id: u64,
        forward_depth: u32,
    ) -> OrdinalTable {
        let mut ordinal = OrdinalTable::new();
        let func_name_rva = emu
            .maps
            .read_dword(self.func_name_tbl + function_id * 4)
            .unwrap_or(0) as u64;
        ordinal.func_name = emu.maps.read_string(func_name_rva + self.mod_base);
        ordinal.ordinal_tbl_rva = emu
            .maps
            .read_dword(self.export_table + ImageExportDirectory::ADDRESS_OF_ORDINALS_OFFSET)
            .unwrap_or(0) as u64;
        ordinal.ordinal_tbl = ordinal.ordinal_tbl_rva + self.mod_base;
        ordinal.ordinal = emu
            .maps
            .read_word(ordinal.ordinal_tbl + 2 * function_id)
            .unwrap_or(0) as u64;
        ordinal.func_addr_tbl_rva = emu
            .maps
            .read_dword(self.export_table + ImageExportDirectory::ADDRESS_OF_FUNCTIONS_OFFSET)
            .unwrap_or(0) as u64;
        ordinal.func_addr_tbl = ordinal.func_addr_tbl_rva + self.mod_base;
        ordinal.func_rva = emu
            .maps
            .read_dword(ordinal.func_addr_tbl + 4 * ordinal.ordinal)
            .unwrap_or(0) as u64;

        if self.export_dir_size > 0
            && ordinal.func_rva >= self.export_table_rva
            && ordinal.func_rva < self.export_table_rva.saturating_add(self.export_dir_size)
        {
            let forwarder = emu.maps.read_string(self.mod_base + ordinal.func_rva);
            let resolved = crate::winapi::winapi64::kernel32::resolve_forwarded_export_string_depth(
                emu,
                &forwarder,
                forward_depth.saturating_add(1),
            );
            if resolved != 0 {
                ordinal.func_va = resolved;
            } else {
                ordinal.func_va = ordinal.func_rva + self.mod_base;
            }
        } else {
            ordinal.func_va = ordinal.func_rva + self.mod_base;
        }

        ordinal
    }

    pub fn get_next_flink(&self, emu: &mut emu::Emu) -> u64 {
        emu.maps.read_qword(self.flink_addr).unwrap_or(0)
    }

    pub fn get_prev_flink(&self, emu: &mut emu::Emu) -> u64 {
        emu.maps
            .read_qword(self.flink_addr + ListEntry64::BLINK_OFFSET)
            .unwrap_or(0)
    }

    pub fn next(&mut self, emu: &mut emu::Emu) -> bool {
        let next = self.get_next_flink(emu);
        if next == 0 || next == self.flink_addr {
            return false;
        }
        self.flink_addr = next;
        self.load(emu);
        true
    }
}

pub fn get_module_base(libname: &str, emu: &mut emu::Emu) -> Option<u64> {
    let mut libname2 = libname.to_string().to_lowercase();
    if !libname2.ends_with(".dll") {
        libname2.push_str(".dll");
    }

    let mut flink = Flink::new(emu);
    flink.load(emu);
    let first_flink = flink.get_ptr();
    let mut iters = 0usize;
    loop {
        if libname.to_string().to_lowercase() == flink.mod_name.to_string().to_lowercase()
            || libname2 == flink.mod_name.to_string().to_lowercase()
        {
            return Some(flink.mod_base);
        }
        if !flink.next(emu) {
            break;
        }
        iters += 1;

        if flink.get_ptr() == first_flink || iters > 4096 {
            break;
        }
    }
    None
}

pub fn show_linked_modules(emu: &mut emu::Emu) {
    let mut flink = Flink::new(emu);
    flink.load(emu);
    let first_flink = flink.get_ptr();

    loop {
        let pe1 = emu
            .maps
            .read_byte(flink.mod_base + flink.pe_hdr)
            .unwrap_or_default();
        let pe2 = emu
            .maps
            .read_byte(flink.mod_base + flink.pe_hdr + 1)
            .unwrap_or_default();
        log::trace!(
            "0x{:x} {} flink:{:x} blink:{:x} base:{:x} pe_hdr:{:x} {:x}{:x}",
            flink.get_ptr(),
            flink.mod_name,
            flink.get_next_flink(emu),
            flink.get_prev_flink(emu),
            flink.mod_base,
            flink.pe_hdr,
            pe1,
            pe2
        );
        if !flink.next(emu) || flink.get_ptr() == first_flink {
            return;
        }
    }
}

pub fn update_ldr_entry_base(libname: &str, base: u64, emu: &mut emu::Emu) {
    let mut flink = Flink::new(emu);
    flink.load(emu);
    let first = flink.get_ptr();
    loop {
        if flink.mod_name.to_lowercase() == libname.to_lowercase() {
            flink.set_mod_base(base, emu);
            return;
        }
        if !flink.next(emu) || flink.get_ptr() == first {
            break;
        }
    }
}

pub fn dynamic_unlink_module(libname: &str, emu: &mut emu::Emu) {
    let mut flink = Flink::new(emu);
    flink.load(emu);
    let first = flink.get_ptr();
    for _ in 0..4096 {
        if flink.mod_name.eq_ignore_ascii_case(libname) {
            unlink_dynamic_entry(emu, flink.get_ptr());
            break;
        }
        if !flink.next(emu) {
            break;
        }
        if flink.get_ptr() == first || flink.get_ptr() == 0 {
            break;
        }
    }
    emu.export_indexes.remove(libname);
}

pub fn dynamic_link_module(base: u64, pe_off: u32, libname: &str, emu: &mut emu::Emu) {
    link_dynamic_entry(base, pe_off, libname, emu);
}

pub(crate) fn link_dynamic_entry(base: u64, pe_off: u32, libname: &str, emu: &mut emu::Emu) -> u64 {
    let peb_base = emu.maps.get_mem("peb").get_base();
    let ldr_addr = emu
        .maps
        .read_qword(peb_base + Peb64::LDR_OFFSET)
        .unwrap_or(0);
    if ldr_addr == 0 {
        return 0;
    }

    // Sentinel LIST_ENTRY nodes inside PEB_LDR_DATA (address, not value).
    let sentinel_load = ldr_addr + PebLdrData64::IN_LOAD_ORDER_MODULE_LIST_OFFSET;
    let sentinel_mem = ldr_addr + PebLdrData64::IN_MEMORY_ORDER_MODULE_LIST_OFFSET;
    let sentinel_init = ldr_addr + PebLdrData64::IN_INITIALIZATION_ORDER_MODULE_LIST_OFFSET;

    // Tail = sentinel.Blink (the last real entry in each ordered list).
    let last_load = emu
        .maps
        .read_qword(sentinel_load + ListEntry64::BLINK_OFFSET)
        .unwrap_or(sentinel_load);
    let last_mem = emu
        .maps
        .read_qword(sentinel_mem + ListEntry64::BLINK_OFFSET)
        .unwrap_or(sentinel_mem);
    let last_init = emu
        .maps
        .read_qword(sentinel_init + ListEntry64::BLINK_OFFSET)
        .unwrap_or(sentinel_init);

    // Derive the real DLL entry point from the PE optional header.
    let entry_point = if base > 0 {
        let pe_hdr = emu
            .maps
            .read_dword(base + PE_POINTER_OFFSET as u64)
            .unwrap_or(0) as u64;
        if pe_hdr > 0 {
            let ep_rva = emu
                .maps
                .read_dword(base + pe_hdr + ENTRY_POINT_RVA_OFFSET)
                .unwrap_or(0) as u64;
            base + ep_rva
        } else {
            pe_off as u64
        }
    } else {
        pe_off as u64
    };

    // New entry: Flink = sentinel (end of list), Blink = current tail.
    let space_addr = create_ldr_entry(emu, base, entry_point, libname, sentinel_load, last_load);

    if space_addr == 0 {
        return 0;
    }

    // Patch the old tail to point forward to the new entry.
    emu.maps.write_qword(last_load, space_addr);
    emu.maps.write_qword(
        last_mem,
        space_addr + LdrDataTableEntry64::IN_MEMORY_ORDER_LINKS_OFFSET,
    );
    emu.maps.write_qword(
        last_init,
        space_addr + LdrDataTableEntry64::IN_INITIALIZATION_ORDER_LINKS_OFFSET,
    );

    // Patch sentinel.Blink to the new entry (it is now the new tail).
    emu.maps
        .write_qword(sentinel_load + ListEntry64::BLINK_OFFSET, space_addr);
    emu.maps.write_qword(
        sentinel_mem + ListEntry64::BLINK_OFFSET,
        space_addr + LdrDataTableEntry64::IN_MEMORY_ORDER_LINKS_OFFSET,
    );
    emu.maps.write_qword(
        sentinel_init + ListEntry64::BLINK_OFFSET,
        space_addr + LdrDataTableEntry64::IN_INITIALIZATION_ORDER_LINKS_OFFSET,
    );

    insert_ldr_hash_entry(
        emu,
        space_addr,
        libname.rsplit(['\\', '/']).next().unwrap_or(libname),
    );
    space_addr
}

fn insert_ldr_hash_entry(emu: &mut emu::Emu, entry: u64, libname: &str) {
    let Some(ntdll_map) = emu.maps.get_map_by_name("ntdll.pe") else {
        return;
    };
    let table = ntdll_map.get_base() + NTDLL_LDRP_HASH_TABLE_RVA64;
    let bucket = ldr_hash_bucket_index(libname);
    let head = table + bucket * ListEntry64::SIZE;

    if !emu.maps.is_mapped(head) {
        return;
    }

    let hash_links = entry + LdrDataTableEntry64::HASH_LINKS_OFFSET;
    let old_flink = emu.maps.read_qword(head).unwrap_or(head);
    let tail = if old_flink == 0 || old_flink == head {
        head
    } else {
        emu.maps
            .read_qword(head + ListEntry64::BLINK_OFFSET)
            .unwrap_or(head)
    };

    emu.maps.write_qword(hash_links, head);
    emu.maps
        .write_qword(hash_links + ListEntry64::BLINK_OFFSET, tail);
    if tail != head {
        emu.maps.write_qword(tail, hash_links);
    }
    emu.maps
        .write_qword(head + ListEntry64::BLINK_OFFSET, hash_links);
    if old_flink == 0 || old_flink == head {
        emu.maps.write_qword(head, hash_links);
    }
}

pub fn create_ldr_entry(
    emu: &mut emu::Emu,
    base: u64,
    entry_point: u64,
    libname: &str,
    next_flink: u64,
    prev_flink: u64,
) -> u64 {
    let Some(names) = LdrNames::new(libname) else {
        log::error!("module path exceeds the UNICODE_STRING length limit");
        return 0;
    };
    let libname = names.base;
    let base_addr;
    let sz = LdrDataTableEntry64::size() + names.storage_size().max(LDR_NAME_RESERVE);
    let space_addr = match emu.maps.alloc(sz) {
        Some(a) => a,
        None => {
            log::error!("cannot alloc LDR entry for {}", libname);
            return 0;
        }
    };
    let mut lib = format!("{}.ldr", libname);
    if emu.maps.exists_mapname(&lib) {
        use std::sync::atomic::{AtomicU32, Ordering};
        static LDR_SEQ: AtomicU32 = AtomicU32::new(0);
        lib = format!(
            "{}.ldr.{}",
            libname,
            LDR_SEQ.fetch_add(1, Ordering::Relaxed)
        );
    }
    let mut image_sz = 0;
    if base > 0 {
        let pe_hdr = emu
            .maps
            .read_dword(base + PE_POINTER_OFFSET as u64)
            .unwrap_or(0) as u64;
        image_sz = emu
            .maps
            .read_dword(base + pe_hdr + IMAGE_SIZE_OFFSET64)
            .unwrap_or(0);
        base_addr = base;
    } else {
        base_addr = space_addr
    }
    let mem = match emu
        .maps
        .create_map(lib.as_str(), space_addr, sz, Permission::READ_WRITE)
    {
        Ok(m) => m,
        Err(e) => {
            log::error!("cannot create ldr entry map for {}: {}", libname, e);
            return 0;
        }
    };
    mem.write_byte(space_addr + sz - 1, 0x61);

    let mut ldr = LdrDataTableEntry64::new();
    // hash_links default to self-referencing; insert_ldr_hash_entry (called by
    // dynamic_link_module) or rebuild_ldr_hash_table patches them into the
    // correct LdrpHashTable bucket.
    let hash_links_addr = space_addr + LdrDataTableEntry64::HASH_LINKS_OFFSET;
    if next_flink != 0 {
        ldr.in_load_order_links.flink = next_flink;
        ldr.in_load_order_links.blink = prev_flink;
        ldr.in_memory_order_links.flink =
            next_flink + LdrDataTableEntry64::IN_MEMORY_ORDER_LINKS_OFFSET;
        ldr.in_memory_order_links.blink =
            prev_flink + LdrDataTableEntry64::IN_MEMORY_ORDER_LINKS_OFFSET;
        ldr.in_initialization_order_links.flink =
            next_flink + LdrDataTableEntry64::IN_INITIALIZATION_ORDER_LINKS_OFFSET;
        ldr.in_initialization_order_links.blink =
            prev_flink + LdrDataTableEntry64::IN_INITIALIZATION_ORDER_LINKS_OFFSET;
    } else {
        ldr.in_load_order_links.flink = space_addr;
        ldr.in_load_order_links.blink = space_addr;
        ldr.in_memory_order_links.flink =
            space_addr + LdrDataTableEntry64::IN_MEMORY_ORDER_LINKS_OFFSET;
        ldr.in_memory_order_links.blink =
            space_addr + LdrDataTableEntry64::IN_MEMORY_ORDER_LINKS_OFFSET;
        ldr.in_initialization_order_links.flink =
            space_addr + LdrDataTableEntry64::IN_INITIALIZATION_ORDER_LINKS_OFFSET;
        ldr.in_initialization_order_links.blink =
            space_addr + LdrDataTableEntry64::IN_INITIALIZATION_ORDER_LINKS_OFFSET;
    }
    ldr.hash_links.flink = hash_links_addr;
    ldr.hash_links.blink = hash_links_addr;
    ldr.dll_base = base_addr;
    ldr.entry_point = entry_point;
    ldr.size_of_image = image_sz;
    ldr.full_dll_name.length = names.full_capacity - 2;
    ldr.full_dll_name.maximum_length = names.full_capacity;
    ldr.full_dll_name.buffer = space_addr + LdrDataTableEntry64::size();
    ldr.base_dll_name.length = names.base_capacity - 2;
    ldr.base_dll_name.maximum_length = names.base_capacity;
    ldr.base_dll_name.buffer = ldr.full_dll_name.buffer + u64::from(names.full_capacity);
    // LDRP_IMAGE_DLL | LDRP_ENTRY_PROCESSED | LDRP_PROCESS_ATTACH_CALLED
    // | LDRP_LOAD_NOTIFICATIONS_SENT — tells the real ntdll loader that
    // this module is fully loaded and doesn't need re-processing.
    ldr.flags = LDR_SYSTEM_MODULE_FLAGS;
    ldr.load_count = u16::MAX;
    ldr.tls_index = 0;
    mem.write_wide_string(ldr.full_dll_name.buffer, &names.full);
    mem.write_wide_string(ldr.base_dll_name.buffer, libname);
    ldr.save(space_addr, &mut emu.maps);

    // Create a DDAG node so Win11's parallel loader treats this module as
    // fully loaded (State = 9 = LdrModulesReadyToRun, LoadCount = -1).
    {
        static DDAG_SEQ: AtomicU32 = AtomicU32::new(1000);
        if let Some(ddag_addr) = emu.maps.alloc(DDAG_NODE_SIZE) {
            let seq = DDAG_SEQ.fetch_add(1, Ordering::Relaxed);
            if emu
                .maps
                .create_map(
                    &format!("ddag_node.{}", seq),
                    ddag_addr,
                    DDAG_NODE_SIZE,
                    Permission::READ_WRITE,
                )
                .is_ok()
            {
                emu.maps
                    .write_dword(ddag_addr + DDAG_LOAD_COUNT_OFFSET, u32::MAX);
                emu.maps
                    .write_dword(ddag_addr + DDAG_STATE_OFFSET, LDR_MODULES_READY_TO_RUN);
                emu.maps.write_qword(
                    space_addr + LdrDataTableEntry64::DDAG_NODE_OFFSET,
                    ddag_addr,
                );
            }
        }
    }

    space_addr
}

/// Patch the main image's `FullDllName`/`BaseDllName` in its LDR entry.
///
/// Some ntdll versions allocate the EXE module's name buffer during
/// `LdrInitializeThunk` (`LdrpInitializeProcess` → `RtlGetFullPathName_UEx`)
/// but, under emulation, never actually copy the path into it: the
/// UNICODE_STRING ends up pointing at uninitialized heap (the `0xFEEEFEEE`
/// fill), and that heap block is later freed and reused. PEB-walking code that
/// reads the EXE module name then sees garbage. We can't rely on the guest's
/// path-resolution running, so we overwrite the EXE entry's names with a
/// stable, mwemu-owned buffer that the guest heap never recycles.
pub fn fix_exe_module_name(emu: &mut emu::Emu) {
    let exe_base = emu.base;
    if exe_base == 0 {
        return;
    }
    let Some(peb_base) = emu.maps.get_map_by_name("peb").map(|m| m.get_base()) else {
        return;
    };
    let ldr_addr = emu
        .maps
        .read_qword(peb_base + Peb64::LDR_OFFSET)
        .unwrap_or(0);
    if ldr_addr == 0 {
        return;
    }

    // Locate the EXE entry by DllBase, walking InLoadOrderModuleList
    // (sentinel at PEB_LDR_DATA+0x10; cur == &entry.InLoadOrderLinks == entry+0).
    let sentinel = ldr_addr + PebLdrData64::IN_LOAD_ORDER_MODULE_LIST_OFFSET;
    let mut cur = emu.maps.read_qword(sentinel).unwrap_or(0);
    let mut entry = 0u64;
    let mut guard = 0;
    while cur != 0 && cur != sentinel && guard < 64 {
        if emu
            .maps
            .read_qword(cur + LdrDataTableEntry64::DLL_BASE_OFFSET)
            .unwrap_or(0)
            == exe_base
        {
            entry = cur;
            break;
        }
        cur = emu.maps.read_qword(cur).unwrap_or(0);
        guard += 1;
    }
    if entry == 0 {
        return;
    }

    let path = main_module_path(emu);
    let Some(names) = LdrNames::new(&path) else {
        return;
    };

    let total = names.storage_size();
    let buf = if let Some(map) = emu.maps.get_map_by_name("exe_ldr_name")
        && map.size() as u64 >= total
    {
        map.get_base()
    } else {
        let name = emu.maps.unique_map_name("exe_ldr_name");
        emu.maps.map(&name, total, Permission::READ_WRITE)
    };
    if buf == 0 {
        return;
    }
    let base_buf = buf + u64::from(names.full_capacity);
    emu.maps.write_wide_string(buf, &names.full);
    emu.maps.write_wide_string(base_buf, names.base);

    for (offset, buffer, capacity) in [
        (
            LdrDataTableEntry64::FULL_DLL_NAME_OFFSET,
            buf,
            names.full_capacity,
        ),
        (
            LdrDataTableEntry64::BASE_DLL_NAME_OFFSET,
            base_buf,
            names.base_capacity,
        ),
    ] {
        let descriptor = entry + offset;
        emu.maps
            .write_word(descriptor, capacity - size_of::<u16>() as u16);
        emu.maps.write_word(
            descriptor + UnicodeString64::MAXIMUM_LENGTH_OFFSET,
            capacity,
        );
        emu.maps
            .write_qword(descriptor + UnicodeString64::BUFFER_OFFSET, buffer);
    }
}

fn ldr_hash_bucket_index(libname: &str) -> u64 {
    let mut hash: u32 = 0;
    for ch in libname.encode_utf16() {
        let folded = if ch >= b'a' as u16 && ch <= b'z' as u16 {
            ch - 0x20
        } else {
            ch
        };
        hash = hash
            .wrapping_mul(LDR_HASH_MULTIPLIER)
            .wrapping_add(folded as u32);
    }
    u64::from(hash) % LDR_HASH_BUCKET_COUNT
}

fn rebuild_ldr_hash_table(emu: &mut emu::Emu, modules: &[ModInfo], entries: &[u64]) {
    let Some(ntdll_map) = emu.maps.get_map_by_name("ntdll.pe") else {
        return;
    };
    let table = ntdll_map.get_base() + NTDLL_LDRP_HASH_TABLE_RVA64;

    for i in 0..LDR_HASH_BUCKET_COUNT {
        let head = table + i * ListEntry64::SIZE;
        emu.maps.write_qword(head, head);
        emu.maps.write_qword(head + ListEntry64::BLINK_OFFSET, head);
    }

    for (module, entry) in modules.iter().zip(entries.iter()) {
        let bucket = ldr_hash_bucket_index(&module.name);
        let head = table + bucket * ListEntry64::SIZE;
        let hash_links = *entry + LdrDataTableEntry64::HASH_LINKS_OFFSET;
        let tail = emu
            .maps
            .read_qword(head + ListEntry64::BLINK_OFFSET)
            .unwrap_or(head);

        emu.maps.write_qword(hash_links, head);
        emu.maps
            .write_qword(hash_links + ListEntry64::BLINK_OFFSET, tail);
        emu.maps.write_qword(tail, hash_links);
        emu.maps
            .write_qword(head + ListEntry64::BLINK_OFFSET, hash_links);
    }
}

/// Populate the ASCII portion of ntdll's NLS upcase table so that
/// case-insensitive name hashing (used by `LdrGetProcedureAddress`) returns
/// non-zero values. Real ntdll fills this from the NLS data file; under
/// `--ssdt` no NLS section is provided, so the buffer pointed to by the
/// global is left zeroed and every export lookup fails with
/// `STATUS_ENTRYPOINT_NOT_FOUND`. Only entries 0x00..0x7F matter for
/// imports/exports, which use ASCII names.
pub fn populate_nls_upcase_table(emu: &mut emu::Emu) {
    let Some(ntdll_map) = emu.maps.get_map_by_name("ntdll.pe") else {
        return;
    };
    let global_ptr = ntdll_map.get_base() + NTDLL_NLS_UPCASE_TABLE_PTR_RVA;
    let table = emu.maps.read_qword(global_ptr).unwrap_or(0);
    if table == 0 || !emu.maps.is_mapped(table) {
        return;
    }
    for b in 0u32..256 {
        let upper = if (b'a' as u32..=b'z' as u32).contains(&b) {
            b - 0x20
        } else {
            b
        };
        let _ = emu.maps.write_word(table + (b as u64) * 2, upper as u16);
    }
}

pub fn ensure_ntdll_loader_globals(emu: &mut emu::Emu) {
    let Some(ntdll_map) = emu.maps.get_map_by_name("ntdll.pe") else {
        return;
    };
    let base = ntdll_map.get_base();

    let list_2680 = base + NTDLL_LDRP_GLOBAL_2680_RVA64;
    emu.maps.write_qword(list_2680, list_2680);
    emu.maps
        .write_qword(list_2680 + ListEntry64::BLINK_OFFSET, list_2680);

    let state_26c0 = base + NTDLL_LDRP_GLOBAL_26C0_RVA64;
    emu.maps.write_qword(state_26c0, u64::MAX);

    let list_26f0 = base + NTDLL_LDRP_GLOBAL_26F0_RVA64;
    emu.maps.write_qword(list_26f0, list_26f0);
    emu.maps
        .write_qword(list_26f0 + ListEntry64::BLINK_OFFSET, list_26f0);

    // Ensure the loader lock CRITICAL_SECTION is in a valid, unlocked state.
    // If LdrInitializeThunk did not fully run, the CS may be all-zero (DebugInfo=null,
    // LockCount=0). RtlEnterCriticalSection then takes the slow path and crashes
    // trying to increment [null+0x24].  Setting DebugInfo=-1 suppresses the debug
    // accounting, and LockCount=-1 (bit0=1) puts the CS into the "unlocked" state.
    let loader_lock = base + NTDLL_LOADER_LOCK_CS_RVA;
    if emu.maps.is_mapped(loader_lock) {
        let debug_info = emu.maps.read_qword(loader_lock).unwrap_or(0);
        if debug_info == 0 {
            let _ = emu.maps.write_qword(loader_lock, u64::MAX); // DebugInfo = -1 (no debug)
            let _ = emu
                .maps
                .write_dword(loader_lock + CRITICAL_SECTION_LOCK_COUNT_OFFSET, u32::MAX); // LockCount = -1 (unlocked)
        }
    }
}

struct ModInfo {
    name: String,
    base: u64,
}

fn rebuild_ldrp_module_base_address_index(emu: &mut emu::Emu, entries: &[u64]) {
    if entries.is_empty() {
        return;
    }
    let Some(ntdll_map) = emu.maps.get_map_by_name("ntdll.pe") else {
        return;
    };
    let ntdll_base = ntdll_map.get_base();

    let tree_root_ptr = ntdll_base + NTDLL_LDRP_MODULE_BASE_ADDRESS_INDEX_RVA;
    let tree_min_ptr = ntdll_base + NTDLL_LDRP_MODULE_BASE_ADDRESS_INDEX_RVA + RB_TREE_MIN_OFFSET;

    let mut items: Vec<(u64, u64)> = entries
        .iter()
        .filter_map(|&entry| {
            let dll_base = emu
                .maps
                .read_qword(entry + LdrDataTableEntry64::DLL_BASE_OFFSET)
                .unwrap_or(0);
            if dll_base == 0 {
                None
            } else {
                Some((dll_base, entry))
            }
        })
        .collect();
    items.sort_by_key(|&(base, _)| base);
    if items.is_empty() {
        return;
    }

    static DDAG_SEQ: AtomicU32 = AtomicU32::new(0);
    let mut node_addrs: Vec<u64> = Vec::with_capacity(items.len());

    for &(_, entry_addr) in &items {
        let ddag_addr = match emu.maps.alloc(DDAG_NODE_SIZE) {
            Some(a) => a,
            None => {
                log::error!("cannot alloc DdagNode");
                continue;
            }
        };
        let seq = DDAG_SEQ.fetch_add(1, Ordering::Relaxed);
        if let Err(e) = emu.maps.create_map(
            &format!("ddag_node.{}", seq),
            ddag_addr,
            DDAG_NODE_SIZE,
            Permission::READ_WRITE,
        ) {
            log::error!("cannot create DdagNode map: {}", e);
            continue;
        }

        // LDR_DDAG_NODE.LoadCount (+0x18) == -1 means "untracked / system" module.
        let _ = emu
            .maps
            .write_dword(ddag_addr + DDAG_LOAD_COUNT_OFFSET, u32::MAX);
        // LDR_DDAG_NODE.State (+0x38) must be >= 9 (LdrModulesInitialized).
        let _ = emu
            .maps
            .write_dword(ddag_addr + DDAG_STATE_OFFSET, LDR_MODULES_READY_TO_RUN);

        let _ = emu.maps.write_qword(
            entry_addr + LdrDataTableEntry64::DDAG_NODE_OFFSET,
            ddag_addr,
        );
        node_addrs.push(entry_addr + LdrDataTableEntry64::BASE_ADDRESS_INDEX_NODE_OFFSET);
    }

    // Build a right-skewed BST (valid, simple).
    // RTL_BALANCED_NODE: [+0x00] Left, [+0x08] Right, [+0x10] ParentValue
    let n = node_addrs.len();
    for i in 0..n {
        let node = node_addrs[i];
        let left = 0u64;
        let right = if i + 1 < n { node_addrs[i + 1] } else { 0 };
        let parent = if i > 0 { node_addrs[i - 1] } else { 0 };
        let _ = emu.maps.write_qword(node, left);
        let _ = emu
            .maps
            .write_qword(node + BALANCED_NODE_RIGHT_OFFSET, right);
        let _ = emu
            .maps
            .write_qword(node + BALANCED_NODE_PARENT_OFFSET, parent);
    }

    let _ = emu.maps.write_qword(tree_root_ptr, node_addrs[0]);
    let _ = emu.maps.write_qword(tree_min_ptr, 0);

    log::trace!(
        "rebuild_ldrp_module_base_address_index: {} modules, root node=0x{:x}",
        n,
        node_addrs[0]
    );
}

pub fn rebuild_ldr_lists(emu: &mut emu::Emu) {
    ensure_peb_system_dependent_07(emu);
    let peb_addr = emu.maps.get_mem("peb").get_base();
    let ldr_addr = emu
        .maps
        .read_qword(peb_addr + Peb64::LDR_OFFSET)
        .unwrap_or(0);
    if ldr_addr == 0 {
        return;
    }

    let exe_base = emu.base;
    let exe_name = emu.cfg.exe_name.clone();

    let pe_names: Vec<String> = emu
        .maps
        .name_map
        .keys()
        .filter(|n| n.ends_with(".pe"))
        .cloned()
        .collect();

    let mut modules: Vec<ModInfo> = vec![ModInfo {
        name: exe_name,
        base: exe_base,
    }];

    for map_name in &pe_names {
        let stem = map_name.trim_end_matches(".pe");
        if stem.eq_ignore_ascii_case("ntdll")
            && let Some(m) = emu.maps.get_map_by_name(map_name)
        {
            modules.push(ModInfo {
                name: "ntdll.dll".into(),
                base: m.get_base(),
            });
        }
    }

    for map_name in &pe_names {
        let stem = map_name.trim_end_matches(".pe");
        if stem.eq_ignore_ascii_case("kernel32")
            && let Some(m) = emu.maps.get_map_by_name(map_name)
        {
            modules.push(ModInfo {
                name: "kernel32.dll".into(),
                base: m.get_base(),
            });
        }
    }

    for map_name in &pe_names {
        let stem = map_name.trim_end_matches(".pe");
        let sl = stem.to_lowercase();
        if sl == "ntdll" || sl == "kernel32" {
            continue;
        }
        let exe_stem = emu
            .cfg
            .filename
            .split('/')
            .next_back()
            .unwrap_or("")
            .split('.')
            .next()
            .unwrap_or("");
        if stem.eq_ignore_ascii_case(exe_stem) {
            continue;
        }
        if let Some(m) = emu.maps.get_map_by_name(map_name) {
            modules.push(ModInfo {
                name: format!("{}.dll", stem),
                base: m.get_base(),
            });
        }
    }

    if modules.is_empty() {
        return;
    }

    let mut entries: Vec<u64> = Vec::new();
    let image_path = main_module_path(emu);
    for (i, m) in modules.iter().enumerate() {
        let entry_point = if m.base > 0 {
            let pe_hdr = emu
                .maps
                .read_dword(m.base + PE_POINTER_OFFSET as u64)
                .unwrap_or(0) as u64;
            if pe_hdr > 0 {
                let ep_rva = emu
                    .maps
                    .read_dword(m.base + pe_hdr + ENTRY_POINT_RVA_OFFSET)
                    .unwrap_or(0) as u64;
                m.base + ep_rva
            } else {
                0
            }
        } else {
            0
        };
        let path = if i == 0 { &image_path } else { &m.name };
        let addr = create_ldr_entry(emu, m.base, entry_point, path, 0, 0);
        entries.push(addr);
    }

    // Link entries using LDR sentinel nodes as terminators (so ntdll end-of-list
    // detection works: Flink == sentinel means end of list).
    let lists = [
        (0, PebLdrData64::IN_LOAD_ORDER_MODULE_LIST_OFFSET),
        (
            LdrDataTableEntry64::IN_MEMORY_ORDER_LINKS_OFFSET,
            PebLdrData64::IN_MEMORY_ORDER_MODULE_LIST_OFFSET,
        ),
        (
            LdrDataTableEntry64::IN_INITIALIZATION_ORDER_LINKS_OFFSET,
            PebLdrData64::IN_INITIALIZATION_ORDER_MODULE_LIST_OFFSET,
        ),
    ];
    for (offset, head_offset) in lists {
        let head = ldr_addr + head_offset;
        let mut tail = head;
        for &entry in &entries {
            let node = entry + offset;
            emu.maps.write_qword(tail, node);
            emu.maps.write_qword(node + ListEntry64::BLINK_OFFSET, tail);
            tail = node;
        }
        emu.maps.write_qword(tail, head);
        emu.maps.write_qword(head + ListEntry64::BLINK_OFFSET, tail);
    }
    emu.maps
        .write_dword(ldr_addr + PebLdrData64::INITIALIZED_OFFSET, 1);

    rebuild_ldr_hash_table(emu, &modules, &entries);
    ensure_ntdll_loader_globals(emu);
    // NOTE: do NOT call `populate_nls_upcase_table` here. ntdll has already
    // hashed module names with whatever upcase table it built during
    // `LdrInitializeThunk`; mutating the table afterwards would cause
    // `LdrpUnicodeStringToHash` to compute different hashes and miss every
    // entry it just registered. If a fix-up is ever needed, do it BEFORE
    // ntdll's first hash use, not after.
    rebuild_ldrp_module_base_address_index(emu, &entries);

    if exe_base != 0 {
        emu.maps
            .write_qword(peb_addr + Peb64::IMAGE_BASE_ADDR_OFFSET, exe_base);
    }

    log::trace!("rebuild_ldr_lists: rebuilt with {} modules", modules.len());
}
