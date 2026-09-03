/// One live allocation made from a `HeapHandle`.
pub struct HeapAllocation {
    pub addr: u64,
    pub size: u64,
}

/// Emulated Win32 heap object backing a handle from `HeapCreate`/`GetProcessHeap`.
///
/// `arena` is an index into `Emu::heap_arenas`; arena 0 is always the
/// process heap (the one `Emu::heap_mut()` returns).
pub struct HeapHandle {
    pub opts: u32,
    pub initial_size: u64,
    pub maximum_size: u64,
    pub arena: usize,
    pub allocations: Vec<HeapAllocation>,
}

impl HeapHandle {
    pub fn new(opts: u32, initial_size: u64, maximum_size: u64, arena: usize) -> Self {
        Self {
            opts,
            initial_size,
            maximum_size,
            arena,
            allocations: Vec::new(),
        }
    }

    pub fn record_allocation(&mut self, addr: u64, size: u64) {
        self.allocations.push(HeapAllocation { addr, size });
    }

    /// Remove the first allocation entry whose address matches `addr`.
    /// Returns true if an entry was removed.
    pub fn forget_allocation(&mut self, addr: u64) -> bool {
        if let Some(pos) = self.allocations.iter().position(|a| a.addr == addr) {
            self.allocations.swap_remove(pos);
            true
        } else {
            false
        }
    }
}
