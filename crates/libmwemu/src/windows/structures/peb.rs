use crate::maps::{Maps, mem64::Mem64};

#[derive(Debug)]
pub struct PEB {
    inheritet_addr_space: u8,
    read_img_file_exec_options: u8,
    pub being_debugged: u8,
    speer_bool: u8,
    padding: u32,
    pub image_base_addr: u32,
    pub ldr: u32, // ptr to PEB_LDR_DATA  +0x0c
    process_parameters: u32,
    reserved4: [u32; 3],
    alt_thunk_list_ptr: u32,
    reserved5: u32,
    reserved6: u32,
    reserved7: u32,
    reserved8: u32,
    alt_thunk_list_ptr_32: u32, // +52 + 45*4 + 96
    reserved9: [u32; 45],
    reserved10: [u8; 96],
    post_process_init_routine: u32,
    reserved11: [u32; 128],
    reserved12: u32,
    session_id: u32,
}

impl PEB {
    pub fn size() -> usize {
        800 // TODO: std::mem::size_of_val
    }

    pub fn new(image_base_addr: u32, ldr: u32, process_parameters: u32) -> PEB {
        PEB {
            inheritet_addr_space: 0,
            read_img_file_exec_options: 0,
            being_debugged: 0,
            speer_bool: 0,
            padding: 0,
            image_base_addr,
            ldr,
            process_parameters,
            reserved4: [0; 3],
            alt_thunk_list_ptr: 0,
            reserved5: 0,
            reserved6: 0,
            reserved7: 0,
            reserved8: 0,
            alt_thunk_list_ptr_32: 0,
            reserved9: [0; 45],
            reserved10: [0; 96],
            post_process_init_routine: 0,
            reserved11: [0; 128],
            reserved12: 0,
            session_id: 0,
        }
    }

    pub fn load(addr: u64, maps: &Maps) -> PEB {
        PEB {
            inheritet_addr_space: maps.read_byte(addr).unwrap(),
            read_img_file_exec_options: maps.read_byte(addr + 1).unwrap(),
            being_debugged: maps.read_byte(addr + 2).unwrap(),
            speer_bool: maps.read_byte(addr + 3).unwrap(),
            padding: maps.read_dword(addr + 4).unwrap(),
            image_base_addr: maps.read_dword(addr + 8).unwrap(),
            ldr: maps.read_dword(addr + 12).unwrap(),
            process_parameters: maps.read_dword(addr + 16).unwrap(),
            reserved4: [
                maps.read_dword(addr + 20).unwrap(),
                maps.read_dword(addr + 24).unwrap(),
                maps.read_dword(addr + 28).unwrap(),
            ],
            alt_thunk_list_ptr: maps.read_dword(addr + 32).unwrap(),
            reserved5: maps.read_dword(addr + 36).unwrap(),
            reserved6: maps.read_dword(addr + 40).unwrap(),
            reserved7: maps.read_dword(addr + 44).unwrap(),
            reserved8: maps.read_dword(addr + 48).unwrap(),
            alt_thunk_list_ptr_32: maps.read_dword(addr + 52).unwrap(),
            reserved9: [0; 45],
            reserved10: [0; 96],
            post_process_init_routine: maps.read_dword(addr + 56).unwrap(),
            reserved11: [0; 128],
            reserved12: maps.read_dword(addr + 60).unwrap(),
            session_id: maps.read_dword(addr + 64).unwrap(),
        }
    }

    pub fn set_image_base(&mut self, image_base: u32) {
        self.image_base_addr = image_base;
    }

    pub fn save(&self, mem: &mut Mem64) {
        let base = mem.get_base();
        mem.write_byte(base, self.inheritet_addr_space);
        mem.write_byte(base + 1, self.read_img_file_exec_options);
        mem.write_byte(base + 2, self.being_debugged);
        mem.write_byte(base + 3, self.speer_bool);
        mem.write_dword(base + 4, self.padding);
        mem.write_dword(base + 8, self.image_base_addr);
        mem.write_dword(base + 12, self.ldr);
        mem.write_dword(base + 16, self.process_parameters);
        mem.write_dword(base + 20, self.reserved4[0]);
        mem.write_dword(base + 24, self.reserved4[1]);
        mem.write_dword(base + 28, self.reserved4[2]);
        mem.write_dword(base + 32, self.alt_thunk_list_ptr);
        mem.write_dword(base + 36, self.reserved5);
        mem.write_dword(base + 40, self.reserved6);
        mem.write_dword(base + 44, self.reserved7);
        mem.write_dword(base + 48, self.reserved8);
        mem.write_dword(base + 52, self.alt_thunk_list_ptr_32);
        mem.write_dword(base + 56, self.post_process_init_routine);
        mem.write_dword(base + 60, self.reserved12);
        mem.write_dword(base + 64, self.session_id);
    }

    pub fn print(&self) {
        log::trace!("{:#x?}", self);
    }
}
