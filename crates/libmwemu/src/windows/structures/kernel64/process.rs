use std::convert::TryInto;

use crate::maps::Maps;

use super::ListEntry64;
use super::base_types::{ClientId, UnicodeString};

#[derive(Debug)]
pub struct NtTib {
    pub exception_list: u64,
    pub stack_base: u64,
    pub stack_limit: u64,
    pub reserved1: u64,
    pub reserved2: u64,
    pub reserved3: u64,
    pub self_ptr: u64,
}

impl Default for NtTib {
    fn default() -> Self {
        Self::new()
    }
}

impl NtTib {
    pub fn new() -> NtTib {
        NtTib {
            exception_list: 0,
            stack_base: 0,
            stack_limit: 0,
            reserved1: 0,
            reserved2: 0,
            reserved3: 0,
            self_ptr: 0,
        }
    }

    pub fn load(addr: u64, maps: &Maps) -> NtTib {
        NtTib {
            exception_list: maps.read_qword(addr).unwrap(),
            stack_base: maps.read_qword(addr + 8).unwrap(),
            stack_limit: maps.read_qword(addr + 16).unwrap(),
            reserved1: maps.read_qword(addr + 24).unwrap(),
            reserved2: maps.read_qword(addr + 32).unwrap(),
            reserved3: maps.read_qword(addr + 40).unwrap(),
            self_ptr: maps.read_qword(addr + 48).unwrap(),
        }
    }

    pub fn save(&self, addr: u64, maps: &mut Maps) {
        maps.write_qword(addr, self.exception_list);
        maps.write_qword(addr + 8, self.stack_base);
        maps.write_qword(addr + 16, self.stack_limit);
        maps.write_qword(addr + 24, self.reserved1);
        maps.write_qword(addr + 32, self.reserved2);
        maps.write_qword(addr + 40, self.reserved3);
        maps.write_qword(addr + 48, self.self_ptr);
    }
}

#[derive(Debug)]
pub struct Teb {
    pub nt_tib: NtTib,
    pub environment_pointer: u64,
    pub client_id: ClientId,
    pub pad0: u64,
    pub active_rpc_handle: u64,
    pub thread_local_storage_pointer: u64,
    pub process_environment_block: u64,
    pub last_error_value: u32,
    pub count_of_owned_critical_sections: u32,
    pub csr_client_thread: u64,
    pub win32_thread_info: u64,
    pub user32_reserved: [u32; 26],
    pub user_reserved: [u32; 5],
    pub wow32_reserved: u64,
    pub current_locale: u32,
}

impl Default for Teb {
    fn default() -> Self {
        Self::new()
    }
}

impl Teb {
    pub fn new() -> Teb {
        Teb {
            nt_tib: NtTib::new(),
            environment_pointer: 0,
            client_id: ClientId::new(),
            pad0: 0,
            active_rpc_handle: 0,
            thread_local_storage_pointer: 0,
            process_environment_block: 0,
            last_error_value: 0,
            count_of_owned_critical_sections: 0,
            csr_client_thread: 0,
            win32_thread_info: 0,
            user32_reserved: [0; 26],
            user_reserved: [0; 5],
            wow32_reserved: 0,
            current_locale: 0,
        }
    }

    pub fn load(addr: u64, maps: &Maps) -> Teb {
        Teb {
            nt_tib: NtTib::load(addr, maps),
            environment_pointer: maps.read_qword(addr + 56).unwrap(),
            client_id: ClientId::load(addr + 64, maps),
            pad0: maps.read_qword(addr + 80).unwrap(),
            active_rpc_handle: maps.read_qword(addr + 88).unwrap(),
            thread_local_storage_pointer: maps.read_qword(addr + 96).unwrap(),
            process_environment_block: maps.read_qword(addr + 104).unwrap(),
            last_error_value: maps.read_dword(addr + 112).unwrap(),
            count_of_owned_critical_sections: maps.read_dword(addr + 116).unwrap(),
            csr_client_thread: maps.read_qword(addr + 120).unwrap(),
            win32_thread_info: maps.read_qword(addr + 128).unwrap(),
            user32_reserved: [0; 26],
            user_reserved: [0; 5],
            wow32_reserved: maps.read_qword(addr + 260).unwrap(),
            current_locale: maps.read_dword(addr + 268).unwrap(),
        }
    }

    pub fn save(&self, addr: u64, maps: &mut Maps) {
        self.nt_tib.save(addr, maps);
        maps.write_qword(addr + 56, self.environment_pointer);
        self.client_id.save(addr + 64, maps);
        maps.write_qword(addr + 80, self.pad0);
        maps.write_qword(addr + 88, self.active_rpc_handle);
        maps.write_qword(addr + 96, self.thread_local_storage_pointer);
        maps.write_qword(addr + 104, self.process_environment_block);
        maps.write_dword(addr + 112, self.last_error_value);
        maps.write_dword(addr + 116, self.count_of_owned_critical_sections);
        maps.write_qword(addr + 120, self.csr_client_thread);
        maps.write_qword(addr + 128, self.win32_thread_info);
        let user32_bytes: Vec<u8> = self
            .user32_reserved
            .iter()
            .copied()
            .flat_map(|v| v.to_le_bytes())
            .collect();
        maps.write_bytes(addr + 136, &user32_bytes);
        let user_bytes: Vec<u8> = self
            .user_reserved
            .iter()
            .copied()
            .flat_map(|v| v.to_le_bytes())
            .collect();
        maps.write_bytes(addr + 240, &user_bytes);
        maps.write_qword(addr + 260, self.wow32_reserved);
        maps.write_dword(addr + 268, self.current_locale);
    }
}

#[derive(Debug)]
pub struct Peb {
    pub inherited_address_space: u8,
    pub read_image_file_exec_options: u8,
    pub being_debugged: u8,
    pub bit_field: u8,
    pub mutant: u64,
    pub image_base_address: u64,
    pub ldr: u64,
    pub process_parameters: u64,
    pub sub_system_data: u64,
    pub process_heap: u64,
    pub fast_peb_lock: u64,
    pub atl_thunk_slist_ptr: u64,
    pub ifeo_key: u64,
    pub cross_process_flags: u64,
    pub user_shared_info_ptr: u64,
    pub system_reserved: u32,
    pub atl_thunk_slist_ptr32: u32,
    pub api_set_map: u64,
    pub tls_expansion_counter: u64,
    pub tls_bitmap: u64,
    pub tls_bitmap_bits: [u32; 2],
    pub read_only_shared_memory_base: u64,
    pub shared_data: u64,
    pub read_only_static_server_data: u64,
    pub ansi_code_page_data: u64,
    pub oem_code_page_data: u64,
    pub unicode_case_table_data: u64,
    pub number_of_processors: u32,
    pub nt_global_flag: u32,
    pub critical_section_timeout: i64,
    pub heap_segment_reserve: u64,
    pub heap_segment_commit: u64,
    pub heap_decommit_total_free_threshold: u64,
    pub heap_decommit_free_block_threshold: u64,
    pub number_of_heaps: u32,
    pub maximum_number_of_heaps: u32,
    pub process_heaps: u64,
    pub gdi_shared_handle_table: u64,
    pub process_starter_helper: u64,
    pub gdi_dc_attribute_list: u64,
    pub loader_lock: u64,
    pub os_major_version: u32,
    pub os_minor_version: u32,
    pub os_build_number: u16,
    pub os_csd_version: u16,
    pub os_platform_id: u32,
    pub image_subsystem: u32,
    pub image_subsystem_major_version: u32,
    pub image_subsystem_minor_version: u64,
    pub gdi_handle_buffer: [u32; 60],
    pub post_process_init_routine: u64,
    pub tls_expansion_bitmap: u64,
    pub tls_expansion_bitmap_bits: [u32; 32],
    pub session_id: u64,
    pub app_compat_flags: u64,
    pub app_compat_flags_user: u64,
    pub p_shim_data: u64,
    pub app_compat_info: u64,
    pub csd_version: UnicodeString,
    pub activation_context_data: u64,
    pub process_assembly_storage_map: u64,
    pub system_default_activation_context_data: u64,
    pub system_assembly_storage_map: u64,
    pub minimum_stack_commit: u64,
    pub fls_callback: u64,
    pub fls_list_head: ListEntry64,
    pub fls_bitmap: u64,
    pub fls_bitmap_bits: [u32; 4],
    pub fls_high_index: u64,
    pub wer_registration_data: u64,
    pub wer_ship_assert_ptr: u64,
    pub p_unused: u64,
    pub p_image_header_hash: u64,
    pub tracing_flags: u64,
    pub csr_server_read_only_shared_memory_base: u64,
    pub tpp_workerp_list_lock: u64,
    pub tpp_workerp_list: ListEntry64,
    pub wait_on_address_hash_table: [u64; 128],
}

impl Default for Peb {
    fn default() -> Self {
        Self::new()
    }
}

impl Peb {
    pub fn new() -> Peb {
        Peb {
            inherited_address_space: 0,
            read_image_file_exec_options: 0,
            being_debugged: 0,
            bit_field: 0,
            mutant: 0,
            image_base_address: 0,
            ldr: 0,
            process_parameters: 0,
            sub_system_data: 0,
            process_heap: 0,
            fast_peb_lock: 0,
            atl_thunk_slist_ptr: 0,
            ifeo_key: 0,
            cross_process_flags: 0,
            user_shared_info_ptr: 0,
            system_reserved: 0,
            atl_thunk_slist_ptr32: 0,
            api_set_map: 0,
            tls_expansion_counter: 0,
            tls_bitmap: 0,
            tls_bitmap_bits: [0; 2],
            read_only_shared_memory_base: 0,
            shared_data: 0,
            read_only_static_server_data: 0,
            ansi_code_page_data: 0,
            oem_code_page_data: 0,
            unicode_case_table_data: 0,
            number_of_processors: 0,
            nt_global_flag: 0,
            critical_section_timeout: 0,
            heap_segment_reserve: 0,
            heap_segment_commit: 0,
            heap_decommit_total_free_threshold: 0,
            heap_decommit_free_block_threshold: 0,
            number_of_heaps: 0,
            maximum_number_of_heaps: 0,
            process_heaps: 0,
            gdi_shared_handle_table: 0,
            process_starter_helper: 0,
            gdi_dc_attribute_list: 0,
            loader_lock: 0,
            os_major_version: 0,
            os_minor_version: 0,
            os_build_number: 0,
            os_csd_version: 0,
            os_platform_id: 0,
            image_subsystem: 0,
            image_subsystem_major_version: 0,
            image_subsystem_minor_version: 0,
            gdi_handle_buffer: [0; 60],
            post_process_init_routine: 0,
            tls_expansion_bitmap: 0,
            tls_expansion_bitmap_bits: [0; 32],
            session_id: 0,
            app_compat_flags: 0,
            app_compat_flags_user: 0,
            p_shim_data: 0,
            app_compat_info: 0,
            csd_version: UnicodeString::new(),
            activation_context_data: 0,
            process_assembly_storage_map: 0,
            system_default_activation_context_data: 0,
            system_assembly_storage_map: 0,
            minimum_stack_commit: 0,
            fls_callback: 0,
            fls_list_head: ListEntry64::new(),
            fls_bitmap: 0,
            fls_bitmap_bits: [0; 4],
            fls_high_index: 0,
            wer_registration_data: 0,
            wer_ship_assert_ptr: 0,
            p_unused: 0,
            p_image_header_hash: 0,
            tracing_flags: 0,
            csr_server_read_only_shared_memory_base: 0,
            tpp_workerp_list_lock: 0,
            tpp_workerp_list: ListEntry64::new(),
            wait_on_address_hash_table: [0; 128],
        }
    }
}

#[derive(Debug)]
pub struct RtlUserProcessParameters {
    pub reserved1: [u8; 16],
    pub reserved2: [u32; 10],
    pub image_path_name: UnicodeString,
    pub command_line: UnicodeString,
}

impl Default for RtlUserProcessParameters {
    fn default() -> Self {
        Self::new()
    }
}

impl RtlUserProcessParameters {
    pub fn new() -> RtlUserProcessParameters {
        RtlUserProcessParameters {
            reserved1: [0; 16],
            reserved2: [0; 10],
            image_path_name: UnicodeString::new(),
            command_line: UnicodeString::new(),
        }
    }

    pub fn load(addr: u64, maps: &Maps) -> RtlUserProcessParameters {
        let mut reserved2 = [0u32; 10];
        if let Some(b) = maps.try_read_bytes(addr + 16, 40) {
            for (i, c) in b.chunks_exact(4).enumerate().take(10) {
                reserved2[i] = u32::from_le_bytes(c.try_into().unwrap());
            }
        }
        RtlUserProcessParameters {
            reserved1: maps.read_bytes_array::<16>(addr),
            reserved2,
            image_path_name: UnicodeString::load(addr + 56, maps),
            command_line: UnicodeString::load(addr + 64, maps),
        }
    }

    pub fn save(&self, addr: u64, maps: &mut Maps) {
        maps.write_bytes(addr, &self.reserved1);
        let reserved2_bytes: Vec<u8> = self
            .reserved2
            .iter()
            .flat_map(|v| v.to_le_bytes())
            .collect();
        maps.write_bytes(addr + 16, &reserved2_bytes);
        self.image_path_name.save(addr + 56, maps);
        self.command_line.save(addr + 64, maps);
    }
}
