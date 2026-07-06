use crate::maps::Maps;

#[derive(Debug)]
pub struct TlsDirectory64 {
    tls_data_start: u64,
    tls_data_end: u64,
    tls_index: u64, // DS:[FS:[2Ch]] + tls_index *4
    tls_callbacks: u64,
    zero_fill_size: u32, // size = tls_data_end - tls_data_start + zero_fill_size
    characteristic: u32,
}

impl TlsDirectory64 {
    pub fn load(addr: u64, maps: &Maps) -> TlsDirectory64 {
        TlsDirectory64 {
            tls_data_start: maps.read_qword(addr).unwrap(),
            tls_data_end: maps.read_qword(addr + 8).unwrap(),
            tls_index: maps.read_qword(addr + 16).unwrap(),
            tls_callbacks: maps.read_qword(addr + 24).unwrap(),
            zero_fill_size: maps.read_dword(addr + 32).unwrap(),
            characteristic: maps.read_dword(addr + 34).unwrap(),
        }
    }

    pub fn print(&self) {
        log::trace!("{:#x?}", self);
    }
}
