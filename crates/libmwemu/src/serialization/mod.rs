use std::{
    fs::File,
    io::{self, Read as _, Write as _},
};

use serde::de::DeserializeOwned;

use crate::emu::Emu;
use crate::serialization::emu::SerializableEmu;
use crate::serialization::legacy_maps::{LegacySerializableMaps, RangeMem64};
use crate::serialization::native::{
    NativeSnapshotUnstable, NativeSnapshotV3, NativeSnapshotV4, NativeSnapshotV5,
};

pub mod emu;
mod fpu;
mod instant;
mod legacy_maps;
pub mod maps;
mod minidump;
pub(crate) mod native;
mod pe32;
mod pe64;
mod thread_context;

const NATIVE_UNSTABLE_MAGIC: &[u8] = b"MWEMU\0UNSTABLE\0";
const NATIVE_V5_MAGIC: &[u8; 8] = b"MWEMU\0\x05\0";
const NATIVE_V4_MAGIC: &[u8; 8] = b"MWEMU\0\x04\0";
const NATIVE_V3_MAGIC: &[u8; 8] = b"MWEMU\0\x03\0";
const NATIVE_V2_MAGIC: &[u8; 8] = b"MWEMU\0\x02\0";

pub struct Serialization;

// Decode only the selected layout. Each snapshot contains a large emulator state.
fn decode_native<T: DeserializeOwned + Into<Emu>>(payload: &[u8]) -> io::Result<Emu> {
    bitcode::deserialize::<T>(payload)
        .map(Into::into)
        .map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error.to_string()))
}

impl Serialization {
    pub fn serialize(emu: &Emu) -> Vec<u8> {
        let serialized = NativeSnapshotUnstable::from(emu);
        let mut data = NATIVE_UNSTABLE_MAGIC.to_vec();
        data.extend(bitcode::serialize(&serialized).unwrap());
        data
    }

    pub fn deserialize(data: &[u8]) -> Emu {
        Self::try_deserialize(data).expect("cannot decode native snapshot")
    }

    pub fn try_deserialize(data: &[u8]) -> io::Result<Emu> {
        if let Some(payload) = data.strip_prefix(NATIVE_UNSTABLE_MAGIC) {
            return decode_native::<NativeSnapshotUnstable>(payload).map_err(|error| {
                io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("invalid UNSTABLE snapshot (use the build that wrote it): {error}"),
                )
            });
        }
        let mut emu = if let Some(payload) = data.strip_prefix(NATIVE_V5_MAGIC) {
            decode_native::<NativeSnapshotV5>(payload)?
        } else if let Some(payload) = data.strip_prefix(NATIVE_V4_MAGIC) {
            decode_native::<NativeSnapshotV4>(payload)?
        } else if let Some(payload) = data.strip_prefix(NATIVE_V3_MAGIC) {
            decode_native::<NativeSnapshotV3>(payload)?
        } else if let Some(payload) = data.strip_prefix(NATIVE_V2_MAGIC) {
            decode_native::<SerializableEmu<LegacySerializableMaps<RangeMem64>>>(payload)?
        } else {
            if data.starts_with(b"MWEMU\0") {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "unsupported native snapshot version (use the build that wrote it)",
                ));
            }
            decode_native::<SerializableEmu<LegacySerializableMaps>>(data)?
        };
        emu.restore_legacy_windows_slots();
        Ok(emu)
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

        let data = Self::serialize(emu);
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
        Self::try_load(filename).expect("cannot load snapshot")
    }

    pub fn try_load(filename: &str) -> io::Result<Emu> {
        let mut file = File::open(filename)?;
        let mut signature = [0u8; 4];
        let count = file.read(&mut signature)?;
        if minidump::is_minidump_path(filename)
            || count == signature.len() && minidump::has_minidump_signature(&signature)
        {
            return minidump::load_from_minidump(filename)
                .map(Into::into)
                .map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error.to_string()));
        }
        let mut bytes = signature[..count].to_vec();
        file.read_to_end(&mut bytes)?;
        Self::try_deserialize(&bytes)
    }
}
