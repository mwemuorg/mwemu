use crate::maps::{Maps, mem64::Mem64};
use crate::windows::constants::process::PEB_BEING_DEBUGGED_OFFSET;

/// Modeled fields of the 32-bit Windows process environment block.
#[derive(Debug, Default)]
pub struct Peb {
    inherited_address_space: u8,
    read_image_file_exec_options: u8,
    pub being_debugged: u8,
    bit_field: u8,
    mutant: u32,
    pub image_base_addr: u32,
    pub ldr: u32,
    process_parameters: u32,
    subsystem_data: u32,
    process_heap: u32,
    fast_peb_lock: u32,
    atl_thunk_slist_ptr: u32,
    reserved5: u32,
    reserved6: u32,
    reserved7: u32,
    reserved8: u32,
    atl_thunk_slist_ptr32: u32,
    post_process_init_routine: u32,
    reserved12: u32,
    session_id: u32,
}

impl Peb {
    pub(crate) const IMAGE_BASE_ADDR_OFFSET: u64 = 0x08;
    pub(crate) const LDR_OFFSET: u64 = 0x0c;
    pub(crate) const PROCESS_HEAP_OFFSET: u64 = 0x18;

    const READ_IMAGE_FILE_EXEC_OPTIONS_OFFSET: u64 = 0x01;
    const BIT_FIELD_OFFSET: u64 = 0x03;
    const MUTANT_OFFSET: u64 = 0x04;
    const PROCESS_PARAMETERS_OFFSET: u64 = 0x10;
    const SUBSYSTEM_DATA_OFFSET: u64 = 0x14;
    const FAST_PEB_LOCK_OFFSET: u64 = 0x1c;
    const ATL_THUNK_SLIST_PTR_OFFSET: u64 = 0x20;
    const RESERVED5_OFFSET: u64 = 0x24;
    const RESERVED6_OFFSET: u64 = 0x28;
    const RESERVED7_OFFSET: u64 = 0x2c;
    const RESERVED8_OFFSET: u64 = 0x30;
    const ATL_THUNK_SLIST_PTR32_OFFSET: u64 = 0x34;
    const POST_PROCESS_INIT_ROUTINE_OFFSET: u64 = 0x14c;
    const RESERVED12_OFFSET: u64 = 0x1d0;
    const SESSION_ID_OFFSET: u64 = 0x1d4;

    pub fn size() -> usize {
        800
    }

    pub fn new(image_base_addr: u32, ldr: u32, process_parameters: u32) -> Self {
        Self {
            image_base_addr,
            ldr,
            process_parameters,
            ..Self::default()
        }
    }

    pub fn load(addr: u64, maps: &Maps) -> Self {
        Self {
            inherited_address_space: maps.read_byte(addr).unwrap_or(0),
            read_image_file_exec_options: maps
                .read_byte(addr + Self::READ_IMAGE_FILE_EXEC_OPTIONS_OFFSET)
                .unwrap_or(0),
            being_debugged: maps
                .read_byte(addr + PEB_BEING_DEBUGGED_OFFSET)
                .unwrap_or(0),
            bit_field: maps.read_byte(addr + Self::BIT_FIELD_OFFSET).unwrap_or(0),
            mutant: maps.read_dword(addr + Self::MUTANT_OFFSET).unwrap_or(0),
            image_base_addr: maps
                .read_dword(addr + Self::IMAGE_BASE_ADDR_OFFSET)
                .unwrap_or(0),
            ldr: maps.read_dword(addr + Self::LDR_OFFSET).unwrap_or(0),
            process_parameters: maps
                .read_dword(addr + Self::PROCESS_PARAMETERS_OFFSET)
                .unwrap_or(0),
            subsystem_data: maps
                .read_dword(addr + Self::SUBSYSTEM_DATA_OFFSET)
                .unwrap_or(0),
            process_heap: maps
                .read_dword(addr + Self::PROCESS_HEAP_OFFSET)
                .unwrap_or(0),
            fast_peb_lock: maps
                .read_dword(addr + Self::FAST_PEB_LOCK_OFFSET)
                .unwrap_or(0),
            atl_thunk_slist_ptr: maps
                .read_dword(addr + Self::ATL_THUNK_SLIST_PTR_OFFSET)
                .unwrap_or(0),
            reserved5: maps.read_dword(addr + Self::RESERVED5_OFFSET).unwrap_or(0),
            reserved6: maps.read_dword(addr + Self::RESERVED6_OFFSET).unwrap_or(0),
            reserved7: maps.read_dword(addr + Self::RESERVED7_OFFSET).unwrap_or(0),
            reserved8: maps.read_dword(addr + Self::RESERVED8_OFFSET).unwrap_or(0),
            atl_thunk_slist_ptr32: maps
                .read_dword(addr + Self::ATL_THUNK_SLIST_PTR32_OFFSET)
                .unwrap_or(0),
            post_process_init_routine: maps
                .read_dword(addr + Self::POST_PROCESS_INIT_ROUTINE_OFFSET)
                .unwrap_or(0),
            reserved12: maps.read_dword(addr + Self::RESERVED12_OFFSET).unwrap_or(0),
            session_id: maps.read_dword(addr + Self::SESSION_ID_OFFSET).unwrap_or(0),
        }
    }

    pub fn set_image_base(&mut self, image_base: u32) {
        self.image_base_addr = image_base;
    }

    pub fn save(&self, mem: &mut Mem64) {
        let base = mem.get_base();
        mem.write_byte(base, self.inherited_address_space);
        mem.write_byte(
            base + Self::READ_IMAGE_FILE_EXEC_OPTIONS_OFFSET,
            self.read_image_file_exec_options,
        );
        mem.write_byte(base + PEB_BEING_DEBUGGED_OFFSET, self.being_debugged);
        mem.write_byte(base + Self::BIT_FIELD_OFFSET, self.bit_field);
        mem.write_dword(base + Self::MUTANT_OFFSET, self.mutant);
        mem.write_dword(base + Self::IMAGE_BASE_ADDR_OFFSET, self.image_base_addr);
        mem.write_dword(base + Self::LDR_OFFSET, self.ldr);
        mem.write_dword(
            base + Self::PROCESS_PARAMETERS_OFFSET,
            self.process_parameters,
        );
        mem.write_dword(base + Self::SUBSYSTEM_DATA_OFFSET, self.subsystem_data);
        mem.write_dword(base + Self::PROCESS_HEAP_OFFSET, self.process_heap);
        mem.write_dword(base + Self::FAST_PEB_LOCK_OFFSET, self.fast_peb_lock);
        mem.write_dword(
            base + Self::ATL_THUNK_SLIST_PTR_OFFSET,
            self.atl_thunk_slist_ptr,
        );
        mem.write_dword(base + Self::RESERVED5_OFFSET, self.reserved5);
        mem.write_dword(base + Self::RESERVED6_OFFSET, self.reserved6);
        mem.write_dword(base + Self::RESERVED7_OFFSET, self.reserved7);
        mem.write_dword(base + Self::RESERVED8_OFFSET, self.reserved8);
        mem.write_dword(
            base + Self::ATL_THUNK_SLIST_PTR32_OFFSET,
            self.atl_thunk_slist_ptr32,
        );
        mem.write_dword(
            base + Self::POST_PROCESS_INIT_ROUTINE_OFFSET,
            self.post_process_init_routine,
        );
        mem.write_dword(base + Self::RESERVED12_OFFSET, self.reserved12);
        mem.write_dword(base + Self::SESSION_ID_OFFSET, self.session_id);
    }

    pub fn print(&self) {
        log::trace!("{:#x?}", self);
    }
}
