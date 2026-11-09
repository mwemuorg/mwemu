use crate::maps::Maps;

pub struct ActCtxSectionKeyedData64 {
    pub cb_size: u32,
    pub ul_data_format_version: u32,
    pub lp_data: u64,
    pub ul_length: u32,
    pub lp_section_global_data: u64,
    pub ul_section_global_data_length: u32,
    pub lp_section_base: u64,
    pub ul_section_total_length: u32,
    pub h_act_ctx: u64,
    pub ul_assembly_roster_index: u32,
    pub ul_flags: u32,
    pub assembly_metadata: [u8; 64],
}

impl Default for ActCtxSectionKeyedData64 {
    fn default() -> Self {
        Self::new()
    }
}

impl ActCtxSectionKeyedData64 {
    pub fn new() -> ActCtxSectionKeyedData64 {
        ActCtxSectionKeyedData64 {
            cb_size: 0,
            ul_data_format_version: 0,
            lp_data: 0,
            ul_length: 0,
            lp_section_global_data: 0,
            ul_section_global_data_length: 0,
            lp_section_base: 0,
            ul_section_total_length: 0,
            h_act_ctx: 0,
            ul_assembly_roster_index: 0,
            ul_flags: 0,
            assembly_metadata: [0; 64],
        }
    }

    pub fn save(&self, addr: u64, maps: &mut Maps) {
        maps.write_dword(addr, self.cb_size);
        maps.write_dword(addr + 4, self.ul_data_format_version);
        maps.write_qword(addr + 8, self.lp_data);
        maps.write_dword(addr + 16, self.ul_length);
        maps.write_qword(addr + 24, self.lp_section_global_data);
        maps.write_dword(addr + 32, self.ul_section_global_data_length);
        maps.write_qword(addr + 40, self.lp_section_base);
        maps.write_dword(addr + 48, self.ul_section_total_length);
        maps.write_qword(addr + 56, self.h_act_ctx);
        maps.write_dword(addr + 64, self.ul_assembly_roster_index);
        maps.write_dword(addr + 68, self.ul_flags);
        maps.write_bytes(addr + 72, &self.assembly_metadata);
    }
}
