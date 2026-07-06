use super::nt_tib64::NtTib64;
use crate::maps::Maps;
use crate::maps::mem64::Mem64;

#[derive(Debug)]
pub struct TEB64 {
    pub nt_tib: NtTib64,
    pub environment_pointer: u64,
    pub process_id: u64,
    pub thread_id: u64,
    pub active_rpc_handle: u64,
    pub thread_local_storage_pointer: u64,
    pub process_environment_block: u64, // PEB64
    pub last_error_value: u32,
    pub count_of_owned_critical_sections: u32,
    pub csr_client_thread: u64,
    pub win32_thread_info: u64,
    pub user32_reserved: [u32; 26],
    pub user_reserved: [u32; 6],
    pub wow32_reserved: u64,
    pub current_locale: u32,
    pub fp_software_status_register: u32,
    pub system_reserved1: [u64; 54],
    pub exception_code: u32,
    pub activation_context_stack_pointer: u64,
}

impl TEB64 {
    pub fn new(peb_addr: u64) -> TEB64 {
        TEB64 {
            nt_tib: NtTib64::new(),
            environment_pointer: 0,
            process_id: 1,
            thread_id: 1,
            active_rpc_handle: 0,
            thread_local_storage_pointer: 0,
            process_environment_block: peb_addr,
            last_error_value: 0,
            count_of_owned_critical_sections: 0,
            csr_client_thread: 0,
            win32_thread_info: 0,
            user32_reserved: [0; 26],
            user_reserved: [0; 6],
            wow32_reserved: 0,
            current_locale: 0x409, // en-US
            fp_software_status_register: 0,
            system_reserved1: [0; 54],
            exception_code: 0,
            activation_context_stack_pointer: 0,
        }
    }

    /// Patch TEB fields that depend on the TEB's own address or the stack layout.
    /// Call this after the TEB has been saved to memory.
    pub fn patch_addresses(
        teb_addr: u64,
        stack_base: u64,
        stack_size: u64,
        maps: &mut crate::maps::Maps,
    ) {
        // NtTib.Self (offset 0x30): pointer to TEB itself — used by gs:[0x30]
        maps.write_qword(teb_addr + 0x30, teb_addr);
        // NtTib.StackBase (offset 0x08): high address (top) of the thread stack
        maps.write_qword(teb_addr + 0x08, stack_base + stack_size);
        // NtTib.StackLimit (offset 0x10): low address (bottom) of the committed stack
        maps.write_qword(teb_addr + 0x10, stack_base);
    }

    pub fn size() -> usize {
        0x1878 // full Windows x64 TEB size
    }

    pub fn load(addr: u64, maps: &Maps) -> TEB64 {
        TEB64 {
            nt_tib: NtTib64::load(addr, maps),
            environment_pointer: maps.read_qword(addr + 0x38).unwrap(),
            process_id: maps.read_qword(addr + 0x40).unwrap(),
            thread_id: maps.read_qword(addr + 0x48).unwrap(),
            active_rpc_handle: maps.read_qword(addr + 0x50).unwrap(),
            thread_local_storage_pointer: maps.read_qword(addr + 0x58).unwrap(),
            process_environment_block: maps.read_qword(addr + 0x60).unwrap(),
            last_error_value: maps.read_dword(addr + 0x68).unwrap(),
            count_of_owned_critical_sections: maps.read_dword(addr + 0x6c).unwrap(),
            csr_client_thread: maps.read_qword(addr + 0x70).unwrap(),
            win32_thread_info: maps.read_qword(addr + 0x78).unwrap(),
            user32_reserved: [0; 26],
            user_reserved: [0; 6],
            wow32_reserved: maps.read_qword(addr + 0x100).unwrap(),
            current_locale: maps.read_dword(addr + 0x108).unwrap(),
            fp_software_status_register: maps.read_dword(addr + 0x10c).unwrap(),
            system_reserved1: [0; 54],
            exception_code: maps.read_dword(addr + 0x2c0).unwrap(),
            activation_context_stack_pointer: maps.read_qword(addr + 0x2c8).unwrap(),
        }
    }
    pub fn set_last_error(&mut self, err: u32) {
        self.last_error_value = err;
    }

    pub fn load_map(addr: u64, map: &Mem64) -> TEB64 {
        TEB64 {
            nt_tib: NtTib64::load_map(addr, map),
            environment_pointer: map.read_qword(addr + 0x38),
            process_id: map.read_qword(addr + 0x40),
            thread_id: map.read_qword(addr + 0x48),
            active_rpc_handle: map.read_qword(addr + 0x50),
            thread_local_storage_pointer: map.read_qword(addr + 0x58),
            process_environment_block: map.read_qword(addr + 0x60),
            last_error_value: map.read_dword(addr + 0x68),
            count_of_owned_critical_sections: map.read_dword(addr + 0x6c),
            csr_client_thread: map.read_qword(addr + 0x70),
            win32_thread_info: map.read_qword(addr + 0x78),
            user32_reserved: [0; 26],
            user_reserved: [0; 6],
            wow32_reserved: map.read_qword(addr + 0x100),
            current_locale: map.read_dword(addr + 0x108),
            fp_software_status_register: map.read_dword(addr + 0x10c),
            system_reserved1: [0; 54],
            exception_code: map.read_dword(addr + 0x2c0),
            activation_context_stack_pointer: map.read_qword(addr + 0x2c8),
        }
    }

    pub fn save(&self, mem: &mut Mem64) {
        let base = mem.get_base();
        self.nt_tib.save(base, mem);
        mem.write_qword(base + 0x38, self.environment_pointer);
        mem.write_qword(base + 0x40, self.process_id);
        mem.write_qword(base + 0x48, self.thread_id);
        mem.write_qword(base + 0x50, self.active_rpc_handle);
        mem.write_qword(base + 0x58, self.thread_local_storage_pointer);
        mem.write_qword(base + 0x60, self.process_environment_block);
        mem.write_dword(base + 0x68, self.last_error_value);
        mem.write_dword(base + 0x6c, self.count_of_owned_critical_sections);
        mem.write_qword(base + 0x70, self.csr_client_thread);
        mem.write_qword(base + 0x78, self.win32_thread_info);
        let mut idx = base + 0x100;
        for i in 0..26 {
            mem.write_dword(idx, self.user32_reserved[i]);
            idx += 4;
        }
        let mut idx = base + 0x108;
        for i in 0..6 {
            mem.write_dword(idx, self.user_reserved[i]);
            idx += 4;
        }
        mem.write_qword(base + 0x100, self.wow32_reserved);
        mem.write_dword(base + 0x108, self.current_locale);
        mem.write_dword(base + 0x10c, self.fp_software_status_register);
        mem.write_dword(base + 0x2c0, self.exception_code);
        mem.write_qword(base + 0x2c8, self.activation_context_stack_pointer);
    }

    pub fn print(&self) {
        log::trace!("{:#x?}", self);
    }
}
