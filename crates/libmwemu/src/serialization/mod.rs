use std::{
    fs::File,
    io::{Read as _, Write as _},
};

use crate::emu::Emu;
use crate::serialization::emu::SerializableEmu;

mod emu;
mod fpu;
mod instant;
mod maps;
mod minidump;
mod pe32;
mod pe64;
mod thread_context;

pub struct Serialization;

impl Serialization {
    pub fn serialize(emu: &Emu) -> Vec<u8> {
        let serialized = SerializableEmu::from(emu);
        bitcode::serialize(&serialized).unwrap()
    }

    pub fn deserialize(data: &[u8]) -> Emu {
        let deserialized: SerializableEmu = bitcode::deserialize(data).unwrap();
        deserialized.into()
    }

    /// Directory where the per-map `.bin` dumps are written: the parent
    /// directory of `filename`, falling back to "." when it has none.
    fn dump_dir(filename: &str) -> String {
        std::path::Path::new(filename)
            .parent()
            .filter(|p| !p.as_os_str().is_empty())
            .map(|p| p.to_string_lossy().into_owned())
            .unwrap_or_else(|| ".".to_string())
    }

    pub fn dump_to_file(emu: &Emu, filename: &str) {
        let dump_dir = Self::dump_dir(filename);
        std::fs::create_dir_all(&dump_dir).unwrap();

        let serialized = SerializableEmu::from(emu);
        let data = bitcode::serialize(&serialized).unwrap();
        let mut file = File::create(filename).unwrap();
        file.write_all(&data).unwrap();
        file.flush().unwrap();
        drop(file);

        // for binary analysis
        emu.maps.save_all(dump_dir);
    }

    pub fn load_from_file(filename: &str) -> Emu {
        let data = std::fs::read(filename).unwrap();
        Self::deserialize(&data)
    }

    pub fn dump_to_minidump(emu: &Emu, filename: &str) -> std::io::Result<()> {
        let dump_dir = Self::dump_dir(filename);
        std::fs::create_dir_all(&dump_dir)?;
        minidump::dump_to_minidump(emu, filename)?;

        // for binary analysis
        emu.maps.save_all(dump_dir);
        Ok(())
    }

    pub fn load_from_minidump(filename: &str) -> Emu {
        let serializable = minidump::load_from_minidump(filename).unwrap();
        serializable.into()
    }

    pub fn dump(emu: &Emu, filename: &str) {
        if minidump::is_minidump_path(filename) {
            Self::dump_to_minidump(emu, filename).unwrap();
        } else {
            Self::dump_to_file(emu, filename);
        }
    }

    pub fn load(filename: &str) -> Emu {
        if minidump::is_minidump_path(filename) {
            return Self::load_from_minidump(filename);
        }

        let mut file = File::open(filename).unwrap();
        let mut signature = [0u8; 4];
        let bytes_read = file.read(&mut signature).unwrap();
        if bytes_read == signature.len() && minidump::has_minidump_signature(&signature) {
            Self::load_from_minidump(filename)
        } else {
            Self::load_from_file(filename)
        }
    }
}
