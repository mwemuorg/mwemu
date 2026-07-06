use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryOperation {
    /// Position/step counter in the emulation
    pub pos: u64,
    /// Instruction pointer at time of operation
    pub rip: u64,
    /// Type of memory operation ("read" or "write")
    pub op: String,
    /// Size of the operation in bits (8, 16, 32, 64)
    pub bits: u32,
    /// Memory address being accessed
    pub address: u64,
    /// Old value before the operation
    pub old_value: u64,
    /// New value after the operation
    pub new_value: u64,
    /// Name of the memory region being accessed
    pub name: String,
}
