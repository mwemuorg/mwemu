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
    pub process_environment_block: u32,
    pub last_error_value: u32,
    pub count_of_owned_critical_sections: u32,
    pub csr_client_thread: u32,
    pub win32_thread_info: u32,
    pub user32_reserved: [u32; 26],
    pub user_reserved: [u32; 5],
    pub wow32_reserved: u32,
    pub current_locale: u32,
    pub fp_software_status_register: u32,
    pub exception_code: u32,
    pub activation_context_stack_pointer: u32,
}

impl TEB {
    pub const NT_TIB_OFFSET: u64 = 0x00;
    pub const ENVIRONMENT_POINTER_OFFSET: u64 = 0x1c;
    pub const CLIENT_ID_OFFSET: u64 = 0x20;
    pub const CLIENT_ID_PROCESS_ID_OFFSET: u64 = 0x20;
    pub const CLIENT_ID_THREAD_ID_OFFSET: u64 = 0x24;
    pub const ACTIVE_RPC_HANDLE_OFFSET: u64 = 0x28;
    pub const THREAD_LOCAL_STORAGE_POINTER_OFFSET: u64 = 0x2c;
    pub const PROCESS_ENVIRONMENT_BLOCK_OFFSET: u64 = 0x30;
    pub const LAST_ERROR_VALUE_OFFSET: u64 = 0x34;
    pub const COUNT_OF_OWNED_CRITICAL_SECTIONS_OFFSET: u64 = 0x38;
    pub const CSR_CLIENT_THREAD_OFFSET: u64 = 0x3c;
    pub const WIN32_THREAD_INFO_OFFSET: u64 = 0x40;
    pub const USER32_RESERVED_OFFSET: u64 = 0x44;
    pub const USER_RESERVED_OFFSET: u64 = 0xac;
    pub const WOW32_RESERVED_OFFSET: u64 = 0xc0;
    pub const CURRENT_LOCALE_OFFSET: u64 = 0xc4;
    pub const FP_SOFTWARE_STATUS_REGISTER_OFFSET: u64 = 0xc8;
    pub const EXCEPTION_CODE_OFFSET: u64 = 0x1a4;
    pub const ACTIVATION_CONTEXT_STACK_POINTER_OFFSET: u64 = 0x1a8;

    pub fn size() -> usize {
        0x1038
    }

    pub fn map_size() -> usize {
        0x2000
    }

    pub fn new(peb_addr: u32) -> TEB {
        TEB {
            nt_tib: NtTib32::new(),
            environment_pointer: 0,
            process_id: 3240,
            thread_id: 1,
            active_rpc_handle: 0,
            thread_local_storage_pointer: 0,
            process_environment_block: peb_addr,
            last_error_value: 0,
            count_of_owned_critical_sections: 0,
            csr_client_thread: 0,
            win32_thread_info: 0,
            user32_reserved: [0; 26],
            user_reserved: [0; 5],
            wow32_reserved: 0,
            current_locale: 0,
            fp_software_status_register: 0,
            exception_code: 0,
            activation_context_stack_pointer: 0,
        }
    }

    pub fn set_last_error(&mut self, err: u32) {
        self.last_error_value = err;
    }

    pub fn load(addr: u64, maps: &Maps) -> TEB {
        let mut user32_reserved = [0; 26];
        for (index, value) in user32_reserved.iter_mut().enumerate() {
            *value = maps
                .read_dword(Self::USER32_RESERVED_OFFSET + addr + index as u64 * 4)
                .unwrap();
        }
        let mut user_reserved = [0; 5];
        for (index, value) in user_reserved.iter_mut().enumerate() {
            *value = maps
                .read_dword(Self::USER_RESERVED_OFFSET + addr + index as u64 * 4)
                .unwrap();
        }

        TEB {
            nt_tib: NtTib32::load(addr + Self::NT_TIB_OFFSET, maps),
            environment_pointer: maps
                .read_dword(addr + Self::ENVIRONMENT_POINTER_OFFSET)
                .unwrap(),
            process_id: maps
                .read_dword(addr + Self::CLIENT_ID_PROCESS_ID_OFFSET)
                .unwrap(),
            thread_id: maps
                .read_dword(addr + Self::CLIENT_ID_THREAD_ID_OFFSET)
                .unwrap(),
            active_rpc_handle: maps
                .read_dword(addr + Self::ACTIVE_RPC_HANDLE_OFFSET)
                .unwrap(),
            thread_local_storage_pointer: maps
                .read_dword(addr + Self::THREAD_LOCAL_STORAGE_POINTER_OFFSET)
                .unwrap(),
            process_environment_block: maps
                .read_dword(addr + Self::PROCESS_ENVIRONMENT_BLOCK_OFFSET)
                .unwrap(),
            last_error_value: maps
                .read_dword(addr + Self::LAST_ERROR_VALUE_OFFSET)
                .unwrap(),
            count_of_owned_critical_sections: maps
                .read_dword(addr + Self::COUNT_OF_OWNED_CRITICAL_SECTIONS_OFFSET)
                .unwrap(),
            csr_client_thread: maps
                .read_dword(addr + Self::CSR_CLIENT_THREAD_OFFSET)
                .unwrap(),
            win32_thread_info: maps
                .read_dword(addr + Self::WIN32_THREAD_INFO_OFFSET)
                .unwrap(),
            user32_reserved,
            user_reserved,
            wow32_reserved: maps.read_dword(addr + Self::WOW32_RESERVED_OFFSET).unwrap(),
            current_locale: maps.read_dword(addr + Self::CURRENT_LOCALE_OFFSET).unwrap(),
            fp_software_status_register: maps
                .read_dword(addr + Self::FP_SOFTWARE_STATUS_REGISTER_OFFSET)
                .unwrap(),
            exception_code: maps.read_dword(addr + Self::EXCEPTION_CODE_OFFSET).unwrap(),
            activation_context_stack_pointer: maps
                .read_dword(addr + Self::ACTIVATION_CONTEXT_STACK_POINTER_OFFSET)
                .unwrap(),
        }
    }

    pub fn load_map(addr: u64, map: &Mem64) -> TEB {
        let mut user32_reserved = [0; 26];
        for (index, value) in user32_reserved.iter_mut().enumerate() {
            *value = map.read_dword(Self::USER32_RESERVED_OFFSET + addr + index as u64 * 4);
        }
        let mut user_reserved = [0; 5];
        for (index, value) in user_reserved.iter_mut().enumerate() {
            *value = map.read_dword(Self::USER_RESERVED_OFFSET + addr + index as u64 * 4);
        }

        TEB {
            nt_tib: NtTib32::load_map(addr + Self::NT_TIB_OFFSET, map),
            environment_pointer: map.read_dword(addr + Self::ENVIRONMENT_POINTER_OFFSET),
            process_id: map.read_dword(addr + Self::CLIENT_ID_PROCESS_ID_OFFSET),
            thread_id: map.read_dword(addr + Self::CLIENT_ID_THREAD_ID_OFFSET),
            active_rpc_handle: map.read_dword(addr + Self::ACTIVE_RPC_HANDLE_OFFSET),
            thread_local_storage_pointer: map
                .read_dword(addr + Self::THREAD_LOCAL_STORAGE_POINTER_OFFSET),
            process_environment_block: map
                .read_dword(addr + Self::PROCESS_ENVIRONMENT_BLOCK_OFFSET),
            last_error_value: map.read_dword(addr + Self::LAST_ERROR_VALUE_OFFSET),
            count_of_owned_critical_sections: map
                .read_dword(addr + Self::COUNT_OF_OWNED_CRITICAL_SECTIONS_OFFSET),
            csr_client_thread: map.read_dword(addr + Self::CSR_CLIENT_THREAD_OFFSET),
            win32_thread_info: map.read_dword(addr + Self::WIN32_THREAD_INFO_OFFSET),
            user32_reserved,
            user_reserved,
            wow32_reserved: map.read_dword(addr + Self::WOW32_RESERVED_OFFSET),
            current_locale: map.read_dword(addr + Self::CURRENT_LOCALE_OFFSET),
            fp_software_status_register: map
                .read_dword(addr + Self::FP_SOFTWARE_STATUS_REGISTER_OFFSET),
            exception_code: map.read_dword(addr + Self::EXCEPTION_CODE_OFFSET),
            activation_context_stack_pointer: map
                .read_dword(addr + Self::ACTIVATION_CONTEXT_STACK_POINTER_OFFSET),
        }
    }

    pub fn save(&self, mem: &mut Mem64) {
        let base = mem.get_base();
        self.nt_tib.save(base + Self::NT_TIB_OFFSET, mem);
        mem.write_dword(
            base + Self::ENVIRONMENT_POINTER_OFFSET,
            self.environment_pointer,
        );
        mem.write_dword(base + Self::CLIENT_ID_PROCESS_ID_OFFSET, self.process_id);
        mem.write_dword(base + Self::CLIENT_ID_THREAD_ID_OFFSET, self.thread_id);
        mem.write_dword(
            base + Self::ACTIVE_RPC_HANDLE_OFFSET,
            self.active_rpc_handle,
        );
        mem.write_dword(
            base + Self::THREAD_LOCAL_STORAGE_POINTER_OFFSET,
            self.thread_local_storage_pointer,
        );
        mem.write_dword(
            base + Self::PROCESS_ENVIRONMENT_BLOCK_OFFSET,
            self.process_environment_block,
        );
        mem.write_dword(base + Self::LAST_ERROR_VALUE_OFFSET, self.last_error_value);
        mem.write_dword(
            base + Self::COUNT_OF_OWNED_CRITICAL_SECTIONS_OFFSET,
            self.count_of_owned_critical_sections,
        );
        mem.write_dword(
            base + Self::CSR_CLIENT_THREAD_OFFSET,
            self.csr_client_thread,
        );
        mem.write_dword(
            base + Self::WIN32_THREAD_INFO_OFFSET,
            self.win32_thread_info,
        );
        for (index, value) in self.user32_reserved.iter().enumerate() {
            mem.write_dword(
                base + Self::USER32_RESERVED_OFFSET + index as u64 * 4,
                *value,
            );
        }
        for (index, value) in self.user_reserved.iter().enumerate() {
            mem.write_dword(base + Self::USER_RESERVED_OFFSET + index as u64 * 4, *value);
        }
        mem.write_dword(base + Self::WOW32_RESERVED_OFFSET, self.wow32_reserved);
        mem.write_dword(base + Self::CURRENT_LOCALE_OFFSET, self.current_locale);
        mem.write_dword(
            base + Self::FP_SOFTWARE_STATUS_REGISTER_OFFSET,
            self.fp_software_status_register,
        );
        mem.write_dword(base + Self::EXCEPTION_CODE_OFFSET, self.exception_code);
        mem.write_dword(
            base + Self::ACTIVATION_CONTEXT_STACK_POINTER_OFFSET,
            self.activation_context_stack_pointer,
        );
    }

    pub fn print(&self) {
        log::trace!("{:#x?}", self);
    }
}
