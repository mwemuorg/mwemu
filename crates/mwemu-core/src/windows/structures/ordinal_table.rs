#[derive(Debug)]
pub struct OrdinalTable {
    pub func_name: String,
    pub ordinal_tbl_rva: u64,
    pub ordinal_tbl: u64,
    pub ordinal: u64,
    pub func_addr_tbl_rva: u64,
    pub func_addr_tbl: u64,
    pub func_rva: u64,
    pub func_va: u64,
}

impl Default for OrdinalTable {
    fn default() -> Self {
        Self::new()
    }
}

impl OrdinalTable {
    pub fn new() -> OrdinalTable {
        OrdinalTable {
            func_name: String::new(),
            ordinal_tbl_rva: 0,
            ordinal_tbl: 0,
            ordinal: 0,
            func_addr_tbl_rva: 0,
            func_addr_tbl: 0,
            func_rva: 0,
            func_va: 0,
        }
    }
}
