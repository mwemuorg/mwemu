use crate::maps::mem64::Mem64;

/// Windows `_HEAP` structure model for x64.
///
/// Covers both the classic NT Heap layout (Win7–Win10) and the revised
/// layout observed in Win11 build 26100+. Fields whose offset moved
/// between versions are written at **both** positions by [`save()`] so
/// binaries targeting either version find valid data.
///
/// See `docs/ANTI_EMU_HEAP.md` for the analysis that drove this layout.
pub struct Heap64 {
    // --- Segment header (all versions) ---
    /// +0x010 SegmentSignature — must be 0xDDEEDDEE.
    pub segment_signature: u32,
    /// +0x014 SegmentFlags (labeled "Flags" in some Win11 builds).
    pub segment_flags: u32,

    // --- Self-referential pointers (anti-emu validation) ---
    /// +0x028 Heap self-pointer — packers dereference the handle.
    pub heap_self_ptr: u64,
    /// +0x030 BaseAddress of the heap arena.
    pub base_address: u64,
    /// +0x038 NumberOfPages committed.
    pub number_of_pages: u32,
    /// +0x040 FirstEntry — first allocatable HEAP_ENTRY.
    pub first_entry: u64,
    /// +0x048 LastValidEntry — past-the-end sentinel.
    pub last_valid_entry: u64,

    // --- Anti-debug fields (NT Heap offsets) ---
    /// +0x070 Flags — 0 = normal non-debug heap.
    pub flags: u32,
    /// +0x074 ForceFlags — non-zero reveals a debugger.
    pub force_flags: u32,
    /// +0x07C EncodeFlagMask — 0x0010_0000 when HEAP_ENTRY encoding is on.
    pub encode_flag_mask: u32,
    /// +0x080 Encoding — HEAP_ENTRY XOR key (16 bytes).
    pub encoding: [u8; 16],
    /// +0x094 VirtualMemoryThreshold (NT Heap, DWORD).
    pub virtual_memory_threshold: u32,
    /// +0x098 Signature — must be 0xEEFFEEFF.
    pub signature: u32,

    // --- Capacity (read by some anti-emu) ---
    /// +0x0C0 TotalFreeSize.
    pub total_free_size: u64,
    /// +0x0C8 MaximumAllocationSize.
    pub maximum_allocation_size: u64,
    /// +0x0D0 ProcessHeapsListIndex (1 = process heap).
    pub process_heaps_list_index: u16,

    // --- Structural lists (dual-written at Win10 and Win11 offsets) ---
    /// BlocksIndex region at +0x2C0 (Win11, ~0x20 bytes).
    pub blocks_index: [u8; 0x20],
    /// FreeLists intra-heap pointer (Win11 at +0x3D8).
    pub free_lists_ptr: u64,
    /// LockVariable intra-heap pointer (Win11 at +0x480).
    pub lock_variable_ptr: u64,

    // --- LFH / FrontEnd ---
    /// +0x1A0 FrontEndHeap pointer.
    pub front_end_heap: u64,
    /// +0x1AC FrontEndHeapType — 0 = backend-only, 2 = LFH active.
    pub front_end_heap_type: u8,
}

impl Heap64 {
    /// Minimum bytes reserved for the `_HEAP` header. The allocator must
    /// not hand out addresses within this range.
    pub const HEADER_SIZE: u64 = 0x800;

    pub fn new(base_addr: u64, arena_size: u64) -> Self {
        let pages = (arena_size / 0x1000) as u32;
        let mut h = Self {
            segment_signature: 0xDDEE_DDEE,
            segment_flags: 0,

            heap_self_ptr: base_addr,
            base_address: base_addr,
            number_of_pages: pages,
            first_entry: base_addr + Self::HEADER_SIZE,
            last_valid_entry: base_addr + arena_size,

            flags: 0,
            force_flags: 0,
            encode_flag_mask: 0x0010_0000,
            encoding: [
                0xA3, 0x7B, 0x4F, 0x1D, 0xE8, 0x56, 0x92, 0xC0, 0x3E, 0xD7, 0x85, 0x6A, 0xF1, 0x2C,
                0x49, 0xB8,
            ],
            virtual_memory_threshold: 0xFE00,
            signature: 0xEEFF_EEFF,

            total_free_size: arena_size.saturating_sub(Self::HEADER_SIZE),
            maximum_allocation_size: 0x7FFF_EFFF_FFFE,
            process_heaps_list_index: 1,

            blocks_index: [0; 0x20],
            free_lists_ptr: base_addr + 0x400,
            lock_variable_ptr: base_addr + 0x500,

            front_end_heap: base_addr + 0x600,
            front_end_heap_type: 2, // LFH active (default on modern Windows)
        };
        // BlocksIndex bucket-count hint (avoids worst-case crashes).
        h.blocks_index[0] = 0x80;
        h
    }

    /// Write every field to the memory region at its documented offset.
    /// Fields that moved between Win10 and Win11 are dual-written; every
    /// `LIST_ENTRY` written here is self-referential (empty list sentinel).
    pub fn save(&self, mem: &mut Mem64) {
        let b = mem.get_base();

        // --- Segment header ---
        mem.write_dword(b + 0x010, self.segment_signature);
        mem.write_dword(b + 0x014, self.segment_flags);

        // --- Self-pointers ---
        mem.write_qword(b + 0x028, self.heap_self_ptr);
        mem.write_qword(b + 0x030, self.base_address);
        mem.write_dword(b + 0x038, self.number_of_pages);
        mem.write_qword(b + 0x040, self.first_entry);
        mem.write_qword(b + 0x048, self.last_valid_entry);

        // --- Anti-debug (NT Heap offsets) ---
        mem.write_dword(b + 0x070, self.flags);
        mem.write_dword(b + 0x074, self.force_flags);
        mem.write_dword(b + 0x07C, self.encode_flag_mask);
        for (i, &v) in self.encoding.iter().enumerate() {
            mem.write_byte(b + 0x080 + i as u64, v);
        }
        mem.write_dword(b + 0x094, self.virtual_memory_threshold);
        mem.write_dword(b + 0x098, self.signature);

        // --- Capacity ---
        mem.write_qword(b + 0x0C0, self.total_free_size);
        mem.write_qword(b + 0x0C8, self.maximum_allocation_size);
        mem.write_word(b + 0x0D0, self.process_heaps_list_index);

        // --- VirtualAllocdBlocks +0x110 (self-ref) ---
        mem.write_qword(b + 0x110, b + 0x110);
        mem.write_qword(b + 0x118, b + 0x110);

        // --- SegmentList (Win10 at +0x120, Win11 at +0x138) ---
        mem.write_qword(b + 0x120, b + 0x120);
        mem.write_qword(b + 0x128, b + 0x120);
        mem.write_qword(b + 0x138, b + 0x138);
        mem.write_qword(b + 0x140, b + 0x138);

        // --- FreeLists (Win10 at +0x158 LIST_ENTRY, Win11 at +0x3D8 ptr) ---
        mem.write_qword(b + 0x158, b + 0x158);
        mem.write_qword(b + 0x160, b + 0x158);
        mem.write_qword(b + 0x3D8, self.free_lists_ptr);

        // --- LockVariable (Win10 at +0x168, Win11 at +0x480) ---
        mem.write_qword(b + 0x168, self.lock_variable_ptr);
        mem.write_qword(b + 0x480, self.lock_variable_ptr);

        // --- FrontEnd / LFH ---
        mem.write_qword(b + 0x1A0, self.front_end_heap);
        mem.write_byte(b + 0x1AC, self.front_end_heap_type);

        // --- BlocksIndex (Win11 at +0x2C0) ---
        for (i, &v) in self.blocks_index.iter().enumerate() {
            mem.write_byte(b + 0x2C0 + i as u64, v);
        }

        // --- VirtualMemoryThreshold (Win11 at +0x384, WORD) ---
        mem.write_word(b + 0x384, self.virtual_memory_threshold as u16);
    }

    pub fn load(base_addr: u64, mem: &Mem64) -> Self {
        let b = mem.get_base();

        let mut blocks_index = [0u8; 0x20];
        for (i, v) in blocks_index.iter_mut().enumerate() {
            *v = mem.read_byte(b + 0x2C0 + i as u64);
        }
        let mut encoding = [0u8; 16];
        for (i, v) in encoding.iter_mut().enumerate() {
            *v = mem.read_byte(b + 0x080 + i as u64);
        }

        Self {
            segment_signature: mem.read_dword(b + 0x010),
            segment_flags: mem.read_dword(b + 0x014),
            heap_self_ptr: mem.read_qword(b + 0x028),
            base_address: mem.read_qword(b + 0x030),
            number_of_pages: mem.read_dword(b + 0x038),
            first_entry: mem.read_qword(b + 0x040),
            last_valid_entry: mem.read_qword(b + 0x048),
            flags: mem.read_dword(b + 0x070),
            force_flags: mem.read_dword(b + 0x074),
            encode_flag_mask: mem.read_dword(b + 0x07C),
            encoding,
            virtual_memory_threshold: mem.read_dword(b + 0x094),
            signature: mem.read_dword(b + 0x098),
            total_free_size: mem.read_qword(b + 0x0C0),
            maximum_allocation_size: mem.read_qword(b + 0x0C8),
            process_heaps_list_index: mem.read_word(b + 0x0D0),
            blocks_index,
            free_lists_ptr: mem.read_qword(b + 0x3D8),
            lock_variable_ptr: mem.read_qword(b + 0x480),
            front_end_heap: mem.read_qword(b + 0x1A0),
            front_end_heap_type: mem.read_byte(b + 0x1AC),
        }
    }
}
