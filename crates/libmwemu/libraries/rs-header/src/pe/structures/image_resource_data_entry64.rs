pub struct ImageResourceDataEntry64 {
    pub offset_to_data: u64,
    pub size: u64,
    pub code_page: u64,
    pub reserved: u64,
}

impl ImageResourceDataEntry64 {
    pub fn new() -> ImageResourceDataEntry64 {
        ImageResourceDataEntry64 {
            offset_to_data: 0,
            size: 0,
            code_page: 0,
            reserved: 0,
        }
    }
}
