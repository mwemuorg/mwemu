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
    pub(crate) const STACK_BASE_OFFSET: u64 = 0x04;
    pub(crate) const STACK_LIMIT_OFFSET: u64 = 0x08;

    const SUB_SYSTEM_TIB_OFFSET: u64 = 0x0c;
    const FIBER_DATA_OFFSET: u64 = 0x10;
    const ARBITRARY_USER_POINTER_OFFSET: u64 = 0x14;
    const SELF_POINTER_OFFSET: u64 = 0x18;

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
            exception_list: maps.read_dword(addr).unwrap_or(0),
            stack_base: maps.read_dword(addr + Self::STACK_BASE_OFFSET).unwrap_or(0),
            stack_limit: maps
                .read_dword(addr + Self::STACK_LIMIT_OFFSET)
                .unwrap_or(0),
            sub_system_tib: maps
                .read_dword(addr + Self::SUB_SYSTEM_TIB_OFFSET)
                .unwrap_or(0),
            fiber_data: maps.read_dword(addr + Self::FIBER_DATA_OFFSET).unwrap_or(0),
            arbitrary_user_pointer: maps
                .read_dword(addr + Self::ARBITRARY_USER_POINTER_OFFSET)
                .unwrap_or(0),
            self_pointer: maps
                .read_dword(addr + Self::SELF_POINTER_OFFSET)
                .unwrap_or(0),
        }
    }

    pub fn load_map(addr: u64, map: &Mem64) -> NtTib32 {
        NtTib32 {
            exception_list: map.read_dword(addr),
            stack_base: map.read_dword(addr + Self::STACK_BASE_OFFSET),
            stack_limit: map.read_dword(addr + Self::STACK_LIMIT_OFFSET),
            sub_system_tib: map.read_dword(addr + Self::SUB_SYSTEM_TIB_OFFSET),
            fiber_data: map.read_dword(addr + Self::FIBER_DATA_OFFSET),
            arbitrary_user_pointer: map.read_dword(addr + Self::ARBITRARY_USER_POINTER_OFFSET),
            self_pointer: map.read_dword(addr + Self::SELF_POINTER_OFFSET),
        }
    }

    pub fn save(&self, addr: u64, mem: &mut Mem64) {
        mem.write_dword(addr, self.exception_list);
        mem.write_dword(addr + Self::STACK_BASE_OFFSET, self.stack_base);
        mem.write_dword(addr + Self::STACK_LIMIT_OFFSET, self.stack_limit);
        mem.write_dword(addr + Self::SUB_SYSTEM_TIB_OFFSET, self.sub_system_tib);
        mem.write_dword(addr + Self::FIBER_DATA_OFFSET, self.fiber_data);
        mem.write_dword(
            addr + Self::ARBITRARY_USER_POINTER_OFFSET,
            self.arbitrary_user_pointer,
        );
        mem.write_dword(addr + Self::SELF_POINTER_OFFSET, self.self_pointer);
    }
}
