pub struct ImageResourceDirectory {
    pub characteristics: u32,
    pub time_date_stamp: u32,
    pub major_version: u16,
    pub minor_version: u16,
    pub number_of_named_entries: u16,
    pub number_of_id_entries: u16,
}

impl ImageResourceDirectory {
    pub fn new() -> ImageResourceDirectory {
        ImageResourceDirectory {
            characteristics: 0,
            time_date_stamp: 0,
            major_version: 0,
            minor_version: 0,
            number_of_named_entries: 0,
            number_of_id_entries: 0,
        }
    }

    pub fn size() -> usize {
        16
    }
}
