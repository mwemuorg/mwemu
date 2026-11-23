use std::array;

use crate::maps::{Maps, mem64::Mem64};
use crate::windows::constants::process::PEB_BEING_DEBUGGED_OFFSET;

/// Modeled fields of the 64-bit Windows process environment block.
#[derive(Debug)]
pub struct Peb64 {
    inherited_address_space: u8,
    read_image_file_exec_options: u8,
    pub being_debugged: u8,
    system_dependent_01: u8,
    padding: u32,
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
    tls_expansion_counter: u32,
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
    gdi_shared_handle_table: u64,
    process_starter_helper: u64,
    gdi_dc_attribute_list: u32,
    loader_lock: u64,
    os_major_version: u32,
    os_minor_version: u32,
    os_build_number: u16,
    os_csd_version: u16,
    os_platform_id: u32,
    image_subsystem: u32,
    image_subsystem_major_version: u32,
    image_subsystem_minor_version: u32,
    active_process_affinity_mask: u64,
    gdi_handle_buffer: [u32; 60],
    post_process_init_routine: u64,
    tls_expansion_bitmap: u64,
    tls_expansion_bitmap_bits: [u32; 32],
    session_id: u32,
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

impl Peb64 {
    pub(crate) const IMAGE_BASE_ADDR_OFFSET: u64 = 0x10;
    pub(crate) const LDR_OFFSET: u64 = 0x18;
    pub(crate) const PROCESS_HEAP_OFFSET: u64 = 0x30;
    pub(crate) const SYSTEM_DEPENDENT_06_OFFSET: u64 = 0x68;
    pub(crate) const CSR_STATIC_TABLE_OFFSET: u64 = 0xa90;
    const CSR_SERVER_BASE: u64 = 0x7efe0000;
    pub(crate) const READ_ONLY_SHARED_MEMORY_BASE_OFFSET: u64 = 0x88;
    pub(crate) const SYSTEM_DEPENDENT_07_OFFSET: u64 = 0x90;
    pub(crate) const READ_ONLY_STATIC_SERVER_DATA_OFFSET: u64 = 0x98;
    pub(crate) const ANSI_CODE_PAGE_DATA_OFFSET: u64 = 0xa0;
    pub(crate) const OEM_CODE_PAGE_DATA_OFFSET: u64 = 0xa8;
    pub(crate) const UNICODE_CASE_TABLE_DATA_OFFSET: u64 = 0xb0;

    const READ_IMAGE_FILE_EXEC_OPTIONS_OFFSET: u64 = 0x01;
    const SYSTEM_DEPENDENT_01_OFFSET: u64 = 0x03;
    const PADDING_OFFSET: u64 = 0x04;
    const MUTANT_OFFSET: u64 = 0x08;
    pub(crate) const PROCESS_PARAMETERS_OFFSET: u64 = 0x20;
    const SUBSYSTEM_DATA_OFFSET: u64 = 0x28;
    const FAST_PEB_LOCK_OFFSET: u64 = 0x38;
    const SYSTEM_DEPENDENT_02_OFFSET: u64 = 0x40;
    const SYSTEM_DEPENDENT_03_OFFSET: u64 = 0x48;
    const SYSTEM_DEPENDENT_04_OFFSET: u64 = 0x50;
    const KERNEL_CALLBACK_TABLE_OFFSET: u64 = 0x58;
    const SYSTEM_RESERVED_OFFSET: u64 = 0x60;
    const SYSTEM_DEPENDENT_05_OFFSET: u64 = 0x64;
    const TLS_EXPANSION_COUNTER_OFFSET: u64 = 0x70;
    const TLS_BITMAP_OFFSET: u64 = 0x78;
    const TLS_BITMAP_BITS_OFFSET: u64 = 0x80;
    const NUMBER_OF_PROCESSORS_OFFSET: u64 = 0xb8;
    const NT_GLOBAL_FLAG_OFFSET: u64 = 0xbc;
    const CRITICAL_SECTION_TIMEOUT_OFFSET: u64 = 0xc0;
    const HEAP_SEGMENT_RESERVE_OFFSET: u64 = 0xc8;
    const HEAP_SEGMENT_COMMIT_OFFSET: u64 = 0xd0;
    const HEAP_DECOMMIT_TOTAL_FREE_THRESHOLD_OFFSET: u64 = 0xd8;
    const HEAP_DECOMMIT_FREE_BLOCK_THRESHOLD_OFFSET: u64 = 0xe0;
    const NUMBER_OF_HEAPS_OFFSET: u64 = 0xe8;
    const MAX_NUMBER_OF_HEAPS_OFFSET: u64 = 0xec;
    const PROCESS_HEAPS_OFFSET: u64 = 0xf0;
    pub const GDI_SHARED_HANDLE_TABLE_OFFSET: u64 = 0xf8;
    const PROCESS_STARTER_HELPER_OFFSET: u64 = 0x100;
    const GDI_DC_ATTRIBUTE_LIST_OFFSET: u64 = 0x108;
    const LOADER_LOCK_OFFSET: u64 = 0x110;
    const OS_MAJOR_VERSION_OFFSET: u64 = 0x118;
    const OS_MINOR_VERSION_OFFSET: u64 = 0x11c;
    const OS_BUILD_NUMBER_OFFSET: u64 = 0x120;
    const OS_CSD_VERSION_OFFSET: u64 = 0x122;
    const OS_PLATFORM_ID_OFFSET: u64 = 0x124;
    const IMAGE_SUBSYSTEM_OFFSET: u64 = 0x128;
    const IMAGE_SUBSYSTEM_MAJOR_VERSION_OFFSET: u64 = 0x12c;
    const IMAGE_SUBSYSTEM_MINOR_VERSION_OFFSET: u64 = 0x130;
    const ACTIVE_PROCESS_AFFINITY_MASK_OFFSET: u64 = 0x138;
    const GDI_HANDLE_BUFFER_OFFSET: u64 = 0x140;
    const POST_PROCESS_INIT_ROUTINE_OFFSET: u64 = 0x230;
    const TLS_EXPANSION_BITMAP_OFFSET: u64 = 0x238;
    const TLS_EXPANSION_BITMAP_BITS_OFFSET: u64 = 0x240;
    pub(crate) const SESSION_ID_OFFSET: u64 = 0x2c0;
    const APP_COMPAT_FLAGS_OFFSET: u64 = 0x2c8;
    const APP_COMPAT_FLAGS_USER_OFFSET: u64 = 0x2d0;
    const P_SHIM_DATA_OFFSET: u64 = 0x2d8;
    const APP_COMPAT_INFO_OFFSET: u64 = 0x2e0;
    const CSD_VERSION_OFFSET: u64 = 0x2e8;
    const ACTIVATE_CONTEXT_DATA_OFFSET: u64 = 0x2f8;
    const PROCESS_ASSEMBLY_STORAGE_MAP_OFFSET: u64 = 0x300;
    const SYSTEM_DEFAULT_ACTIVATION_CONTEXT_DATA_OFFSET: u64 = 0x308;
    const SYSTEM_ASSEMBLY_STORAGE_MAP_OFFSET: u64 = 0x310;
    const MINIMUM_STACK_COMMIT_OFFSET: u64 = 0x318;

    pub fn size() -> usize {
        0x1000 // Windows 10 x64 PEB extends to ~0x7B8; use a full page
    }

    pub fn new(image_base_addr: u64, ldr: u64, process_parameters: u64) -> Self {
        Self {
            inherited_address_space: 0,
            read_image_file_exec_options: 0,
            being_debugged: 0,
            system_dependent_01: 0,
            padding: 0,
            mutant: u64::MAX,
            image_base_addr,
            ldr,
            process_parameters,
            subsystem_data: 0,
            process_heap: 0x520000,
            fast_peb_lock: 0x7710a900,
            system_dependent_02: 0,
            system_dependent_03: 0,
            system_dependent_04: 0x2,
            kernel_callback_table: 0x76f59500,
            system_reserved: 0,
            system_dependent_05: 0,
            system_dependent_06: 0x7feff2f0000,
            tls_expansion_counter: 0,
            tls_bitmap: 0x77102590,
            tls_bitmap_bits: [0x1fff, 0],
            read_only_shared_memory_base: Self::CSR_SERVER_BASE,
            system_dependent_07: 0,
            read_only_static_server_data: Self::CSR_SERVER_BASE + Self::CSR_STATIC_TABLE_OFFSET,
            ansi_code_page_data: 0x7fffffb0000,
            oem_code_page_data: 0x7fffffc0228,
            unicode_case_table_data: 0x7fffffd0650,
            number_of_processors: 0x1,
            nt_global_flag: 0,
            critical_section_timeout: 0xffffe86d079b8000,
            heap_segment_reserve: 0x100000,
            heap_segment_commit: 0x2000,
            heap_decommit_total_free_threshold: 0x10000,
            heap_decommit_free_block_threshold: 0x10000,
            number_of_heaps: 0x4,
            max_number_of_heaps: 0x10,
            process_heaps: 0x7710a6c0,
            gdi_shared_handle_table: 0x920000,
            process_starter_helper: 0,
            gdi_dc_attribute_list: 0x14,
            loader_lock: 0x77107490,
            os_major_version: 0x6,
            os_minor_version: 0x1,
            os_build_number: 0x1db1,
            os_csd_version: 0x100,
            os_platform_id: 0x2,
            image_subsystem: 0x3,
            image_subsystem_major_version: 0x5,
            image_subsystem_minor_version: 0x2,
            active_process_affinity_mask: 0x1,
            gdi_handle_buffer: [0; 60],
            post_process_init_routine: 0,
            tls_expansion_bitmap: 0x77102580,
            tls_expansion_bitmap_bits: [0; 32],
            session_id: 0x1,
            app_compat_flags: 0,
            app_compat_flags_user: 0,
            p_shim_data: 0,
            app_compat_info: 0,
            csd_version: [0x1e001c, 0x7efe0afa],
            activate_context_data: 0,
            process_assembly_storage_map: 0,
            system_default_activation_context_data: 0,
            system_assembly_storage_map: 0,
            minimum_stack_commit: 0,
        }
    }

    pub fn load(addr: u64, maps: &Maps) -> Self {
        Self {
            inherited_address_space: maps.read_byte(addr).unwrap_or(0),
            read_image_file_exec_options: maps
                .read_byte(addr + Self::READ_IMAGE_FILE_EXEC_OPTIONS_OFFSET)
                .unwrap_or(0),
            being_debugged: maps
                .read_byte(addr + PEB_BEING_DEBUGGED_OFFSET)
                .unwrap_or(0),
            system_dependent_01: maps
                .read_byte(addr + Self::SYSTEM_DEPENDENT_01_OFFSET)
                .unwrap_or(0),
            padding: maps.read_dword(addr + Self::PADDING_OFFSET).unwrap_or(0),
            mutant: maps.read_qword(addr + Self::MUTANT_OFFSET).unwrap_or(0),
            image_base_addr: maps
                .read_qword(addr + Self::IMAGE_BASE_ADDR_OFFSET)
                .unwrap_or(0),
            ldr: maps.read_qword(addr + Self::LDR_OFFSET).unwrap_or(0),
            process_parameters: maps
                .read_qword(addr + Self::PROCESS_PARAMETERS_OFFSET)
                .unwrap_or(0),
            subsystem_data: maps
                .read_qword(addr + Self::SUBSYSTEM_DATA_OFFSET)
                .unwrap_or(0),
            process_heap: maps
                .read_qword(addr + Self::PROCESS_HEAP_OFFSET)
                .unwrap_or(0),
            fast_peb_lock: maps
                .read_qword(addr + Self::FAST_PEB_LOCK_OFFSET)
                .unwrap_or(0),
            system_dependent_02: maps
                .read_qword(addr + Self::SYSTEM_DEPENDENT_02_OFFSET)
                .unwrap_or(0),
            system_dependent_03: maps
                .read_qword(addr + Self::SYSTEM_DEPENDENT_03_OFFSET)
                .unwrap_or(0),
            system_dependent_04: maps
                .read_qword(addr + Self::SYSTEM_DEPENDENT_04_OFFSET)
                .unwrap_or(0),
            kernel_callback_table: maps
                .read_qword(addr + Self::KERNEL_CALLBACK_TABLE_OFFSET)
                .unwrap_or(0),
            system_reserved: maps
                .read_dword(addr + Self::SYSTEM_RESERVED_OFFSET)
                .unwrap_or(0),
            system_dependent_05: maps
                .read_dword(addr + Self::SYSTEM_DEPENDENT_05_OFFSET)
                .unwrap_or(0),
            system_dependent_06: maps
                .read_qword(addr + Self::SYSTEM_DEPENDENT_06_OFFSET)
                .unwrap_or(0),
            tls_expansion_counter: maps
                .read_dword(addr + Self::TLS_EXPANSION_COUNTER_OFFSET)
                .unwrap_or(0),
            tls_bitmap: maps.read_qword(addr + Self::TLS_BITMAP_OFFSET).unwrap_or(0),
            tls_bitmap_bits: array::from_fn(|i| {
                maps.read_dword(addr + Self::TLS_BITMAP_BITS_OFFSET + (i * size_of::<u32>()) as u64)
                    .unwrap_or(0)
            }),
            read_only_shared_memory_base: maps
                .read_qword(addr + Self::READ_ONLY_SHARED_MEMORY_BASE_OFFSET)
                .unwrap_or(0),
            system_dependent_07: maps
                .read_qword(addr + Self::SYSTEM_DEPENDENT_07_OFFSET)
                .unwrap_or(0),
            read_only_static_server_data: maps
                .read_qword(addr + Self::READ_ONLY_STATIC_SERVER_DATA_OFFSET)
                .unwrap_or(0),
            ansi_code_page_data: maps
                .read_qword(addr + Self::ANSI_CODE_PAGE_DATA_OFFSET)
                .unwrap_or(0),
            oem_code_page_data: maps
                .read_qword(addr + Self::OEM_CODE_PAGE_DATA_OFFSET)
                .unwrap_or(0),
            unicode_case_table_data: maps
                .read_qword(addr + Self::UNICODE_CASE_TABLE_DATA_OFFSET)
                .unwrap_or(0),
            number_of_processors: maps
                .read_dword(addr + Self::NUMBER_OF_PROCESSORS_OFFSET)
                .unwrap_or(0),
            nt_global_flag: maps
                .read_dword(addr + Self::NT_GLOBAL_FLAG_OFFSET)
                .unwrap_or(0),
            critical_section_timeout: maps
                .read_qword(addr + Self::CRITICAL_SECTION_TIMEOUT_OFFSET)
                .unwrap_or(0),
            heap_segment_reserve: maps
                .read_qword(addr + Self::HEAP_SEGMENT_RESERVE_OFFSET)
                .unwrap_or(0),
            heap_segment_commit: maps
                .read_qword(addr + Self::HEAP_SEGMENT_COMMIT_OFFSET)
                .unwrap_or(0),
            heap_decommit_total_free_threshold: maps
                .read_qword(addr + Self::HEAP_DECOMMIT_TOTAL_FREE_THRESHOLD_OFFSET)
                .unwrap_or(0),
            heap_decommit_free_block_threshold: maps
                .read_qword(addr + Self::HEAP_DECOMMIT_FREE_BLOCK_THRESHOLD_OFFSET)
                .unwrap_or(0),
            number_of_heaps: maps
                .read_dword(addr + Self::NUMBER_OF_HEAPS_OFFSET)
                .unwrap_or(0),
            max_number_of_heaps: maps
                .read_dword(addr + Self::MAX_NUMBER_OF_HEAPS_OFFSET)
                .unwrap_or(0),
            process_heaps: maps
                .read_qword(addr + Self::PROCESS_HEAPS_OFFSET)
                .unwrap_or(0),
            gdi_shared_handle_table: maps
                .read_qword(addr + Self::GDI_SHARED_HANDLE_TABLE_OFFSET)
                .unwrap_or(0),
            process_starter_helper: maps
                .read_qword(addr + Self::PROCESS_STARTER_HELPER_OFFSET)
                .unwrap_or(0),
            gdi_dc_attribute_list: maps
                .read_dword(addr + Self::GDI_DC_ATTRIBUTE_LIST_OFFSET)
                .unwrap_or(0),
            loader_lock: maps
                .read_qword(addr + Self::LOADER_LOCK_OFFSET)
                .unwrap_or(0),
            os_major_version: maps
                .read_dword(addr + Self::OS_MAJOR_VERSION_OFFSET)
                .unwrap_or(0),
            os_minor_version: maps
                .read_dword(addr + Self::OS_MINOR_VERSION_OFFSET)
                .unwrap_or(0),
            os_build_number: maps
                .read_word(addr + Self::OS_BUILD_NUMBER_OFFSET)
                .unwrap_or(0),
            os_csd_version: maps
                .read_word(addr + Self::OS_CSD_VERSION_OFFSET)
                .unwrap_or(0),
            os_platform_id: maps
                .read_dword(addr + Self::OS_PLATFORM_ID_OFFSET)
                .unwrap_or(0),
            image_subsystem: maps
                .read_dword(addr + Self::IMAGE_SUBSYSTEM_OFFSET)
                .unwrap_or(0),
            image_subsystem_major_version: maps
                .read_dword(addr + Self::IMAGE_SUBSYSTEM_MAJOR_VERSION_OFFSET)
                .unwrap_or(0),
            image_subsystem_minor_version: maps
                .read_dword(addr + Self::IMAGE_SUBSYSTEM_MINOR_VERSION_OFFSET)
                .unwrap_or(0),
            active_process_affinity_mask: maps
                .read_qword(addr + Self::ACTIVE_PROCESS_AFFINITY_MASK_OFFSET)
                .unwrap_or(0),
            gdi_handle_buffer: array::from_fn(|i| {
                maps.read_dword(
                    addr + Self::GDI_HANDLE_BUFFER_OFFSET + (i * size_of::<u32>()) as u64,
                )
                .unwrap_or(0)
            }),
            post_process_init_routine: maps
                .read_qword(addr + Self::POST_PROCESS_INIT_ROUTINE_OFFSET)
                .unwrap_or(0),
            tls_expansion_bitmap: maps
                .read_qword(addr + Self::TLS_EXPANSION_BITMAP_OFFSET)
                .unwrap_or(0),
            tls_expansion_bitmap_bits: array::from_fn(|i| {
                maps.read_dword(
                    addr + Self::TLS_EXPANSION_BITMAP_BITS_OFFSET + (i * size_of::<u32>()) as u64,
                )
                .unwrap_or(0)
            }),
            session_id: maps.read_dword(addr + Self::SESSION_ID_OFFSET).unwrap_or(0),
            app_compat_flags: maps
                .read_qword(addr + Self::APP_COMPAT_FLAGS_OFFSET)
                .unwrap_or(0),
            app_compat_flags_user: maps
                .read_qword(addr + Self::APP_COMPAT_FLAGS_USER_OFFSET)
                .unwrap_or(0),
            p_shim_data: maps
                .read_qword(addr + Self::P_SHIM_DATA_OFFSET)
                .unwrap_or(0),
            app_compat_info: maps
                .read_qword(addr + Self::APP_COMPAT_INFO_OFFSET)
                .unwrap_or(0),
            csd_version: array::from_fn(|i| {
                maps.read_qword(addr + Self::CSD_VERSION_OFFSET + (i * size_of::<u64>()) as u64)
                    .unwrap_or(0)
            }),
            activate_context_data: maps
                .read_qword(addr + Self::ACTIVATE_CONTEXT_DATA_OFFSET)
                .unwrap_or(0),
            process_assembly_storage_map: maps
                .read_qword(addr + Self::PROCESS_ASSEMBLY_STORAGE_MAP_OFFSET)
                .unwrap_or(0),
            system_default_activation_context_data: maps
                .read_qword(addr + Self::SYSTEM_DEFAULT_ACTIVATION_CONTEXT_DATA_OFFSET)
                .unwrap_or(0),
            system_assembly_storage_map: maps
                .read_qword(addr + Self::SYSTEM_ASSEMBLY_STORAGE_MAP_OFFSET)
                .unwrap_or(0),
            minimum_stack_commit: maps
                .read_qword(addr + Self::MINIMUM_STACK_COMMIT_OFFSET)
                .unwrap_or(0),
        }
    }

    pub fn save(&self, mem: &mut Mem64) {
        let base = mem.get_base();
        mem.write_byte(base, self.inherited_address_space);
        mem.write_byte(
            base + Self::READ_IMAGE_FILE_EXEC_OPTIONS_OFFSET,
            self.read_image_file_exec_options,
        );
        mem.write_byte(base + PEB_BEING_DEBUGGED_OFFSET, self.being_debugged);
        mem.write_byte(
            base + Self::SYSTEM_DEPENDENT_01_OFFSET,
            self.system_dependent_01,
        );
        mem.write_dword(base + Self::PADDING_OFFSET, self.padding);
        mem.write_qword(base + Self::MUTANT_OFFSET, self.mutant);
        mem.write_qword(base + Self::IMAGE_BASE_ADDR_OFFSET, self.image_base_addr);
        mem.write_qword(base + Self::LDR_OFFSET, self.ldr);
        mem.write_qword(
            base + Self::PROCESS_PARAMETERS_OFFSET,
            self.process_parameters,
        );
        mem.write_qword(base + Self::SUBSYSTEM_DATA_OFFSET, self.subsystem_data);
        mem.write_qword(base + Self::PROCESS_HEAP_OFFSET, self.process_heap);
        mem.write_qword(base + Self::FAST_PEB_LOCK_OFFSET, self.fast_peb_lock);
        mem.write_qword(
            base + Self::SYSTEM_DEPENDENT_02_OFFSET,
            self.system_dependent_02,
        );
        mem.write_qword(
            base + Self::SYSTEM_DEPENDENT_03_OFFSET,
            self.system_dependent_03,
        );
        mem.write_qword(
            base + Self::SYSTEM_DEPENDENT_04_OFFSET,
            self.system_dependent_04,
        );
        mem.write_qword(
            base + Self::KERNEL_CALLBACK_TABLE_OFFSET,
            self.kernel_callback_table,
        );
        mem.write_dword(base + Self::SYSTEM_RESERVED_OFFSET, self.system_reserved);
        mem.write_dword(
            base + Self::SYSTEM_DEPENDENT_05_OFFSET,
            self.system_dependent_05,
        );
        mem.write_qword(
            base + Self::SYSTEM_DEPENDENT_06_OFFSET,
            self.system_dependent_06,
        );
        mem.write_dword(
            base + Self::TLS_EXPANSION_COUNTER_OFFSET,
            self.tls_expansion_counter,
        );
        mem.write_qword(base + Self::TLS_BITMAP_OFFSET, self.tls_bitmap);
        for (i, value) in self.tls_bitmap_bits.iter().enumerate() {
            mem.write_dword(
                base + Self::TLS_BITMAP_BITS_OFFSET + (i * size_of::<u32>()) as u64,
                *value,
            );
        }
        mem.write_qword(
            base + Self::READ_ONLY_SHARED_MEMORY_BASE_OFFSET,
            self.read_only_shared_memory_base,
        );
        mem.write_qword(
            base + Self::SYSTEM_DEPENDENT_07_OFFSET,
            self.system_dependent_07,
        );
        mem.write_qword(
            base + Self::READ_ONLY_STATIC_SERVER_DATA_OFFSET,
            self.read_only_static_server_data,
        );
        mem.write_qword(
            base + Self::ANSI_CODE_PAGE_DATA_OFFSET,
            self.ansi_code_page_data,
        );
        mem.write_qword(
            base + Self::OEM_CODE_PAGE_DATA_OFFSET,
            self.oem_code_page_data,
        );
        mem.write_qword(
            base + Self::UNICODE_CASE_TABLE_DATA_OFFSET,
            self.unicode_case_table_data,
        );
        mem.write_dword(
            base + Self::NUMBER_OF_PROCESSORS_OFFSET,
            self.number_of_processors,
        );
        mem.write_dword(base + Self::NT_GLOBAL_FLAG_OFFSET, self.nt_global_flag);
        mem.write_qword(
            base + Self::CRITICAL_SECTION_TIMEOUT_OFFSET,
            self.critical_section_timeout,
        );
        mem.write_qword(
            base + Self::HEAP_SEGMENT_RESERVE_OFFSET,
            self.heap_segment_reserve,
        );
        mem.write_qword(
            base + Self::HEAP_SEGMENT_COMMIT_OFFSET,
            self.heap_segment_commit,
        );
        mem.write_qword(
            base + Self::HEAP_DECOMMIT_TOTAL_FREE_THRESHOLD_OFFSET,
            self.heap_decommit_total_free_threshold,
        );
        mem.write_qword(
            base + Self::HEAP_DECOMMIT_FREE_BLOCK_THRESHOLD_OFFSET,
            self.heap_decommit_free_block_threshold,
        );
        mem.write_dword(base + Self::NUMBER_OF_HEAPS_OFFSET, self.number_of_heaps);
        mem.write_dword(
            base + Self::MAX_NUMBER_OF_HEAPS_OFFSET,
            self.max_number_of_heaps,
        );
        mem.write_qword(base + Self::PROCESS_HEAPS_OFFSET, self.process_heaps);
        mem.write_qword(
            base + Self::GDI_SHARED_HANDLE_TABLE_OFFSET,
            self.gdi_shared_handle_table,
        );
        mem.write_qword(
            base + Self::PROCESS_STARTER_HELPER_OFFSET,
            self.process_starter_helper,
        );
        mem.write_dword(
            base + Self::GDI_DC_ATTRIBUTE_LIST_OFFSET,
            self.gdi_dc_attribute_list,
        );
        mem.write_qword(base + Self::LOADER_LOCK_OFFSET, self.loader_lock);
        mem.write_dword(base + Self::OS_MAJOR_VERSION_OFFSET, self.os_major_version);
        mem.write_dword(base + Self::OS_MINOR_VERSION_OFFSET, self.os_minor_version);
        mem.write_word(base + Self::OS_BUILD_NUMBER_OFFSET, self.os_build_number);
        mem.write_word(base + Self::OS_CSD_VERSION_OFFSET, self.os_csd_version);
        mem.write_dword(base + Self::OS_PLATFORM_ID_OFFSET, self.os_platform_id);
        mem.write_dword(base + Self::IMAGE_SUBSYSTEM_OFFSET, self.image_subsystem);
        mem.write_dword(
            base + Self::IMAGE_SUBSYSTEM_MAJOR_VERSION_OFFSET,
            self.image_subsystem_major_version,
        );
        mem.write_dword(
            base + Self::IMAGE_SUBSYSTEM_MINOR_VERSION_OFFSET,
            self.image_subsystem_minor_version,
        );
        mem.write_qword(
            base + Self::ACTIVE_PROCESS_AFFINITY_MASK_OFFSET,
            self.active_process_affinity_mask,
        );
        for (i, value) in self.gdi_handle_buffer.iter().enumerate() {
            mem.write_dword(
                base + Self::GDI_HANDLE_BUFFER_OFFSET + (i * size_of::<u32>()) as u64,
                *value,
            );
        }
        mem.write_qword(
            base + Self::POST_PROCESS_INIT_ROUTINE_OFFSET,
            self.post_process_init_routine,
        );
        mem.write_qword(
            base + Self::TLS_EXPANSION_BITMAP_OFFSET,
            self.tls_expansion_bitmap,
        );
        for (i, value) in self.tls_expansion_bitmap_bits.iter().enumerate() {
            mem.write_dword(
                base + Self::TLS_EXPANSION_BITMAP_BITS_OFFSET + (i * size_of::<u32>()) as u64,
                *value,
            );
        }
        mem.write_dword(base + Self::SESSION_ID_OFFSET, self.session_id);
        mem.write_qword(base + Self::APP_COMPAT_FLAGS_OFFSET, self.app_compat_flags);
        mem.write_qword(
            base + Self::APP_COMPAT_FLAGS_USER_OFFSET,
            self.app_compat_flags_user,
        );
        mem.write_qword(base + Self::P_SHIM_DATA_OFFSET, self.p_shim_data);
        mem.write_qword(base + Self::APP_COMPAT_INFO_OFFSET, self.app_compat_info);
        for (i, value) in self.csd_version.iter().enumerate() {
            mem.write_qword(
                base + Self::CSD_VERSION_OFFSET + (i * size_of::<u64>()) as u64,
                *value,
            );
        }
        mem.write_qword(
            base + Self::ACTIVATE_CONTEXT_DATA_OFFSET,
            self.activate_context_data,
        );
        mem.write_qword(
            base + Self::PROCESS_ASSEMBLY_STORAGE_MAP_OFFSET,
            self.process_assembly_storage_map,
        );
        mem.write_qword(
            base + Self::SYSTEM_DEFAULT_ACTIVATION_CONTEXT_DATA_OFFSET,
            self.system_default_activation_context_data,
        );
        mem.write_qword(
            base + Self::SYSTEM_ASSEMBLY_STORAGE_MAP_OFFSET,
            self.system_assembly_storage_map,
        );
        mem.write_qword(
            base + Self::MINIMUM_STACK_COMMIT_OFFSET,
            self.minimum_stack_commit,
        );
    }

    pub fn print(&self) {
        log::trace!("{:#x?}", self);
    }
}
