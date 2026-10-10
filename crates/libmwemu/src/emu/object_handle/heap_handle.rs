/// Emulated Win32 heap object backing a handle from `HeapCreate`/`GetProcessHeap`.
///
/// `arena` is an index into `Emu::heap_arenas`; arena 0 is always the
/// process heap (the one `Emu::heap_mut()` returns).
pub struct HeapHandle {
    pub initSZ: usize,
    pub maxSZ: usize,
    pub arena: usize,
    /// The real mapped base address of this heap's arena, handed to the guest
    /// as the heap handle value. On real Windows the heap handle IS the base
    /// address of the HEAP structure; guests (packers especially) validate it
    /// by reading bytes near it.
    pub base_addr: u64,
}

impl HeapHandle {
    pub fn new(_opt: u32, initSZ: usize, maxSZ: usize, arena: usize) -> Self {
        Self {
            initSZ,
            maxSZ,
            arena,
            base_addr: 0,
        }
    }
}
