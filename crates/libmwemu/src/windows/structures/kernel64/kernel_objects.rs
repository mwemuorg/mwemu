use crate::maps::Maps;

use super::ListEntry64;

#[derive(Debug)]
pub struct MDL {
    pub next: u64,
    pub size: u16,
    pub mdl_flags: u16,
    pub process: u64,
    pub mapped_system_va: u64,
    pub start_va: u64,
    pub byte_count: u32,
    pub byte_offset: u32,
}

impl Default for MDL {
    fn default() -> Self {
        Self::new()
    }
}

impl MDL {
    pub fn size() -> u32 {
        40
    }

    pub fn new() -> MDL {
        MDL {
            next: 0,
            size: 0,
            mdl_flags: 0,
            process: 0,
            mapped_system_va: 0,
            start_va: 0,
            byte_count: 0,
            byte_offset: 0,
        }
    }

    pub fn load(addr: u64, maps: &Maps) -> MDL {
        MDL {
            next: maps.read_qword(addr).unwrap(),
            size: maps.read_word(addr + 8).unwrap(),
            mdl_flags: maps.read_word(addr + 10).unwrap(),
            process: maps.read_qword(addr + 16).unwrap(),
            mapped_system_va: maps.read_qword(addr + 24).unwrap(),
            start_va: maps.read_qword(addr + 32).unwrap(),
            byte_count: maps.read_dword(addr + 40).unwrap(),
            byte_offset: maps.read_dword(addr + 44).unwrap(),
        }
    }

    pub fn save(&self, addr: u64, maps: &mut Maps) {
        maps.write_qword(addr, self.next);
        maps.write_word(addr + 8, self.size);
        maps.write_word(addr + 10, self.mdl_flags);
        maps.write_qword(addr + 16, self.process);
        maps.write_qword(addr + 24, self.mapped_system_va);
        maps.write_qword(addr + 32, self.start_va);
        maps.write_dword(addr + 40, self.byte_count);
        maps.write_dword(addr + 44, self.byte_offset);
    }
}

#[derive(Debug, Copy, Clone)]
pub struct KIDTEntry {
    pub offset_low: u16,
    pub selector: u16,
    pub base: u32,
}

impl Default for KIDTEntry {
    fn default() -> Self {
        Self::new()
    }
}

impl KIDTEntry {
    pub fn size() -> u32 {
        8
    }

    pub fn new() -> KIDTEntry {
        KIDTEntry {
            offset_low: 0,
            selector: 0,
            base: 0,
        }
    }

    pub fn load(addr: u64, maps: &Maps) -> KIDTEntry {
        KIDTEntry {
            offset_low: maps.read_word(addr).unwrap(),
            selector: maps.read_word(addr + 2).unwrap(),
            base: maps.read_dword(addr + 4).unwrap(),
        }
    }

    pub fn save(&self, addr: u64, maps: &mut Maps) {
        maps.write_word(addr, self.offset_low);
        maps.write_word(addr + 2, self.selector);
        maps.write_dword(addr + 4, self.base);
    }
}

#[derive(Debug, Copy, Clone)]
pub struct KIDTEntry64 {
    pub offset_low: u16,
    pub selector: u16,
    pub reserved0: u16,
    pub offset_middle: u16,
    pub offset_high: u32,
    pub reserved1: u32,
}

impl Default for KIDTEntry64 {
    fn default() -> Self {
        Self::new()
    }
}

impl KIDTEntry64 {
    pub fn size() -> u32 {
        16
    }

    pub fn new() -> KIDTEntry64 {
        KIDTEntry64 {
            offset_low: 0,
            selector: 0,
            reserved0: 0,
            offset_middle: 0,
            offset_high: 0,
            reserved1: 0,
        }
    }

    pub fn load(addr: u64, maps: &Maps) -> KIDTEntry64 {
        KIDTEntry64 {
            offset_low: maps.read_word(addr).unwrap(),
            selector: maps.read_word(addr + 2).unwrap(),
            reserved0: maps.read_word(addr + 4).unwrap(),
            offset_middle: maps.read_word(addr + 6).unwrap(),
            offset_high: maps.read_dword(addr + 8).unwrap(),
            reserved1: maps.read_dword(addr + 12).unwrap(),
        }
    }

    pub fn save(&self, addr: u64, maps: &mut Maps) {
        maps.write_word(addr, self.offset_low);
        maps.write_word(addr + 2, self.selector);
        maps.write_word(addr + 4, self.reserved0);
        maps.write_word(addr + 6, self.offset_middle);
        maps.write_dword(addr + 8, self.offset_high);
        maps.write_dword(addr + 12, self.reserved1);
    }
}

#[derive(Debug)]
pub struct EThread {
    pub data: [u8; 4096],
}

impl Default for EThread {
    fn default() -> Self {
        Self::new()
    }
}

impl EThread {
    pub fn size() -> u32 {
        4096
    }

    pub fn new() -> EThread {
        EThread { data: [0; 4096] }
    }

    pub fn load(addr: u64, maps: &Maps) -> EThread {
        let buf = maps.read_bytes_array::<4096>(addr);
        EThread { data: buf }
    }

    pub fn save(&self, addr: u64, maps: &mut Maps) {
        maps.write_bytes(addr, &self.data);
    }
}

#[derive(Debug)]
pub struct EProcess {
    pub data: [u8; 4096],
}

impl Default for EProcess {
    fn default() -> Self {
        Self::new()
    }
}

impl EProcess {
    pub fn size() -> u32 {
        4096
    }

    pub fn new() -> EProcess {
        EProcess { data: [0; 4096] }
    }

    pub fn load(addr: u64, maps: &Maps) -> EProcess {
        let buf = maps.read_bytes_array::<4096>(addr);
        EProcess { data: buf }
    }

    pub fn save(&self, addr: u64, maps: &mut Maps) {
        maps.write_bytes(addr, &self.data);
    }
}

#[derive(Debug)]
pub struct KEVent {
    pub data: [u8; 4096],
}

impl Default for KEVent {
    fn default() -> Self {
        Self::new()
    }
}

impl KEVent {
    pub fn size() -> u32 {
        4096
    }

    pub fn new() -> KEVent {
        KEVent { data: [0; 4096] }
    }

    pub fn load(addr: u64, maps: &Maps) -> KEVent {
        let buf = maps.read_bytes_array::<4096>(addr);
        KEVent { data: buf }
    }

    pub fn save(&self, addr: u64, maps: &mut Maps) {
        maps.write_bytes(addr, &self.data);
    }
}

#[derive(Debug)]
pub struct Mutant {
    pub data: [u8; 4096],
}

impl Default for Mutant {
    fn default() -> Self {
        Self::new()
    }
}

impl Mutant {
    pub fn size() -> u32 {
        4096
    }

    pub fn new() -> Mutant {
        Mutant { data: [0; 4096] }
    }

    pub fn load(addr: u64, maps: &Maps) -> Mutant {
        let buf = maps.read_bytes_array::<4096>(addr);
        Mutant { data: buf }
    }

    pub fn save(&self, addr: u64, maps: &mut Maps) {
        maps.write_bytes(addr, &self.data);
    }
}

#[derive(Debug)]
pub struct IDT {
    pub limit: u16,
    pub descriptors: u64,
}

impl Default for IDT {
    fn default() -> Self {
        Self::new()
    }
}

impl IDT {
    pub fn size() -> u32 {
        10
    }

    pub fn new() -> IDT {
        IDT {
            limit: 0,
            descriptors: 0,
        }
    }

    pub fn load(addr: u64, maps: &Maps) -> IDT {
        IDT {
            limit: maps.read_word(addr).unwrap(),
            descriptors: maps.read_qword(addr + 2).unwrap(),
        }
    }

    pub fn save(&self, addr: u64, maps: &mut Maps) {
        maps.write_word(addr, self.limit);
        maps.write_qword(addr + 2, self.descriptors);
    }
}

#[derive(Debug)]
pub struct KAPC {
    pub type_: u8,
    pub spare_byte0: u8,
    pub size: u8,
    pub spare_byte1: u8,
    pub spare_long0: u32,
    pub thread: u64,
    pub apc_list_entry: ListEntry64,
    pub kernel_routine: u64,
    pub rundown_routine: u64,
    pub normal_routine: u64,
    pub normal_context: u64,
    pub system_argument1: u64,
    pub system_argument2: u64,
    pub apc_state_index: u8,
    pub apc_mode: u8,
    pub inserted: u8,
}

impl Default for KAPC {
    fn default() -> Self {
        Self::new()
    }
}

impl KAPC {
    pub fn size() -> u32 {
        72
    }

    pub fn new() -> KAPC {
        KAPC {
            type_: 0,
            spare_byte0: 0,
            size: 0,
            spare_byte1: 0,
            spare_long0: 0,
            thread: 0,
            apc_list_entry: ListEntry64::new(),
            kernel_routine: 0,
            rundown_routine: 0,
            normal_routine: 0,
            normal_context: 0,
            system_argument1: 0,
            system_argument2: 0,
            apc_state_index: 0,
            apc_mode: 0,
            inserted: 0,
        }
    }
}

#[derive(Debug)]
pub struct ObjectAttributes {
    pub length: u32,
    pub root_directory: u64,
    pub object_name: u64,
    pub attributes: u32,
    pub security_descriptor: u64,
    pub security_quality_of_service: u64,
}

impl Default for ObjectAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl ObjectAttributes {
    pub fn size() -> u32 {
        40
    }

    pub fn new() -> ObjectAttributes {
        ObjectAttributes {
            length: 0,
            root_directory: 0,
            object_name: 0,
            attributes: 0,
            security_descriptor: 0,
            security_quality_of_service: 0,
        }
    }
}

#[derive(Debug)]
pub struct FileStandardInformation {
    pub allocation_size: u64,
    pub end_of_file: u64,
    pub number_of_links: u32,
    pub delete_pending: u8,
    pub directory: u8,
}

impl Default for FileStandardInformation {
    fn default() -> Self {
        Self::new()
    }
}

impl FileStandardInformation {
    pub fn size() -> u32 {
        18
    }

    pub fn new() -> FileStandardInformation {
        FileStandardInformation {
            allocation_size: 0,
            end_of_file: 0,
            number_of_links: 0,
            delete_pending: 0,
            directory: 0,
        }
    }
}

#[derive(Debug)]
pub struct DescriptorTable {
    pub table: [KIDTEntry64; 256],
}

impl Default for DescriptorTable {
    fn default() -> Self {
        Self::new()
    }
}

impl DescriptorTable {
    pub fn size() -> u32 {
        16 * 256
    }

    pub fn new() -> DescriptorTable {
        DescriptorTable {
            table: [KIDTEntry64::new(); 256],
        }
    }
}
