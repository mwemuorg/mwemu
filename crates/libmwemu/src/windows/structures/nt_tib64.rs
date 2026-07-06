use crate::maps::{Maps, mem64::Mem64};

#[derive(Debug)]
pub struct NtTib64 {
    pub exception_list: u64,
    pub stack_base: u64,
    pub stack_limit: u64,
    pub sub_system_tib: u64,
    pub fiber_data: u64,
    pub arbitrary_user_pointer: u64,
    pub self_pointer: u64,
}

impl Default for NtTib64 {
    fn default() -> Self {
        Self::new()
    }
}

impl NtTib64 {
    pub fn new() -> NtTib64 {
        NtTib64 {
            exception_list: 0,
            stack_base: 0,
            stack_limit: 0,
            sub_system_tib: 0,
            fiber_data: 0,
            arbitrary_user_pointer: 0,
            self_pointer: 0,
        }
    }

    pub fn size() -> usize {
        56
    }

    pub fn load(addr: u64, maps: &Maps) -> NtTib64 {
        NtTib64 {
            exception_list: maps.read_qword(addr).unwrap(),
            stack_base: maps.read_qword(addr + 8).unwrap(),
            stack_limit: maps.read_qword(addr + 16).unwrap(),
            sub_system_tib: maps.read_qword(addr + 24).unwrap(),
            fiber_data: maps.read_qword(addr + 32).unwrap(),
            arbitrary_user_pointer: maps.read_qword(addr + 40).unwrap(),
            self_pointer: maps.read_qword(addr + 48).unwrap(),
        }
    }

    pub fn load_map(addr: u64, map: &Mem64) -> NtTib64 {
        NtTib64 {
            exception_list: map.read_qword(addr),
            stack_base: map.read_qword(addr + 8),
            stack_limit: map.read_qword(addr + 16),
            sub_system_tib: map.read_qword(addr + 24),
            fiber_data: map.read_qword(addr + 32),
            arbitrary_user_pointer: map.read_qword(addr + 40),
            self_pointer: map.read_qword(addr + 48),
        }
    }

    pub fn save(&self, base: u64, mem: &mut Mem64) {
        mem.write_qword(base, self.exception_list);
        mem.write_qword(base + 8, self.stack_base);
        mem.write_qword(base + 16, self.stack_limit);
        mem.write_qword(base + 24, self.sub_system_tib);
        mem.write_qword(base + 32, self.fiber_data);
        mem.write_qword(base + 40, self.arbitrary_user_pointer);
        mem.write_qword(base + 48, self.self_pointer);
    }
}
