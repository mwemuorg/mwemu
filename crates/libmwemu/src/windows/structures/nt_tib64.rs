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
    pub(crate) const STACK_BASE_OFFSET: u64 = 0x08;
    pub(crate) const STACK_LIMIT_OFFSET: u64 = 0x10;
    pub(crate) const SELF_POINTER_OFFSET: u64 = 0x30;

    const SUB_SYSTEM_TIB_OFFSET: u64 = 0x18;
    const FIBER_DATA_OFFSET: u64 = 0x20;
    const ARBITRARY_USER_POINTER_OFFSET: u64 = 0x28;

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
            exception_list: maps.read_qword(addr).unwrap_or(0),
            stack_base: maps.read_qword(addr + Self::STACK_BASE_OFFSET).unwrap_or(0),
            stack_limit: maps
                .read_qword(addr + Self::STACK_LIMIT_OFFSET)
                .unwrap_or(0),
            sub_system_tib: maps
                .read_qword(addr + Self::SUB_SYSTEM_TIB_OFFSET)
                .unwrap_or(0),
            fiber_data: maps.read_qword(addr + Self::FIBER_DATA_OFFSET).unwrap_or(0),
            arbitrary_user_pointer: maps
                .read_qword(addr + Self::ARBITRARY_USER_POINTER_OFFSET)
                .unwrap_or(0),
            self_pointer: maps
                .read_qword(addr + Self::SELF_POINTER_OFFSET)
                .unwrap_or(0),
        }
    }

    pub fn load_map(addr: u64, map: &Mem64) -> NtTib64 {
        NtTib64 {
            exception_list: map.read_qword(addr),
            stack_base: map.read_qword(addr + Self::STACK_BASE_OFFSET),
            stack_limit: map.read_qword(addr + Self::STACK_LIMIT_OFFSET),
            sub_system_tib: map.read_qword(addr + Self::SUB_SYSTEM_TIB_OFFSET),
            fiber_data: map.read_qword(addr + Self::FIBER_DATA_OFFSET),
            arbitrary_user_pointer: map.read_qword(addr + Self::ARBITRARY_USER_POINTER_OFFSET),
            self_pointer: map.read_qword(addr + Self::SELF_POINTER_OFFSET),
        }
    }

    pub fn save(&self, base: u64, mem: &mut Mem64) {
        mem.write_qword(base, self.exception_list);
        mem.write_qword(base + Self::STACK_BASE_OFFSET, self.stack_base);
        mem.write_qword(base + Self::STACK_LIMIT_OFFSET, self.stack_limit);
        mem.write_qword(base + Self::SUB_SYSTEM_TIB_OFFSET, self.sub_system_tib);
        mem.write_qword(base + Self::FIBER_DATA_OFFSET, self.fiber_data);
        mem.write_qword(
            base + Self::ARBITRARY_USER_POINTER_OFFSET,
            self.arbitrary_user_pointer,
        );
        mem.write_qword(base + Self::SELF_POINTER_OFFSET, self.self_pointer);
    }
}
