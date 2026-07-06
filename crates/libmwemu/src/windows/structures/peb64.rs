use crate::maps::{Maps, mem64::Mem64};

// 64bits
// https://bytepointer.com/resources/tebpeb64.htm   (from xp to win8)
// https://www.tssc.de/winint/Win10_19042_ntoskrnl/_PEB64.htm (win10)

#[derive(Debug)]
pub struct PEB64 {
    inheritet_addr_space: u8,
    read_img_file_exec_options: u8,
    pub being_debugged: u8,
    system_dependent_01: u8,
    dummy_align: u32,
    mutant: u64,
    pub image_base_addr: u64,
    pub ldr: u64,
    process_parameters: u64,
    subsystem_data: u64,
    process_heap: u64,
    fast_peb_lock: u64,
    system_dependent_02: u64,
    system_dependent_03: u64,
    system_dependent_04: u64,
    kernel_callback_table: u64,
    system_reserved: u32,
    system_dependent_05: u32,
    system_dependent_06: u64,
    tls_expansion_counter: u64,
    tls_bitmap: u64,
    tls_bitmap_bits: [u32; 2],
    read_only_shared_memory_base: u64,
    system_dependent_07: u64,
    read_only_static_server_data: u64,
    ansi_code_page_data: u64,
    oem_code_page_data: u64,
    unicode_case_table_data: u64,
    number_of_processors: u32,
    nt_global_flag: u32,
    critical_section_timeout: u64,
    heap_segment_reserve: u64,
    heap_segment_commit: u64,
    heap_decommit_total_free_threshold: u64,
    heap_decommit_free_block_threshold: u64,
    number_of_heaps: u32,
    max_number_of_heaps: u32,
    process_heaps: u64,
    gdi_share_handle_table: u64,
    process_starter_helper: u64,
    gdi_dc_attribute_list: u64,
    loader_lock: u64,
    os_major_version: u32,
    os_minor_version: u32,
    os_build_number: u16,
    oscsd_version: u16,
    os_platform_id: u32,
    image_subsystem: u32,
    image_subsystem_major_version: u32,
    image_subsystem_minor_version: u64,
    active_process_afinity_mask: u64,
    gdi_handle_buffer: [u64; 30],
    post_process_init_routine: u64,
    tls_expansion_bitmap: u64,
    tls_expansion_bitmap_bits: [u32; 32],
    session_id: u64,
    app_compat_flags: u64,
    app_compat_flags_user: u64,
    p_shim_data: u64,
    app_compat_info: u64,
    csd_version: [u64; 2],
    activate_context_data: u64,
    process_assembly_storage_map: u64,
    system_default_activation_context_data: u64,
    system_assembly_storage_map: u64,
    minimum_stack_commit: u64,
}

impl PEB64 {
    pub fn size() -> usize {
        0x1000 // Windows 10 x64 PEB extends to ~0x7B8; use a full page
    }

    pub fn new(image_base_addr: u64, ldr: u64, process_parameters: u64) -> PEB64 {
        PEB64 {
            inheritet_addr_space: 0x0,
            read_img_file_exec_options: 0x0,
            being_debugged: 0x0,
            system_dependent_01: 0x0,
            dummy_align: 0x0,
            mutant: 0xffffffffffffffff,
            image_base_addr,
            ldr,
            process_parameters,
            subsystem_data: 0x0,
            process_heap: 0x520000,
            fast_peb_lock: 0x7710a900,
            system_dependent_02: 0x0,
            system_dependent_03: 0x0,
            system_dependent_04: 0x2,
            kernel_callback_table: 0x76f59500,
            system_reserved: 0x0,
            system_dependent_05: 0x0,
            system_dependent_06: 0x7feff2f0000,
            tls_expansion_counter: 0x0,
            tls_bitmap: 0x77102590,
            tls_bitmap_bits: [0x1fff, 0x0],
            read_only_shared_memory_base: 0x7efe0000,
            system_dependent_07: 0x0,
            read_only_static_server_data: 0x7efe0a90,
            ansi_code_page_data: 0x7fffffb0000,
            oem_code_page_data: 0x7fffffc0228,
            unicode_case_table_data: 0x7fffffd0650,
            number_of_processors: 0x1,
            nt_global_flag: 0x70,
            critical_section_timeout: 0xffffe86d079b8000,
            heap_segment_reserve: 0x100000,
            heap_segment_commit: 0x2000,
            heap_decommit_total_free_threshold: 0x10000,
            heap_decommit_free_block_threshold: 0x10000,
            number_of_heaps: 0x4,
            max_number_of_heaps: 0x10,
            process_heaps: 0x7710a6c0,
            gdi_share_handle_table: 0x920000,
            process_starter_helper: 0x0,
            gdi_dc_attribute_list: 0x14,
            loader_lock: 0x77107490,
            os_major_version: 0x6,
            os_minor_version: 0x1,
            os_build_number: 0x1db1,
            oscsd_version: 0x100,
            os_platform_id: 0x2,
            image_subsystem: 0x3,
            image_subsystem_major_version: 0x5,
            image_subsystem_minor_version: 0x2,
            active_process_afinity_mask: 0x1,
            gdi_handle_buffer: [
                0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
                0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
            ],
            post_process_init_routine: 0x0,
            tls_expansion_bitmap: 0x77102580,
            tls_expansion_bitmap_bits: [
                0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
                0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
            ],
            session_id: 0x1,
            app_compat_flags: 0x0,
            app_compat_flags_user: 0x0,
            p_shim_data: 0x0,
            app_compat_info: 0x0,
            csd_version: [0x1e001c, 0x7efe0afa],
            activate_context_data: 0x0,
            process_assembly_storage_map: 0x0,
            system_default_activation_context_data: 0x230000,
            system_assembly_storage_map: 0x0,
            minimum_stack_commit: 0x0,
        }
    }

    pub fn load(addr: u64, maps: &Maps) -> PEB64 {
        PEB64 {
            inheritet_addr_space: maps.read_byte(addr).unwrap(),
            read_img_file_exec_options: maps.read_byte(addr + 0x1).unwrap(),
            being_debugged: maps.read_byte(addr + 0x2).unwrap(),
            system_dependent_01: maps.read_byte(addr + 0x3).unwrap(),
            dummy_align: 0,
            mutant: maps.read_qword(addr + 0x8).unwrap(),
            image_base_addr: maps.read_qword(addr + 0x10).unwrap(),
            ldr: maps.read_qword(addr + 0x18).unwrap(),
            process_parameters: maps.read_qword(addr + 0x20).unwrap(),
            subsystem_data: maps.read_qword(addr + 0x28).unwrap(),
            process_heap: maps.read_qword(addr + 0x30).unwrap(),
            fast_peb_lock: maps.read_qword(addr + 0x38).unwrap(),
            system_dependent_02: maps.read_qword(addr + 0x40).unwrap(),
            system_dependent_03: maps.read_qword(addr + 0x48).unwrap(),
            system_dependent_04: maps.read_qword(addr + 0x50).unwrap(),
            kernel_callback_table: maps.read_qword(addr + 0x58).unwrap(),
            system_reserved: maps.read_dword(addr + 0x60).unwrap(),
            system_dependent_05: maps.read_dword(addr + 0x64).unwrap(),
            system_dependent_06: maps.read_qword(addr + 0x68).unwrap(),
            tls_expansion_counter: maps.read_qword(addr + 0x70).unwrap(),
            tls_bitmap: maps.read_qword(addr + 0x78).unwrap(),
            tls_bitmap_bits: [
                maps.read_dword(addr + 0x80).unwrap(),
                maps.read_dword(addr + 0x84).unwrap(),
            ],
            read_only_shared_memory_base: maps.read_qword(addr + 0x88).unwrap(),
            system_dependent_07: maps.read_qword(addr + 0x90).unwrap(),
            read_only_static_server_data: maps.read_qword(addr + 0x98).unwrap(),
            ansi_code_page_data: maps.read_qword(addr + 0xa0).unwrap(),
            oem_code_page_data: maps.read_qword(addr + 0xa8).unwrap(),
            unicode_case_table_data: maps.read_qword(addr + 0xb0).unwrap(),
            number_of_processors: maps.read_dword(addr + 0xb8).unwrap(),
            nt_global_flag: maps.read_dword(addr + 0xbc).unwrap(),
            critical_section_timeout: maps.read_qword(addr + 0xc0).unwrap(),
            heap_segment_reserve: maps.read_qword(addr + 0xc8).unwrap(),
            heap_segment_commit: maps.read_qword(addr + 0xd0).unwrap(),
            heap_decommit_total_free_threshold: maps.read_qword(addr + 0xd8).unwrap(),
            heap_decommit_free_block_threshold: maps.read_qword(addr + 0xd8).unwrap(),
            number_of_heaps: maps.read_dword(addr + 0xe8).unwrap(),
            max_number_of_heaps: maps.read_dword(addr + 0xec).unwrap(),
            process_heaps: maps.read_qword(addr + 0xf0).unwrap(),
            gdi_share_handle_table: maps.read_qword(addr + 0xf8).unwrap(),
            process_starter_helper: maps.read_qword(addr + 0x100).unwrap(),
            gdi_dc_attribute_list: maps.read_qword(addr + 0x108).unwrap(),
            loader_lock: maps.read_qword(addr + 0x110).unwrap(),
            os_major_version: maps.read_dword(addr + 0x118).unwrap(),
            os_minor_version: maps.read_dword(addr + 0x11c).unwrap(),
            os_build_number: maps.read_word(addr + 0x120).unwrap(),
            oscsd_version: maps.read_word(addr + 0x122).unwrap(),
            os_platform_id: maps.read_dword(addr + 0x124).unwrap(),
            image_subsystem: maps.read_dword(addr + 0x128).unwrap(),
            image_subsystem_major_version: maps.read_dword(addr + 0x12c).unwrap(),
            image_subsystem_minor_version: maps.read_qword(addr + 0x130).unwrap(),
            active_process_afinity_mask: maps.read_qword(addr + 0x138).unwrap(),
            gdi_handle_buffer: [0; 30],
            post_process_init_routine: maps.read_qword(addr + 0x230).unwrap(),
            tls_expansion_bitmap: maps.read_qword(addr + 0x238).unwrap(),
            tls_expansion_bitmap_bits: [0; 32],
            session_id: maps.read_qword(addr + 0x2c0).unwrap(),
            app_compat_flags: maps.read_qword(addr + 0x2c8).unwrap(),
            app_compat_flags_user: maps.read_qword(addr + 0x2d0).unwrap(),
            p_shim_data: maps.read_qword(addr + 0x2d8).unwrap(),
            app_compat_info: maps.read_qword(addr + 0x2e0).unwrap(),
            csd_version: [
                maps.read_qword(addr + 0x2e8).unwrap(),
                maps.read_qword(addr + 0x2f0).unwrap(),
            ],
            activate_context_data: maps.read_qword(addr + 0x2f8).unwrap(),
            process_assembly_storage_map: maps.read_qword(addr + 0x300).unwrap(),
            system_default_activation_context_data: maps.read_qword(addr + 0x308).unwrap(),
            system_assembly_storage_map: maps.read_qword(addr + 0x310).unwrap(),
            minimum_stack_commit: maps.read_qword(addr + 0x318).unwrap(),
        }
    }

    pub fn save(&self, mem: &mut Mem64) {
        let base = mem.get_base();
        mem.write_byte(base, self.inheritet_addr_space);
        mem.write_byte(base + 1, self.read_img_file_exec_options);
        mem.write_byte(base + 2, self.being_debugged);
        mem.write_byte(base + 3, self.system_dependent_01);
        mem.write_dword(base + 4, self.dummy_align);
        mem.write_qword(base + 8, self.mutant);
        mem.write_qword(base + 16, self.image_base_addr);
        mem.write_qword(base + 24, self.ldr);
        mem.write_qword(base + 32, self.process_parameters);
        mem.write_qword(base + 40, self.subsystem_data);
        mem.write_qword(base + 48, self.process_heap);
        mem.write_qword(base + 56, self.fast_peb_lock);
        mem.write_qword(base + 64, self.system_dependent_02);
        mem.write_qword(base + 72, self.system_dependent_03);
        mem.write_qword(base + 80, self.system_dependent_04);
        mem.write_qword(base + 88, self.kernel_callback_table);
        mem.write_dword(base + 96, self.system_reserved);
        mem.write_dword(base + 100, self.system_dependent_05);
        mem.write_qword(base + 104, self.system_dependent_06);
        mem.write_qword(base + 112, self.tls_expansion_counter);
        mem.write_qword(base + 120, self.tls_bitmap);
        mem.write_dword(base + 128, self.tls_bitmap_bits[0]);
        mem.write_dword(base + 132, self.tls_bitmap_bits[1]);
        mem.write_qword(base + 136, self.read_only_shared_memory_base);
        mem.write_qword(base + 144, self.system_dependent_07);
        mem.write_qword(base + 152, self.read_only_static_server_data);
        mem.write_qword(base + 160, self.ansi_code_page_data);
        mem.write_qword(base + 168, self.oem_code_page_data);
        mem.write_qword(base + 176, self.unicode_case_table_data);
        mem.write_dword(base + 184, self.number_of_processors);
        mem.write_dword(base + 188, self.nt_global_flag);
        mem.write_qword(base + 192, self.critical_section_timeout);
        mem.write_qword(base + 200, self.heap_segment_reserve);
        mem.write_qword(base + 208, self.heap_segment_commit);
        mem.write_qword(base + 216, self.heap_decommit_total_free_threshold);
        mem.write_qword(base + 224, self.heap_decommit_free_block_threshold);
        mem.write_dword(base + 232, self.number_of_heaps);
        mem.write_dword(base + 236, self.max_number_of_heaps);
        mem.write_qword(base + 240, self.process_heaps);
        mem.write_qword(base + 248, self.gdi_share_handle_table);
        mem.write_qword(base + 256, self.process_starter_helper);
        mem.write_qword(base + 264, self.gdi_dc_attribute_list);
        mem.write_qword(base + 272, self.loader_lock);
        mem.write_dword(base + 280, self.os_major_version);
        mem.write_dword(base + 284, self.os_minor_version);
        mem.write_word(base + 288, self.os_build_number);
        mem.write_word(base + 290, self.oscsd_version);
        mem.write_dword(base + 292, self.os_platform_id);
        mem.write_dword(base + 296, self.image_subsystem);
        mem.write_dword(base + 300, self.image_subsystem_major_version);
        mem.write_qword(base + 304, self.image_subsystem_minor_version);
        mem.write_qword(base + 312, self.active_process_afinity_mask);
        let mut idx = base + 312 + 8;
        for i in 0..30 {
            mem.write_qword(idx, self.gdi_handle_buffer[i as usize]);
            idx += 8;
        }
        mem.write_qword(idx, self.post_process_init_routine);
        mem.write_qword(idx + 8, self.tls_expansion_bitmap);
        idx += 8;
        for i in 0..32 {
            mem.write_dword(idx, self.tls_expansion_bitmap_bits[i]);
            idx += 4;
        }
        mem.write_qword(idx, self.session_id);
        mem.write_qword(idx + 8, self.app_compat_flags);
        mem.write_qword(idx + 16, self.app_compat_flags_user);
        mem.write_qword(idx + 24, self.p_shim_data);
        mem.write_qword(idx + 32, self.app_compat_info);
        mem.write_qword(idx + 40, self.csd_version[0]);
        mem.write_qword(idx + 48, self.csd_version[1]);
        mem.write_qword(idx + 56, self.activate_context_data);
        mem.write_qword(idx + 64, self.process_assembly_storage_map);
        mem.write_qword(idx + 72, self.system_default_activation_context_data);
        mem.write_qword(idx + 80, self.system_assembly_storage_map);
        mem.write_qword(idx + 88, self.minimum_stack_commit);
    }

    pub fn print(&self) {
        log::trace!("{:#x?}", self);
    }
}
