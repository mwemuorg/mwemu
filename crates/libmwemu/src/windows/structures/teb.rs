use super::nt_tib32::NtTib32;
use crate::maps::Maps;
use crate::maps::mem64::Mem64;

#[derive(Debug)]
pub struct TEB {
    pub nt_tib: NtTib32,
    pub environment_pointer: u32,
    pub process_id: u32,
    pub thread_id: u32,
    pub active_rpc_handle: u32,
    pub thread_local_storage_pointer: u32,
    pub process_environment_block: u32, // PEB 0x30
    pub last_error_value: u32,
    pub count_of_owned_critical_sections: u32,
    pub csr_client_thread: u32,
    pub win32_thread_info: u32,
    pub user32_reserved: [u32; 26],
    pub user_reserved: [u32; 6],
    pub wow32_reserved: u32,
    pub current_locale: u32,
    pub fp_software_status_register: u32,
    pub system_reserved1: [u64; 54],
    pub exception_code: u32,
    pub activation_context_stack_pointer: u32,
    pub spare_bytes: [u8; 24],
    pub tx_fs_context: u32,
}

impl TEB {
    pub fn size() -> usize {
        1000
    }

    pub fn new(peb_addr: u32) -> TEB {
        TEB {
            nt_tib: NtTib32::new(),
            environment_pointer: 0,
            process_id: 3240,
            thread_id: 1,
            active_rpc_handle: 0,
            thread_local_storage_pointer: 0,
            process_environment_block: peb_addr, // PEB 0x30
            last_error_value: 0,
            count_of_owned_critical_sections: 0,
            csr_client_thread: 0,
            win32_thread_info: 0,
            user32_reserved: [0; 26],
            user_reserved: [0; 6],
            wow32_reserved: 0,
            current_locale: 0,
            fp_software_status_register: 0,
            system_reserved1: [0; 54],
            exception_code: 0,
            activation_context_stack_pointer: 0,
            spare_bytes: [0; 24],
            tx_fs_context: 0,
        }
    }

    pub fn set_last_error(&mut self, err: u32) {
        self.last_error_value = err;
    }

    pub fn load(addr: u64, maps: &Maps) -> TEB {
        TEB {
            nt_tib: NtTib32::load(addr, maps),
            environment_pointer: maps.read_dword(addr + 28).unwrap(),
            process_id: maps.read_dword(addr + 32).unwrap(),
            thread_id: maps.read_dword(addr + 36).unwrap(),
            active_rpc_handle: maps.read_dword(addr + 40).unwrap(),
            thread_local_storage_pointer: maps.read_dword(addr + 44).unwrap(),
            process_environment_block: maps.read_dword(addr + 48).unwrap(),
            last_error_value: maps.read_dword(addr + 52).unwrap(),
            count_of_owned_critical_sections: maps.read_dword(addr + 56).unwrap(),
            csr_client_thread: maps.read_dword(addr + 60).unwrap(),
            win32_thread_info: maps.read_dword(addr + 64).unwrap(),
            user32_reserved: [0; 26],
            user_reserved: [0; 6],
            wow32_reserved: maps.read_dword(addr + 70).unwrap(),
            current_locale: maps.read_dword(addr + 74).unwrap(),
            fp_software_status_register: maps.read_dword(addr + 78).unwrap(),
            system_reserved1: [0; 54],
            exception_code: maps.read_dword(addr + 82).unwrap(),
            activation_context_stack_pointer: maps.read_dword(addr + 86).unwrap(),
            spare_bytes: [0; 24],
            tx_fs_context: maps.read_dword(addr + 190).unwrap(),
        }
    }

    pub fn load_map(addr: u64, map: &Mem64) -> TEB {
        TEB {
            nt_tib: NtTib32::load_map(addr, map),
            environment_pointer: map.read_dword(addr + 28),
            process_id: map.read_dword(addr + 32),
            thread_id: map.read_dword(addr + 36),
            active_rpc_handle: map.read_dword(addr + 40),
            thread_local_storage_pointer: map.read_dword(addr + 44),
            process_environment_block: map.read_dword(addr + 48),
            last_error_value: map.read_dword(addr + 52),
            count_of_owned_critical_sections: map.read_dword(addr + 56),
            csr_client_thread: map.read_dword(addr + 60),
            win32_thread_info: map.read_dword(addr + 64),
            user32_reserved: [0; 26],
            user_reserved: [0; 6],
            wow32_reserved: map.read_dword(addr + 70),
            current_locale: map.read_dword(addr + 74),
            fp_software_status_register: map.read_dword(addr + 78),
            system_reserved1: [0; 54],
            exception_code: map.read_dword(addr + 82),
            activation_context_stack_pointer: map.read_dword(addr + 86),
            spare_bytes: [0; 24],
            tx_fs_context: map.read_dword(addr + 190),
        }
    }

    pub fn save(&self, mem: &mut Mem64) {
        let base = mem.get_base();
        self.nt_tib.save(base, mem);
        mem.write_dword(base + 28, self.environment_pointer);
        mem.write_dword(base + 32, self.process_id);
        mem.write_dword(base + 36, self.thread_id);
        mem.write_dword(base + 40, self.active_rpc_handle);
        mem.write_dword(base + 44, self.thread_local_storage_pointer);
        mem.write_dword(base + 48, self.process_environment_block);
        mem.write_dword(base + 52, self.last_error_value);
        mem.write_dword(base + 56, self.count_of_owned_critical_sections);
        mem.write_dword(base + 60, self.csr_client_thread);
        mem.write_dword(base + 64, self.win32_thread_info);
        //maps.write_dword(addr + 68, self.user32_reserved);
        //maps.write_dword(addr + 70, self.user_reserved);
        mem.write_dword(base + 70, self.wow32_reserved);
        mem.write_dword(base + 74, self.current_locale);
        mem.write_dword(base + 78, self.fp_software_status_register);
        mem.write_dword(base + 82, self.exception_code);
        mem.write_dword(base + 86, self.activation_context_stack_pointer);
    }

    pub fn print(&self) {
        log::trace!("{:#x?}", self);
    }
}
