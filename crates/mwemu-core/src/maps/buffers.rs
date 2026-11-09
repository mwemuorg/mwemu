use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum BufferKind {
    Crt,
    Aligned,
    Bstr,
    Task,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct BufferAllocation {
    pub base: u64,
    pub size: u64,
    pub alignment: u64,
    pub kind: BufferKind,
    pub arena: bool,
}
