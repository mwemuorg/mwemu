//! What the PE loader returns. The loader itself is in `mwemu-windows`; the
//! types are here because `Emu::load_pe32()`, `load_pe64()` and
//! `map_dll_pe64()` return them.

use rs_header::pe::pe64::PE64;

/// Location of a successfully loaded PE image in guest memory.
#[derive(Debug, Clone, Copy)]
pub struct PeLoadInfo {
    pub base: u64,
    pub header_offset: u32,
}

/// A relocated PE64 image whose imports can be bound after mapping dependencies.
pub struct MappedPe64 {
    pub base: u64,
    pub image: PE64,
    pub raw: Vec<u8>,
}
