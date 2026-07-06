use serde::{Deserialize, Serialize};

use rs_header::pe::pe32::PE32;

/// On-disk form of a loaded PE32. rs-header's parser is borrow-based and does
/// not keep the file bytes, so the raw image is stored here (owned at runtime by
/// `Emu::pe32_raw`) and re-parsed on restore.
#[derive(Serialize, Deserialize)]
pub struct SerializablePE32 {
    pub filename: String,
    pub raw: Vec<u8>,
}

impl From<SerializablePE32> for PE32 {
    fn from(serialized: SerializablePE32) -> Self {
        PE32::parse(&serialized.filename, &serialized.raw)
    }
}
