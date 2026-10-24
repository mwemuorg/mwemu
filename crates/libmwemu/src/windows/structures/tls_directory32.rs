use crate::maps::Maps;

#[derive(Debug)]
pub struct TlsDirectory32 {
    tls_data_start: u32,
    tls_data_end: u32,
    tls_index: u32, // DS:[FS:[2Ch]] + tls_index *4
    tls_callbacks: u32,
    zero_fill_size: u32, // size = tls_data_end - tls_data_start + zero_fill_size
    characteristic: u32,
}

impl TlsDirectory32 {
    pub fn load(addr: u64, maps: &Maps) -> TlsDirectory32 {
        TlsDirectory32 {
            tls_data_start: maps.read_dword(addr).unwrap_or(0),
            tls_data_end: maps.read_dword(addr + 4).unwrap_or(0),
            tls_index: maps.read_dword(addr + 8).unwrap_or(0),
            tls_callbacks: maps.read_dword(addr + 12).unwrap_or(0),
            zero_fill_size: maps.read_dword(addr + 16).unwrap_or(0),
            characteristic: maps.read_dword(addr + 20).unwrap_or(0),
        }
    }

    pub fn print(&self) {
        log::trace!("{:#x?}", self);
    }
}
