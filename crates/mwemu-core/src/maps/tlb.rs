use serde::{Deserialize, Serialize};

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
    #[serde(with = "serde_arrays")]
    entry: [TLBEntry; TLB_SIZE],
}

impl Default for TLB {
    fn default() -> Self {
        Self::new()
    }
}

impl TLB {
    pub fn new() -> Self {
        Self {
            entry: std::array::from_fn(|_| TLBEntry::new()),
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
