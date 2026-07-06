use crate::maps::{Maps, mem64::Mem64};

#[derive(Debug)]
pub struct NtTib32 {
    pub exception_list: u32,
    pub stack_base: u32,
    pub stack_limit: u32,
    pub sub_system_tib: u32,
    pub fiber_data: u32,
    pub arbitrary_user_pointer: u32,
    pub self_pointer: u32,
}

impl Default for NtTib32 {
    fn default() -> Self {
        Self::new()
    }
}

impl NtTib32 {
    pub fn size() -> usize {
        28
    }

    pub fn new() -> NtTib32 {
        NtTib32 {
            exception_list: 0,
            stack_base: 0,
            stack_limit: 0,
            sub_system_tib: 0,
            fiber_data: 0,
            arbitrary_user_pointer: 0,
            self_pointer: 0,
        }
    }

    pub fn load(addr: u64, maps: &Maps) -> NtTib32 {
        NtTib32 {
            exception_list: maps.read_dword(addr).unwrap(),
            stack_base: maps.read_dword(addr + 4).unwrap(),
            stack_limit: maps.read_dword(addr + 8).unwrap(),
            sub_system_tib: maps.read_dword(addr + 12).unwrap(),
            fiber_data: maps.read_dword(addr + 16).unwrap(),
            arbitrary_user_pointer: maps.read_dword(addr + 20).unwrap(),
            self_pointer: maps.read_dword(addr + 24).unwrap(),
        }
    }

    pub fn load_map(addr: u64, map: &Mem64) -> NtTib32 {
        NtTib32 {
            exception_list: map.read_dword(addr),
            stack_base: map.read_dword(addr + 4),
            stack_limit: map.read_dword(addr + 8),
            sub_system_tib: map.read_dword(addr + 12),
            fiber_data: map.read_dword(addr + 16),
            arbitrary_user_pointer: map.read_dword(addr + 20),
            self_pointer: map.read_dword(addr + 24),
        }
    }

    pub fn save(&self, addr: u64, mem: &mut Mem64) {
        mem.write_dword(addr, self.exception_list);
        mem.write_dword(addr + 4, self.stack_base);
        mem.write_dword(addr + 8, self.stack_limit);
        mem.write_dword(addr + 12, self.sub_system_tib);
        mem.write_dword(addr + 16, self.fiber_data);
        mem.write_dword(addr + 20, self.arbitrary_user_pointer);
        mem.write_dword(addr + 24, self.self_pointer);
    }
}
