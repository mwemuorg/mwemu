/// Emulated Win32 heap object backing a handle from `HeapCreate`/`GetProcessHeap`.
///
/// `arena` is an index into `Emu::heap_arenas`; arena 0 is always the
/// process heap (the one `Emu::heap_mut()` returns).
pub struct HeapHandle {
    pub initSZ: usize,
    pub maxSZ: usize,
    pub arena: usize,
    /// Real, mapped address handed to the guest for this handle (0 = not
    /// exposed yet). Real Windows heap handles ARE the heap's base address;
    /// guest code (packers especially) sometimes validates a handle by
    /// reading bytes near it, which faults against a bare small integer.
    pub addr: u64,
}

impl HeapHandle {
    pub fn new(_opt: u32, initSZ: usize, maxSZ: usize, arena: usize) -> Self {
        Self {
            initSZ,
            maxSZ,
            arena,
            addr: 0,
        }
    }
}
