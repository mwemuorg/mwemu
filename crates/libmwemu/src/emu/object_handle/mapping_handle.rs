pub struct MappingHandle {
    pub name: String,
    pub file_handle: Option<u32>, // Handle to the file being mapped (None if paging file)
    pub protect: u32,
    pub max_size: u64,
}

impl MappingHandle {
    pub fn new(name: String, file_handle: Option<u32>, protect: u32, max_size: u64) -> Self {
        Self {
            name,
            file_handle,
            protect,
            max_size,
        }
    }
}
