use std::fmt;

use serde::de::{Error, SeqAccess, Visitor};
use serde::ser::SerializeTuple;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

// Helper constants and functions
const TLB_GLOBAL_PAGE: u32 = 0x80000000; // Example value
const TLB_SIZE: usize = 2048;
pub fn LPF_OF(addr: u64) -> u64 {
    // Implementation of LPF_OF macro/function
    addr & 0xfffffffffffff000
}

pub const INVALID_TLB_ENTRY: u64 = 0xffffffffffffffff;

#[derive(Clone, Serialize, Deserialize)]
pub struct TLBEntry {
    pub lpf: u64,
    // Store the key (usize) to the Mem64 of the slab
    pub mem64: usize,
}

impl TLBEntry {
    fn new() -> Self {
        Self {
            lpf: INVALID_TLB_ENTRY,
            mem64: 0,
        }
    }

    fn valid(&self) -> bool {
        self.lpf != INVALID_TLB_ENTRY
    }

    pub fn invalidate(&mut self) {
        self.lpf = INVALID_TLB_ENTRY;
    }

    #[inline]
    pub fn set_mem(&mut self, lpf: u64, mem64: usize) {
        self.lpf = lpf;
        self.mem64 = mem64;
    }

    #[inline]
    pub fn get_mem(&self) -> usize {
        self.mem64
    }
}
#[derive(Clone, Serialize, Deserialize)]
pub struct TLB {
    #[serde(
        serialize_with = "serialize_entries",
        deserialize_with = "deserialize_entries"
    )]
    entry: Box<[TLBEntry]>,
}

// Published snapshots encode the TLB as a fixed tuple. Decode straight into
// heap storage so serde never puts the 2,048-entry table on the caller's stack.
fn serialize_entries<S: Serializer>(
    entries: &[TLBEntry],
    serializer: S,
) -> Result<S::Ok, S::Error> {
    let mut tuple = serializer.serialize_tuple(TLB_SIZE)?;
    for entry in entries {
        tuple.serialize_element(entry)?;
    }
    tuple.end()
}

fn deserialize_entries<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<Box<[TLBEntry]>, D::Error> {
    struct Entries;

    impl<'de> Visitor<'de> for Entries {
        type Value = Box<[TLBEntry]>;

        fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(formatter, "{TLB_SIZE} TLB entries")
        }

        fn visit_seq<A: SeqAccess<'de>>(self, mut sequence: A) -> Result<Self::Value, A::Error> {
            let mut entries = Vec::with_capacity(TLB_SIZE);
            for index in 0..TLB_SIZE {
                entries.push(
                    sequence
                        .next_element()?
                        .ok_or_else(|| A::Error::invalid_length(index, &self))?,
                );
            }
            Ok(entries.into_boxed_slice())
        }
    }

    deserializer.deserialize_tuple(TLB_SIZE, Entries)
}

impl Default for TLB {
    fn default() -> Self {
        Self::new()
    }
}

impl TLB {
    pub fn new() -> Self {
        Self {
            entry: vec![TLBEntry::new(); TLB_SIZE].into_boxed_slice(),
        }
    }

    // call flush everytime the memory is free
    // This is an extremely overhead but most of the time malware don't use VirtualFree a lot
    #[inline]
    pub fn flush(&mut self) {
        for entry in &mut self.entry {
            entry.invalidate();
        }
    }

    #[inline]
    pub fn get_index_of(&self, lpf: u64, len: u64) -> usize {
        const TLB_MASK: u32 = ((TLB_SIZE - 1) << 12) as u32;
        (((lpf + len) & (TLB_MASK as u64)) >> 12) as usize
    }

    #[inline]
    pub fn get_entry_of(&self, addr: u64, len: u64) -> &TLBEntry {
        let lpf = LPF_OF(addr);
        let idx = self.get_index_of(lpf, len);
        &self.entry[idx]
    }

    #[inline]
    pub fn get_entry_of_mut(&mut self, addr: u64, len: u64) -> &mut TLBEntry {
        let lpf = LPF_OF(addr);
        let idx = self.get_index_of(lpf, len);
        &mut self.entry[idx]
    }
    pub fn invlpg(&mut self, laddr: u64) {
        let tlb_entry = self.get_entry_of_mut(laddr, 0);
        if LPF_OF(tlb_entry.lpf) == LPF_OF(laddr) {
            tlb_entry.invalidate();
        }
    }
}
