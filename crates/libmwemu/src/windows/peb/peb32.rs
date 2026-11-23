use super::{
    EXPORT_DIRECTORY_OFFSET32, IMAGE_SIZE_OFFSET32, LDR_NAME_RESERVE, LdrNames, command_line,
    map_wide_string, unlink_dynamic_entry,
};
use crate::debug::console::Console;
use crate::emu;
use crate::maps::mem64::Permission;
use crate::winapi::common::modules::main_module_path;
use crate::winapi::environment_store;
use crate::windows::structures::{
    Heap32, ImageExportDirectory, LdrDataTableEntry, ListEntry, OrdinalTable, Peb, PebLdrData,
    RtlUserProcessParameters32, TEB,
};
use goblin::pe::header::PE_POINTER_OFFSET;

pub fn init_ldr(emu: &mut emu::Emu) -> u64 {
    let ldr_sz = PebLdrData::size() + 100;
    let ldr_addr = match emu.maps.lib32_alloc(ldr_sz as u64) {
        Some(a) => a,
        None => {
            log::error!("cannot alloc the LDR");
            return 0;
        }
    };
    log::debug!("LDR ALLOCATED AT: 0x{:x}", ldr_addr);
    if let Err(e) = emu
        .maps
        .create_map("ldr", ldr_addr, ldr_sz as u64, Permission::READ_WRITE)
    {
        log::error!("cannot create ldr map: {}", e);
        return 0;
    }
    let image_path = main_module_path(emu);
    let module_entry = create_ldr_entry(emu, 0, 0, &image_path, 0, 0) as u32;
    let mut ldr = PebLdrData::new();
    ldr.initializated = 1;
    ldr.in_load_order_module_list.flink = module_entry;
    ldr.in_load_order_module_list.blink = module_entry;
    ldr.in_memory_order_module_list.flink =
        module_entry + LdrDataTableEntry::IN_MEMORY_ORDER_LINKS_OFFSET as u32;
    ldr.in_memory_order_module_list.blink =
        module_entry + LdrDataTableEntry::IN_MEMORY_ORDER_LINKS_OFFSET as u32;
    ldr.in_initialization_order_module_list.flink =
        module_entry + LdrDataTableEntry::IN_INITIALIZATION_ORDER_LINKS_OFFSET as u32;
    ldr.in_initialization_order_module_list.blink =
        module_entry + LdrDataTableEntry::IN_INITIALIZATION_ORDER_LINKS_OFFSET as u32;
    ldr.entry_in_progress = module_entry;
    ldr.save(ldr_addr, &mut emu.maps);

    ldr_addr
}

pub fn init_arguments(emu: &mut emu::Emu) -> u64 {
    let addr = emu.maps.map(
        "RtlUserProcessParameters32",
        RtlUserProcessParameters32::size() as u64,
        Permission::READ_WRITE_EXECUTE,
    );
    let mut params_struct = RtlUserProcessParameters32::new();

    let image_path = main_module_path(emu);
    let params = command_line(&image_path, &emu.cfg.arguments);
    let filename_len = (image_path.encode_utf16().count() as u64 + 1) * 2;
    let cmdline_len = (params.encode_utf16().count() as u64 + 1) * 2;

    let filename = map_wide_string(&mut emu.maps, "file_name", &image_path, filename_len);
    let cmdline = map_wide_string(&mut emu.maps, "command_line", &params, cmdline_len);

    params_struct.image_path_name.length = (filename_len - 2) as u16;
    params_struct.image_path_name.maximum_length = filename_len as u16;
    params_struct.image_path_name.buffer = filename as u32;

    params_struct.command_line.length = (cmdline_len - 2) as u16;
    params_struct.command_line.maximum_length = cmdline_len as u16;
    params_struct.command_line.buffer = cmdline as u32;

    params_struct.save(addr, &mut emu.maps);
    if let Err(error) = environment_store::initialize(emu) {
        log::error!("cannot initialize the process environment: {error}");
        return 0;
    }
    addr
}

fn init_process_heap(emu: &mut emu::Emu) -> u32 {
    let heap_sz: u64 = 0x400;
    let h = match emu.maps.lib32_alloc(heap_sz) {
        Some(a) => a,
        None => {
            log::error!("cannot alloc fake ProcessHeap (32-bit)");
            return 0;
        }
    };
    if let Err(e) = emu
        .maps
        .create_map("process_heap", h, heap_sz, Permission::READ_WRITE)
    {
        log::error!("cannot create process_heap map: {}", e);
        return 0;
    }
    emu.maps.memset(h, 0, heap_sz as usize);

    let heap = Heap32::new(h as u32, heap_sz as u32);
    heap.save(emu.maps.get_mem_mut("process_heap"));

    let lock_addr = h + 0x300;
    emu.maps.write_dword(lock_addr + 0x04, 0xFFFF_FFFF);

    h as u32
}

pub fn init_peb(emu: &mut emu::Emu) {
    let ldr = init_ldr(emu);
    let args_addr = init_arguments(emu);
    let heap_addr = init_process_heap(emu);

    let peb_addr = match emu.maps.lib32_alloc(Peb::size() as u64) {
        Some(a) => a,
        None => {
            log::error!("cannot alloc the PEB32");
            return;
        }
    };
    let peb_map =
        match emu
            .maps
            .create_map("peb", peb_addr, Peb::size() as u64, Permission::READ_WRITE)
        {
            Ok(m) => m,
            Err(e) => {
                log::error!("cannot create peb map: {}", e);
                return;
            }
        };
    let peb = Peb::new(0, ldr as u32, args_addr as u32);
    peb.save(peb_map);
    // Write ProcessHeap at PEB32 offset +0x18
    emu.maps
        .write_dword(peb_addr + Peb::PROCESS_HEAP_OFFSET, heap_addr);

    let teb_addr = match emu.maps.lib32_alloc(TEB::map_size() as u64) {
        Some(a) => a,
        None => {
            log::error!("cannot alloc the TEB32");
            return;
        }
    };
    if let Err(e) = emu.maps.create_map(
        "teb",
        teb_addr,
        TEB::map_size() as u64,
        Permission::READ_WRITE,
    ) {
        log::error!("cannot create teb map: {}", e);
        return;
    }
    emu.maps.memset(teb_addr, 0, TEB::map_size());
    let mut teb = TEB::new(peb_addr as u32);
    teb.thread_id = emu.current_thread().id as u32;
    teb.save(emu.maps.get_mem_mut("teb"));
}

pub fn update_peb_image_base(emu: &mut emu::Emu, base: u32) {
    let peb = emu.maps.get_mem("peb");
    let peb_base = peb.get_base();
    emu.maps
        .write_dword(peb_base + Peb::IMAGE_BASE_ADDR_OFFSET, base);
}

#[derive(Debug)]
pub struct Flink {
    flink_addr: u64,
    pub ldr_entry: LdrDataTableEntry,
    pub mod_base: u64,
    pub mod_name: String,
    pub pe_hdr: u64,

    pub export_table_rva: u64,
    pub export_table: u64,
    pub num_of_funcs: u64,
    pub func_name_tbl_rva: u64,
    pub func_name_tbl: u64,
}

impl Flink {
    pub fn save(&mut self, emu: &mut emu::Emu) {}

    pub fn new(emu: &mut emu::Emu) -> Flink {
        let peb = emu.maps.get_mem("peb");
        let peb_base = peb.get_base();
        let ldr_addr = peb.read_dword(peb_base + Peb::LDR_OFFSET) as u64; // peb->ldr

        let ldr = PebLdrData::load(ldr_addr, &emu.maps);

        let flink = emu
            .maps
            .read_dword(ldr.in_load_order_module_list.flink.into())
            .unwrap_or(0) as u64;

        Flink {
            flink_addr: flink,
            ldr_entry: LdrDataTableEntry::load(flink, &emu.maps),
            mod_base: 0,
            mod_name: String::new(),
            pe_hdr: 0,
            export_table_rva: 0,
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

    pub fn load(&mut self, emu: &mut emu::Emu) -> bool {
        self.get_mod_base();
        self.get_mod_name(emu);
        self.get_pe_hdr(emu);
        self.get_export_table(emu)
    }

    fn get_mod_base(&mut self) {
        self.mod_base = self.ldr_entry.dll_base as u64;
        if self.mod_base == 0 {
            log::warn!("modbase is zero");
        }
    }

    pub fn set_mod_base(&mut self, base: u64, emu: &mut emu::Emu) {
        self.ldr_entry.dll_base = base as u32;
        emu.maps.write_dword(
            self.flink_addr + LdrDataTableEntry::DLL_BASE_OFFSET,
            base as u32,
        ); // dll_base
    }

    pub fn get_mod_name(&mut self, emu: &mut emu::Emu) {
        let mod_name_ptr = self.ldr_entry.base_dll_name.buffer as u64;

        self.mod_name = emu.maps.read_wide_string(mod_name_ptr);
    }

    pub fn has_module(&self) -> bool {
        if self.mod_base == 0 || self.flink_addr == 0 {
            return false;
        }
        true
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

    pub fn get_export_table(&mut self, emu: &mut emu::Emu) -> bool {
        if self.pe_hdr == 0 {
            return false;
        }

        if self.mod_base == 0 {
            return false;
        }

        self.export_table_rva = match emu
            .maps
            .read_dword(self.mod_base + self.pe_hdr + EXPORT_DIRECTORY_OFFSET32)
        {
            Some(v) => v as u64,
            None => {
                // .expect("error reading export_table_rva") as u64;
                return false;
            }
        };

        if self.export_table_rva == 0 {
            return false;
        }

        self.export_table = self.export_table_rva + self.mod_base;
        self.num_of_funcs = match emu
            .maps
            .read_dword(self.export_table + ImageExportDirectory::NUMBER_OF_NAMES_OFFSET)
        {
            Some(num_of_funcs) => num_of_funcs as u64,
            None => {
                log::trace!(
                    "error reading export_table 0x{:x} = 0x{:x} + 0x{:x}",
                    self.export_table,
                    self.export_table_rva,
                    self.mod_base
                );
                0
            }
        };

        if self.num_of_funcs > 0 {
            self.func_name_tbl_rva = emu
                .maps
                .read_dword(self.export_table + ImageExportDirectory::ADDRESS_OF_NAMES_OFFSET)
                .unwrap_or(0) as u64;
            self.func_name_tbl = self.func_name_tbl_rva + self.mod_base;
        }

        true
    }

    pub fn get_function_ordinal(&self, emu: &mut emu::Emu, function_id: u64) -> OrdinalTable {
        let mut ordinal = OrdinalTable::new();

        let func_name_rva = match emu.maps.read_dword(self.func_name_tbl + function_id * 4) {
            Some(addr) => addr as u64,
            None => return ordinal,
        };

        if func_name_rva == 0 {
            ordinal.func_name = "-".to_string();
        } else {
            if self.mod_base == 0 {
                return ordinal;
            }
            ordinal.func_name = emu.maps.read_string(func_name_rva + self.mod_base);
        }

        if ordinal.func_name == "VCOMPort" {
            Console::spawn_console(emu);
        }

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
        ordinal.func_va = ordinal.func_rva + self.mod_base;

        ordinal
    }

    pub fn get_next_flink(&self, emu: &mut emu::Emu) -> u64 {
        emu.maps.read_dword(self.flink_addr).unwrap_or(0) as u64
    }

    pub fn get_prev_flink(&self, emu: &mut emu::Emu) -> u64 {
        emu.maps
            .read_dword(self.flink_addr + ListEntry::BLINK_OFFSET)
            .unwrap_or(0) as u64
    }

    pub fn next(&mut self, emu: &mut emu::Emu) {
        self.flink_addr = self.get_next_flink(emu);
        self.ldr_entry = LdrDataTableEntry::load(self.flink_addr, &emu.maps);
        self.load(emu);
    }
}

pub fn get_module_base(libname: &str, emu: &mut emu::Emu) -> Option<u64> {
    let mut libname2: String = libname.to_string().to_lowercase();
    if !libname2.ends_with(".dll") {
        libname2.push_str(".dll");
    }

    let mut flink = Flink::new(emu);
    flink.load(emu);
    let first_flink = flink.get_ptr();
    let mut iters = 0usize;
    loop {
        //log::trace!("{} == {}", libname2, flink.mod_name);

        if libname.to_string().to_lowercase() == flink.mod_name.to_string().to_lowercase()
            || libname2 == flink.mod_name.to_string().to_lowercase()
        {
            return Some(flink.mod_base);
        }
        flink.next(emu);
        iters += 1;

        if flink.get_ptr() == first_flink || flink.get_ptr() == 0 || iters > 4096 {
            break;
        }
    }
    None
}

pub fn show_linked_modules(emu: &mut emu::Emu) {
    let mut flink = Flink::new(emu);
    flink.load(emu);
    let first_flink = flink.get_ptr();

    // get last element
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
        flink.next(emu);
        if flink.get_ptr() == first_flink || flink.get_ptr() == 0 {
            return;
        }
    }
}

pub fn update_ldr_entry_base(libname: &str, base: u64, emu: &mut emu::Emu) {
    let mut flink = Flink::new(emu);
    flink.load(emu);
    while flink.mod_name.to_lowercase() != libname.to_lowercase() {
        flink.next(emu);
    }
    flink.set_mod_base(base, emu);
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
        flink.next(emu);
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
    /*
     * LoadLibary* family triggers this.
     */

    let peb_addr = emu.maps.get_mem("peb").get_base();
    let ldr_addr = emu.maps.read_dword(peb_addr + Peb::LDR_OFFSET).unwrap_or(0) as u64;
    if ldr_addr == 0 {
        return 0;
    }

    // The 32-bit lists are circular and use the first entry as the head rather
    // than an in-list sentinel node. Append to the tail so initialization order
    // remains visible to PEB walkers: EXE, ntdll, kernel32, kernelbase, ... .
    let first_flink = emu
        .maps
        .read_dword(ldr_addr + PebLdrData::IN_LOAD_ORDER_MODULE_LIST_OFFSET)
        .unwrap_or(0) as u64;
    if first_flink == 0 {
        return 0;
    }
    let mut tail = first_flink;
    for _ in 0..4096 {
        let next = emu.maps.read_dword(tail).unwrap_or(0) as u64;
        if next == 0 {
            return 0;
        }
        if next == first_flink {
            break;
        }
        tail = next;
    }

    let space_addr = create_ldr_entry(
        emu,
        base as u32,
        pe_off,
        libname,
        first_flink as u32,
        tail as u32,
    );

    if space_addr == 0 {
        return 0;
    }

    // Link the new entry after the current tail and before the first entry.
    emu.maps.write_dword(tail, space_addr as u32);
    emu.maps.write_dword(
        tail + LdrDataTableEntry::IN_MEMORY_ORDER_LINKS_OFFSET,
        (space_addr + LdrDataTableEntry::IN_MEMORY_ORDER_LINKS_OFFSET) as u32,
    );
    emu.maps.write_dword(
        tail + LdrDataTableEntry::IN_INITIALIZATION_ORDER_LINKS_OFFSET,
        (space_addr + LdrDataTableEntry::IN_INITIALIZATION_ORDER_LINKS_OFFSET) as u32,
    );
    emu.maps
        .write_dword(first_flink + ListEntry::BLINK_OFFSET, space_addr as u32);
    emu.maps.write_dword(
        first_flink + LdrDataTableEntry::IN_MEMORY_ORDER_LINKS_OFFSET + ListEntry::BLINK_OFFSET,
        (space_addr + LdrDataTableEntry::IN_MEMORY_ORDER_LINKS_OFFSET) as u32,
    );
    emu.maps.write_dword(
        first_flink
            + LdrDataTableEntry::IN_INITIALIZATION_ORDER_LINKS_OFFSET
            + ListEntry::BLINK_OFFSET,
        (space_addr + LdrDataTableEntry::IN_INITIALIZATION_ORDER_LINKS_OFFSET) as u32,
    );

    // Keep the list-head Blink values synchronized with the circular entries.
    emu.maps.write_dword(
        ldr_addr + PebLdrData::IN_LOAD_ORDER_MODULE_LIST_OFFSET + ListEntry::BLINK_OFFSET,
        space_addr as u32,
    );
    emu.maps.write_dword(
        ldr_addr + PebLdrData::IN_MEMORY_ORDER_MODULE_LIST_OFFSET + ListEntry::BLINK_OFFSET,
        (space_addr + LdrDataTableEntry::IN_MEMORY_ORDER_LINKS_OFFSET) as u32,
    );
    emu.maps.write_dword(
        ldr_addr + PebLdrData::IN_INITIALIZATION_ORDER_MODULE_LIST_OFFSET + ListEntry::BLINK_OFFSET,
        (space_addr + LdrDataTableEntry::IN_INITIALIZATION_ORDER_LINKS_OFFSET) as u32,
    );
    space_addr
}

pub fn create_ldr_entry(
    emu: &mut emu::Emu,
    base: u32,
    entry_point: u32,
    libname: &str,
    next_flink: u32,
    prev_flink: u32,
) -> u64 {
    let Some(names) = LdrNames::new(libname) else {
        log::error!("module path exceeds the UNICODE_STRING length limit");
        return 0;
    };
    let libname = names.base;
    let base_addr;
    let sz = LdrDataTableEntry::size() as u64 + names.storage_size().max(LDR_NAME_RESERVE);
    let space_addr = match emu.maps.alloc(sz) {
        Some(a) => a,
        None => {
            log::error!("cannot alloc LDR for LoadLibraryA");
            return 0;
        }
    };
    let mut lib = libname.to_string();
    lib.push_str(".ldr");
    let mut image_sz = 0;
    if base > 0 {
        let pe_hdr = emu
            .maps
            .read_dword(base as u64 + PE_POINTER_OFFSET as u64)
            .unwrap_or(0) as u64;
        image_sz = emu
            .maps
            .read_dword(base as u64 + pe_hdr + IMAGE_SIZE_OFFSET32)
            .unwrap_or(0) as u64;
        base_addr = base;
    } else {
        let addr = match emu.maps.alloc(sz) {
            Some(a) => a,
            None => {
                log::error!("out of memory, cannot create the .ldr entry");
                return 0;
            }
        };
        if addr > u32::MAX as u64 {
            {
                log::warn!("allocating .ldr  > u32::MAX");
                return 0;
            }
        }
        base_addr = addr as u32;
    }
    let mem = match emu
        .maps
        .create_map(lib.as_str(), space_addr, sz, Permission::READ_WRITE)
    {
        Ok(m) => m,
        Err(e) => {
            log::error!("create_ldr_entry cannot create map: {}", e);
            return 0;
        }
    };
    mem.write_byte(space_addr + sz - 1, 0x61);

    let mut ldr = LdrDataTableEntry::new();
    if next_flink != 0 {
        ldr.in_load_order_links.flink = next_flink;
        ldr.in_load_order_links.blink = prev_flink;
        ldr.in_memory_order_links.flink =
            next_flink + LdrDataTableEntry::IN_MEMORY_ORDER_LINKS_OFFSET as u32;
        ldr.in_memory_order_links.blink =
            prev_flink + LdrDataTableEntry::IN_MEMORY_ORDER_LINKS_OFFSET as u32;
        ldr.in_initialization_order_links.flink =
            next_flink + LdrDataTableEntry::IN_INITIALIZATION_ORDER_LINKS_OFFSET as u32;
        ldr.in_initialization_order_links.blink =
            prev_flink + LdrDataTableEntry::IN_INITIALIZATION_ORDER_LINKS_OFFSET as u32;
    } else {
        ldr.in_load_order_links.flink = space_addr as u32;
        ldr.in_load_order_links.blink = space_addr as u32;
        ldr.in_memory_order_links.flink =
            space_addr as u32 + LdrDataTableEntry::IN_MEMORY_ORDER_LINKS_OFFSET as u32;
        ldr.in_memory_order_links.blink =
            space_addr as u32 + LdrDataTableEntry::IN_MEMORY_ORDER_LINKS_OFFSET as u32;
        ldr.in_initialization_order_links.flink =
            space_addr as u32 + LdrDataTableEntry::IN_INITIALIZATION_ORDER_LINKS_OFFSET as u32;
        ldr.in_initialization_order_links.blink =
            space_addr as u32 + LdrDataTableEntry::IN_INITIALIZATION_ORDER_LINKS_OFFSET as u32;
    }
    ldr.dll_base = base_addr;
    ldr.entry_point = entry_point;
    ldr.size_of_image = image_sz as u32;
    ldr.full_dll_name.length = names.full_capacity - 2;
    ldr.full_dll_name.maximum_length = names.full_capacity;
    ldr.full_dll_name.buffer = space_addr as u32 + LdrDataTableEntry::size() as u32;
    ldr.base_dll_name.length = names.base_capacity - 2;
    ldr.base_dll_name.maximum_length = names.base_capacity;
    ldr.base_dll_name.buffer = ldr.full_dll_name.buffer + u32::from(names.full_capacity);
    ldr.flags = 0;
    ldr.load_count = 0;
    ldr.tls_index = 0;
    ldr.hash_links.flink = next_flink;
    ldr.hash_links.blink = prev_flink;
    mem.write_wide_string(u64::from(ldr.full_dll_name.buffer), &names.full);
    mem.write_wide_string(u64::from(ldr.base_dll_name.buffer), libname);
    ldr.save(space_addr, &mut emu.maps);

    // http://terminus.rewolf.pl/terminus/structures/ntdll/_LDR_DATA_TABLE_ENTRY_x64.html

    space_addr
}
