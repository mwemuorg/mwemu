pub struct ImageResourceDataEntry32 {
    pub offset_to_data: u32,
    pub size: u32,
    pub code_page: u32,
    pub reserved: u32,
}

impl ImageResourceDataEntry32 {
    pub fn new() -> ImageResourceDataEntry32 {
        ImageResourceDataEntry32 {
            offset_to_data: 0,
            size: 0,
            code_page: 0,
            reserved: 0,
        }
    }
}
