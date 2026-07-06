use crate::maps::Maps;

pub struct ActCtxSectionKeyedData32 {
    pub cb_size: u32,
    pub ul_data_format_version: u32,
    pub lp_data: u32,
    pub ul_length: u32,
    pub lp_section_global_data: u32,
    pub ul_section_global_data_length: u32,
    pub lp_section_base: u32,
    pub ul_section_total_length: u32,
    pub h_act_ctx: u32,
    pub ul_assembly_roster_index: u32,
    pub ul_flags: u32,
    pub assembly_metadata: [u8; 64],
}

impl Default for ActCtxSectionKeyedData32 {
    fn default() -> Self {
        Self::new()
    }
}

impl ActCtxSectionKeyedData32 {
    pub fn new() -> ActCtxSectionKeyedData32 {
        ActCtxSectionKeyedData32 {
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
        maps.write_dword(addr + 8, self.lp_data);
        maps.write_dword(addr + 12, self.ul_length);
        maps.write_dword(addr + 16, self.lp_section_global_data);
        maps.write_dword(addr + 20, self.ul_section_global_data_length);
        maps.write_dword(addr + 24, self.lp_section_base);
        maps.write_dword(addr + 28, self.ul_section_total_length);
        maps.write_dword(addr + 32, self.h_act_ctx);
        maps.write_dword(addr + 36, self.ul_assembly_roster_index);
        maps.write_dword(addr + 40, self.ul_flags);
        maps.write_bytes(addr + 44, &self.assembly_metadata);
    }
}
