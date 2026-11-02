use crate::maps::Maps;

/// `MEMORY_BASIC_INFORMATION` for **x64** (Windows SDK `winnt.h`): 48 bytes.
///
/// The legacy [`MemoryBasicInformation`] type uses a compact 30-byte layout used in some
/// 32-bit paths; **kernel/user code on x64 expects 64-bit pointers and `SIZE_T RegionSize`**
/// at offsets 0, 8, and 24. Filling the wrong layout makes `VirtualQuery`/`NtQueryVirtualMemory`
/// consumers read garbage (e.g. bogus RIP like `0x85cc`).
#[derive(Debug, Clone)]
pub struct MemoryBasicInformation64 {
    pub base_address: u64,
    pub allocation_base: u64,
    pub allocation_protect: u32,
    pub partition_id: u16,
    pub reserved: u16,
    pub region_size: u64,
    pub state: u32,
    pub protect: u32,
    pub typ: u32,
}

impl MemoryBasicInformation64 {
    pub const SIZE: u64 = 48;

    pub fn guess(addr: u64, maps: &mut Maps) -> Self {
        match maps.get_mem_by_addr_mut(addr) {
            Some(mem) => MemoryBasicInformation64 {
                base_address: mem.get_base(),
                allocation_base: mem.get_base(),
                allocation_protect: 0x40, // PAGE_EXECUTE_READWRITE
                partition_id: 0,
                reserved: 0,
                region_size: mem.size() as u64,
                state: 0x1000, // MEM_COMMIT
                protect: 0x40,
                typ: 0x20000, // MEM_PRIVATE
            },
            None => MemoryBasicInformation64 {
                base_address: 0,
                allocation_base: 0,
                allocation_protect: 0,
                partition_id: 0,
                reserved: 0,
                region_size: 0,
                state: 0x10000, // MEM_FREE
                protect: 1,     // PAGE_NOACCESS
                typ: 0,
            },
        }
    }

    pub fn load(addr: u64, maps: &Maps) -> Self {
        Self {
            base_address: maps.read_qword(addr).unwrap_or(0),
            allocation_base: maps.read_qword(addr + 8).unwrap_or(0),
            allocation_protect: maps.read_dword(addr + 16).unwrap_or(0),
            partition_id: maps.read_word(addr + 20).unwrap_or(0),
            reserved: maps.read_word(addr + 22).unwrap_or(0),
            region_size: maps.read_qword(addr + 24).unwrap_or(0),
            state: maps.read_dword(addr + 32).unwrap_or(0),
            protect: maps.read_dword(addr + 36).unwrap_or(0),
            typ: maps.read_dword(addr + 40).unwrap_or(0),
        }
    }

    pub fn save(&self, addr: u64, maps: &mut Maps) {
        maps.write_qword(addr, self.base_address);
        maps.write_qword(addr + 8, self.allocation_base);
        maps.write_dword(addr + 16, self.allocation_protect);
        maps.write_word(addr + 20, self.partition_id);
        maps.write_word(addr + 22, self.reserved);
        maps.write_qword(addr + 24, self.region_size);
        maps.write_dword(addr + 32, self.state);
        maps.write_dword(addr + 36, self.protect);
        maps.write_dword(addr + 40, self.typ);
        // Trailing padding so sizeof == 48 (MSVC x64).
        maps.write_dword(addr + 44, 0);
    }
}

#[derive(Debug)]
pub struct MemoryBasicInformation {
    pub base_address: u32,
    pub allocation_base: u32,
    pub allocation_protect: u32,
    pub partition_id: u16,
    pub region_size: u32,
    pub state: u32,
    pub protect: u32,
    pub typ: u32,
}

impl MemoryBasicInformation {
    pub fn guess(addr: u64, maps: &mut Maps) -> MemoryBasicInformation {
        match maps.get_mem_by_addr_mut(addr) {
            Some(mem) => MemoryBasicInformation {
                base_address: mem.get_base() as u32,
                allocation_base: mem.get_base() as u32,
                allocation_protect: 0xff,
                partition_id: 0,
                region_size: mem.size() as u32,
                state: 0,
                protect: 0xff,
                typ: 0,
            },
            None => MemoryBasicInformation {
                base_address: 0,
                allocation_base: 0,
                allocation_protect: 0xff,
                partition_id: 0,
                region_size: 0,
                state: 0,
                protect: 0xff,
                typ: 0,
            },
        }
    }

    pub fn load(addr: u64, maps: &Maps) -> MemoryBasicInformation {
        MemoryBasicInformation {
            base_address: maps.read_dword(addr).unwrap_or(0),
            allocation_base: maps.read_dword(addr + 4).unwrap_or(0),
            allocation_protect: maps.read_dword(addr + 8).unwrap_or(0),
            partition_id: maps.read_word(addr + 12).unwrap_or(0),
            region_size: maps.read_dword(addr + 14).unwrap_or(0),
            state: maps.read_dword(addr + 18).unwrap_or(0),
            protect: maps.read_dword(addr + 22).unwrap_or(0),
            typ: maps.read_dword(addr + 26).unwrap_or(0),
        }
    }

    pub fn size() -> u64 {
        30
    }

    pub fn save(&self, addr: u64, maps: &mut Maps) {
        maps.write_dword(addr, self.base_address);
        maps.write_dword(addr + 4, self.allocation_base);
        maps.write_dword(addr + 8, self.allocation_protect);
        maps.write_word(addr + 12, self.partition_id);
        maps.write_dword(addr + 14, self.region_size);
        maps.write_dword(addr + 18, self.state);
        maps.write_dword(addr + 22, self.protect);
        maps.write_dword(addr + 26, self.typ);
    }

    pub fn print(&self) {
        log::trace!("{:#x?}", self);
    }
}
