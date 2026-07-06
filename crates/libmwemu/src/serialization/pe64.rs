use serde::{Deserialize, Serialize};

use rs_header::pe::pe64::PE64;

/// On-disk form of a loaded PE64. rs-header's parser is borrow-based and does
/// not keep the file bytes, so the raw image is stored here (owned at runtime by
/// `Emu::pe64_raw`) and re-parsed on restore.
#[derive(Serialize, Deserialize)]
pub struct SerializablePE64 {
    pub filename: String,
    pub raw: Vec<u8>,
}

impl From<SerializablePE64> for PE64 {
    fn from(serialized: SerializablePE64) -> Self {
        PE64::parse(&serialized.filename, &serialized.raw)
    }
}
