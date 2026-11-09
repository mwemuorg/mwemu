use std::error::Error;
use std::io;
use std::path::Path;

use crate::emu::Emu;
use crate::serialization::emu::SerializableEmu;

mod context;
mod reader;
mod writer;

pub fn dump_to_minidump(emu: &Emu, filename: &str) -> io::Result<()> {
    writer::MinidumpWriter::write_to_file(emu, filename)
}

pub fn load_from_minidump(filename: &str) -> Result<SerializableEmu, Box<dyn Error>> {
    reader::MinidumpReader::from_minidump_file(filename)
}

pub fn is_minidump_path(filename: &str) -> bool {
    Path::new(filename)
        .extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| ext.eq_ignore_ascii_case("dmp") || ext.eq_ignore_ascii_case("mdmp"))
        .unwrap_or(false)
}

pub fn has_minidump_signature(data: &[u8]) -> bool {
    data.starts_with(b"MDMP")
}
