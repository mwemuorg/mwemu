use crate::maps::Maps;

use super::base_types::{ClientId, UnicodeString};

#[derive(Debug)]
pub struct SystemModule {
    pub reserved: [u64; 2],
    pub base: u64,
    pub size: u32,
    pub flags: u32,
    pub index: u16,
    pub unknown: u16,
    pub load_count: u16,
    pub module_name_offset: u16,
    pub image_name: [u8; 256],
}

impl Default for SystemModule {
    fn default() -> Self {
        Self::new()
    }
}

impl SystemModule {
    pub fn size() -> u32 {
        288
    }

    pub fn new() -> SystemModule {
        SystemModule {
            reserved: [0; 2],
            base: 0,
            size: 0,
            flags: 0,
            index: 0,
            unknown: 0,
            load_count: 0,
            module_name_offset: 0,
            image_name: [0; 256],
        }
    }

    pub fn load(addr: u64, maps: &Maps) -> SystemModule {
        let mut reserved = [0u64; 2];
        reserved[0] = maps.read_qword(addr).unwrap();
        reserved[1] = maps.read_qword(addr + 8).unwrap();
        SystemModule {
            reserved,
            base: maps.read_qword(addr + 16).unwrap(),
            size: maps.read_dword(addr + 24).unwrap(),
            flags: maps.read_dword(addr + 28).unwrap(),
            index: maps.read_word(addr + 32).unwrap(),
            unknown: maps.read_word(addr + 34).unwrap(),
            load_count: maps.read_word(addr + 36).unwrap(),
            module_name_offset: maps.read_word(addr + 38).unwrap(),
            image_name: maps.read_bytes_array::<256>(addr + 40),
        }
    }

    pub fn save(&self, addr: u64, maps: &mut Maps) {
        maps.write_qword(addr, self.reserved[0]);
        maps.write_qword(addr + 8, self.reserved[1]);
        maps.write_qword(addr + 16, self.base);
        maps.write_dword(addr + 24, self.size);
        maps.write_dword(addr + 28, self.flags);
        maps.write_word(addr + 32, self.index);
        maps.write_word(addr + 34, self.unknown);
        maps.write_word(addr + 36, self.load_count);
        maps.write_word(addr + 38, self.module_name_offset);
        maps.write_bytes(addr + 40, &self.image_name);
    }
}

#[derive(Debug)]
pub struct SystemTimeOfDayInformation {
    pub boot_time: u64,
    pub current_time: u64,
    pub time_zone_bias: u64,
    pub time_zone_id: u32,
    pub reserved: u32,
    pub boot_time_bias: u64,
    pub sleep_time_bias: u64,
}

impl Default for SystemTimeOfDayInformation {
    fn default() -> Self {
        Self::new()
    }
}

impl SystemTimeOfDayInformation {
    pub fn size() -> u32 {
        56
    }

    pub fn new() -> SystemTimeOfDayInformation {
        SystemTimeOfDayInformation {
            boot_time: 0,
            current_time: 0,
            time_zone_bias: 0,
            time_zone_id: 0,
            reserved: 0,
            boot_time_bias: 0,
            sleep_time_bias: 0,
        }
    }

    pub fn load(addr: u64, maps: &Maps) -> SystemTimeOfDayInformation {
        SystemTimeOfDayInformation {
            boot_time: maps.read_qword(addr).unwrap(),
            current_time: maps.read_qword(addr + 8).unwrap(),
            time_zone_bias: maps.read_qword(addr + 16).unwrap(),
            time_zone_id: maps.read_dword(addr + 24).unwrap(),
            reserved: maps.read_dword(addr + 28).unwrap(),
            boot_time_bias: maps.read_qword(addr + 32).unwrap(),
            sleep_time_bias: maps.read_qword(addr + 40).unwrap(),
        }
    }

    pub fn save(&self, addr: u64, maps: &mut Maps) {
        maps.write_qword(addr, self.boot_time);
        maps.write_qword(addr + 8, self.current_time);
        maps.write_qword(addr + 16, self.time_zone_bias);
        maps.write_dword(addr + 24, self.time_zone_id);
        maps.write_dword(addr + 28, self.reserved);
        maps.write_qword(addr + 32, self.boot_time_bias);
        maps.write_qword(addr + 40, self.sleep_time_bias);
    }
}

#[derive(Debug)]
pub struct SystemProcessInformation {
    pub next_entry_offset: u32,
    pub number_of_threads: u32,
    pub reserved1: [u8; 48],
    pub image_name: UnicodeString,
    pub base_priority: u32,
    pub unique_process_id: u64,
    pub inherited_from_unique_process_id: u64,
    pub handle_count: u32,
    pub session_id: u32,
    pub unique_process_key: u64,
    pub peak_virtual_size: u64,
    pub virtual_size: u64,
    pub page_fault_count: u32,
    pub peak_working_set_size: u64,
    pub working_set_size: u64,
    pub quota_peak_paged_pool_usage: u64,
    pub quota_paged_pool_usage: u64,
    pub quota_peak_non_paged_pool_usage: u64,
    pub quota_non_paged_pool_usage: u64,
    pub pagefile_usage: u64,
    pub peak_pagefile_usage: u64,
    pub private_page_count: u64,
    pub reserved7: [u64; 6],
}

impl Default for SystemProcessInformation {
    fn default() -> Self {
        Self::new()
    }
}

impl SystemProcessInformation {
    pub fn size() -> u32 {
        224
    }

    pub fn new() -> SystemProcessInformation {
        SystemProcessInformation {
            next_entry_offset: 0,
            number_of_threads: 0,
            reserved1: [0; 48],
            image_name: UnicodeString::new(),
            base_priority: 0,
            unique_process_id: 0,
            inherited_from_unique_process_id: 0,
            handle_count: 0,
            session_id: 0,
            unique_process_key: 0,
            peak_virtual_size: 0,
            virtual_size: 0,
            page_fault_count: 0,
            peak_working_set_size: 0,
            working_set_size: 0,
            quota_peak_paged_pool_usage: 0,
            quota_paged_pool_usage: 0,
            quota_peak_non_paged_pool_usage: 0,
            quota_non_paged_pool_usage: 0,
            pagefile_usage: 0,
            peak_pagefile_usage: 0,
            private_page_count: 0,
            reserved7: [0; 6],
        }
    }
}

#[derive(Debug)]
pub struct SystemThreadInformation {
    pub reserved1: [u64; 3],
    pub reserved2: u32,
    pub start_address: u64,
    pub client_id: ClientId,
    pub priority: u32,
    pub base_priority: u32,
    pub context_switches: u32,
    pub thread_state: u32,
    pub wait_reason: u32,
}

impl Default for SystemThreadInformation {
    fn default() -> Self {
        Self::new()
    }
}

impl SystemThreadInformation {
    pub fn size() -> u32 {
        60
    }

    pub fn new() -> SystemThreadInformation {
        SystemThreadInformation {
            reserved1: [0; 3],
            reserved2: 0,
            start_address: 0,
            client_id: ClientId::new(),
            priority: 0,
            base_priority: 0,
            context_switches: 0,
            thread_state: 0,
            wait_reason: 0,
        }
    }

    pub fn load(addr: u64, maps: &Maps) -> SystemThreadInformation {
        SystemThreadInformation {
            reserved1: [
                maps.read_qword(addr).unwrap(),
                maps.read_qword(addr + 8).unwrap(),
                maps.read_qword(addr + 16).unwrap(),
            ],
            reserved2: maps.read_dword(addr + 24).unwrap(),
            start_address: maps.read_qword(addr + 32).unwrap(),
            client_id: ClientId::load(addr + 40, maps),
            priority: maps.read_dword(addr + 48).unwrap(),
            base_priority: maps.read_dword(addr + 52).unwrap(),
            context_switches: maps.read_dword(addr + 56).unwrap(),
            thread_state: maps.read_dword(addr + 60).unwrap(),
            wait_reason: maps.read_dword(addr + 64).unwrap(),
        }
    }

    pub fn save(&self, addr: u64, maps: &mut Maps) {
        maps.write_qword(addr, self.reserved1[0]);
        maps.write_qword(addr + 8, self.reserved1[1]);
        maps.write_qword(addr + 16, self.reserved1[2]);
        maps.write_dword(addr + 24, self.reserved2);
        maps.write_qword(addr + 32, self.start_address);
        self.client_id.save(addr + 40, maps);
        maps.write_dword(addr + 48, self.priority);
        maps.write_dword(addr + 52, self.base_priority);
        maps.write_dword(addr + 56, self.context_switches);
        maps.write_dword(addr + 60, self.thread_state);
        maps.write_dword(addr + 64, self.wait_reason);
    }
}

#[derive(Debug)]
pub struct RtlOsVersionInfoW {
    pub dw_os_version_info_size: u32,
    pub dw_major_version: u32,
    pub dw_minor_version: u32,
    pub dw_build_number: u32,
    pub dw_platform_id: u32,
    pub sz_csd_version: [u8; 256],
}

impl Default for RtlOsVersionInfoW {
    fn default() -> Self {
        Self::new()
    }
}

impl RtlOsVersionInfoW {
    pub fn size() -> u32 {
        276
    }

    pub fn new() -> RtlOsVersionInfoW {
        RtlOsVersionInfoW {
            dw_os_version_info_size: 0,
            dw_major_version: 0,
            dw_minor_version: 0,
            dw_build_number: 0,
            dw_platform_id: 0,
            sz_csd_version: [0; 256],
        }
    }

    pub fn load(addr: u64, maps: &Maps) -> RtlOsVersionInfoW {
        let buf = maps.read_bytes_array::<256>(addr + 20);
        RtlOsVersionInfoW {
            dw_os_version_info_size: maps.read_dword(addr).unwrap(),
            dw_major_version: maps.read_dword(addr + 4).unwrap(),
            dw_minor_version: maps.read_dword(addr + 8).unwrap(),
            dw_build_number: maps.read_dword(addr + 12).unwrap(),
            dw_platform_id: maps.read_dword(addr + 16).unwrap(),
            sz_csd_version: buf,
        }
    }

    pub fn save(&self, addr: u64, maps: &mut Maps) {
        maps.write_dword(addr, self.dw_os_version_info_size);
        maps.write_dword(addr + 4, self.dw_major_version);
        maps.write_dword(addr + 8, self.dw_minor_version);
        maps.write_dword(addr + 12, self.dw_build_number);
        maps.write_dword(addr + 16, self.dw_platform_id);
        maps.write_bytes(addr + 20, &self.sz_csd_version);
    }
}

#[derive(Debug)]
pub struct RtlOsVersionInfoExW {
    pub dw_os_version_info_size: u32,
    pub dw_major_version: u32,
    pub dw_minor_version: u32,
    pub dw_build_number: u32,
    pub dw_platform_id: u32,
    pub sz_csd_version: [u8; 256],
    pub w_service_pack_major: u16,
    pub w_service_pack_minor: u16,
    pub w_suite_mask: u16,
    pub w_product_type: u8,
    pub w_reserved: u8,
}

impl Default for RtlOsVersionInfoExW {
    fn default() -> Self {
        Self::new()
    }
}

impl RtlOsVersionInfoExW {
    pub fn size() -> u32 {
        284
    }

    pub fn new() -> RtlOsVersionInfoExW {
        RtlOsVersionInfoExW {
            dw_os_version_info_size: 0,
            dw_major_version: 0,
            dw_minor_version: 0,
            dw_build_number: 0,
            dw_platform_id: 0,
            sz_csd_version: [0; 256],
            w_service_pack_major: 0,
            w_service_pack_minor: 0,
            w_suite_mask: 0,
            w_product_type: 0,
            w_reserved: 0,
        }
    }

    pub fn load(addr: u64, maps: &Maps) -> RtlOsVersionInfoExW {
        let buf = maps.read_bytes_array::<256>(addr + 20);
        RtlOsVersionInfoExW {
            dw_os_version_info_size: maps.read_dword(addr).unwrap(),
            dw_major_version: maps.read_dword(addr + 4).unwrap(),
            dw_minor_version: maps.read_dword(addr + 8).unwrap(),
            dw_build_number: maps.read_dword(addr + 12).unwrap(),
            dw_platform_id: maps.read_dword(addr + 16).unwrap(),
            sz_csd_version: buf,
            w_service_pack_major: maps.read_word(addr + 276).unwrap(),
            w_service_pack_minor: maps.read_word(addr + 278).unwrap(),
            w_suite_mask: maps.read_word(addr + 280).unwrap(),
            w_product_type: maps.read_byte(addr + 282).unwrap(),
            w_reserved: maps.read_byte(addr + 283).unwrap(),
        }
    }

    pub fn save(&self, addr: u64, maps: &mut Maps) {
        maps.write_dword(addr, self.dw_os_version_info_size);
        maps.write_dword(addr + 4, self.dw_major_version);
        maps.write_dword(addr + 8, self.dw_minor_version);
        maps.write_dword(addr + 12, self.dw_build_number);
        maps.write_dword(addr + 16, self.dw_platform_id);
        maps.write_bytes(addr + 20, &self.sz_csd_version);
        maps.write_word(addr + 276, self.w_service_pack_major);
        maps.write_word(addr + 278, self.w_service_pack_minor);
        maps.write_word(addr + 280, self.w_suite_mask);
        maps.write_byte(addr + 282, self.w_product_type);
        maps.write_byte(addr + 283, self.w_reserved);
    }
}
