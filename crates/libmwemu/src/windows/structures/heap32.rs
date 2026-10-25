use crate::maps::mem64::Mem64;

/// Windows `_HEAP` structure model for x86 (32-bit).
///
/// Mirrors `Heap64` with 32-bit pointer widths. Fields whose offset
/// differs between Win10 (NT Heap) and Win11 are dual-written by
/// [`save()`].
///
/// See `docs/ANTI_EMU_HEAP.md`.
pub struct Heap32 {
    // --- Segment header ---
    /// +0x000 HEAP_ENTRY (8 bytes).
    pub entry: [u8; 8],
    /// +0x008 SegmentSignature — must be 0xDDEEDDEE.
    pub segment_signature: u32,
    /// +0x00C SegmentFlags.
    pub segment_flags: u32,

    // --- Self-referential pointers ---
    /// +0x018 Heap self-pointer.
    pub heap_self_ptr: u32,
    /// +0x01C BaseAddress.
    pub base_address: u32,
    /// +0x020 NumberOfPages.
    pub number_of_pages: u32,
    /// +0x024 FirstEntry.
    pub first_entry: u32,
    /// +0x028 LastValidEntry.
    pub last_valid_entry: u32,

    // --- Anti-debug (standard Win10 x86 offsets) ---
    /// +0x040 Flags — 0 = normal non-debug heap.
    pub flags: u32,
    /// +0x044 ForceFlags — non-zero reveals a debugger.
    pub force_flags: u32,
    /// +0x04C EncodeFlagMask — 0x0010_0000 when encoding is active.
    pub encode_flag_mask: u32,
    /// +0x050 Encoding — HEAP_ENTRY XOR key (8 bytes on x86).
    pub encoding: [u8; 8],
    /// +0x05C VirtualMemoryThreshold (Win10 NT Heap, DWORD).
    pub virtual_memory_threshold: u32,
    /// +0x060 Signature — must be 0xEEFFEEFF.
    pub signature: u32,

    // --- Capacity ---
    /// +0x074 TotalFreeSize.
    pub total_free_size: u32,
    /// +0x078 MaximumAllocationSize.
    pub maximum_allocation_size: u32,
    /// +0x07C ProcessHeapsListIndex (1 = process heap).
    pub process_heaps_list_index: u16,

    // --- Structural lists ---
    /// SegmentList.Flink — written at +0x0A4.
    pub segment_list_flink: u32,
    /// SegmentList.Blink — written at +0x0A8.
    pub segment_list_blink: u32,
    /// BlocksIndex region at +0x150 (Win11, ~0x10 bytes).
    pub blocks_index: [u8; 0x10],
    /// FreeLists.Flink — written at +0x0C0 (Win10) and +0x1AC (Win11).
    pub free_lists_flink: u32,
    /// FreeLists.Blink.
    pub free_lists_blink: u32,
    /// LockVariable — written at +0x0C8 (Win10) and +0x24C (Win11).
    pub lock_variable_ptr: u32,

    // --- LFH ---
    /// +0x0F0 FrontEndHeap pointer.
    pub front_end_heap: u32,
    /// +0x0F8 FrontEndHeapType — 0 = backend-only, 2 = LFH active.
    pub front_end_heap_type: u8,
}

impl Heap32 {
    /// Minimum header reservation.
    pub const HEADER_SIZE: u32 = 0x400;

    pub fn new(base_addr: u32, arena_size: u32) -> Self {
        let pages = arena_size / 0x1000;
        let mut h = Self {
            entry: [0; 8],
            segment_signature: 0xDDEE_DDEE,
            segment_flags: 0,

            heap_self_ptr: base_addr,
            base_address: base_addr,
            number_of_pages: pages,
            first_entry: base_addr.wrapping_add(Self::HEADER_SIZE),
            last_valid_entry: base_addr.wrapping_add(arena_size),

            flags: 0,
            force_flags: 0,
            encode_flag_mask: 0x0010_0000,
            encoding: [0xA3, 0x7B, 0x4F, 0x1D, 0xE8, 0x56, 0x92, 0xC0],
            virtual_memory_threshold: 0xFE00,
            signature: 0xEEFF_EEFF,

            total_free_size: arena_size.saturating_sub(Self::HEADER_SIZE),
            maximum_allocation_size: 0x7FFF_EFFE,
            process_heaps_list_index: 1,

            segment_list_flink: base_addr.wrapping_add(0x0A4),
            segment_list_blink: base_addr.wrapping_add(0x0A4),
            blocks_index: [0; 0x10],
            free_lists_flink: base_addr.wrapping_add(0x1AC),
            free_lists_blink: base_addr.wrapping_add(0x1AC),
            lock_variable_ptr: base_addr.wrapping_add(0x300),

            front_end_heap: base_addr.wrapping_add(0x200),
            front_end_heap_type: 2,
        };
        h.blocks_index[0] = 0x80;
        h
    }

    /// Write every field at its documented offset, dual-writing where
    /// Win10 and Win11 offsets differ.
    pub fn save(&self, mem: &mut Mem64) {
        let b = mem.get_base();

        // --- Segment header ---
        mem.write_bytes(b, &self.entry);
        mem.write_dword(b + 0x008, self.segment_signature);
        mem.write_dword(b + 0x00C, self.segment_flags);

        // --- Self-pointers ---
        mem.write_dword(b + 0x018, self.heap_self_ptr);
        mem.write_dword(b + 0x01C, self.base_address);
        mem.write_dword(b + 0x020, self.number_of_pages);
        mem.write_dword(b + 0x024, self.first_entry);
        mem.write_dword(b + 0x028, self.last_valid_entry);

        // --- Anti-debug ---
        mem.write_dword(b + 0x040, self.flags);
        mem.write_dword(b + 0x044, self.force_flags);
        mem.write_dword(b + 0x04C, self.encode_flag_mask);
        for (i, &v) in self.encoding.iter().enumerate() {
            mem.write_byte(b + 0x050 + i as u64, v);
        }
        mem.write_dword(b + 0x05C, self.virtual_memory_threshold);
        mem.write_dword(b + 0x060, self.signature);

        // --- Capacity ---
        mem.write_dword(b + 0x074, self.total_free_size);
        mem.write_dword(b + 0x078, self.maximum_allocation_size);
        mem.write_word(b + 0x07C, self.process_heaps_list_index);

        // --- SegmentList +0x0A4 (self-ref) ---
        mem.write_dword(b + 0x0A4, self.segment_list_flink);
        mem.write_dword(b + 0x0A8, self.segment_list_blink);

        // --- FreeLists (Win10 at +0x0C0 self-ref, Win11 at +0x1AC) ---
        let b32 = b as u32;
        mem.write_dword(b + 0x0C0, b32.wrapping_add(0x0C0));
        mem.write_dword(b + 0x0C4, b32.wrapping_add(0x0C0));
        mem.write_dword(b + 0x1AC, self.free_lists_flink);
        mem.write_dword(b + 0x1B0, self.free_lists_blink);

        // --- LockVariable (Win10 at +0x0C8, Win11 at +0x24C) ---
        mem.write_dword(b + 0x0C8, self.lock_variable_ptr);
        mem.write_dword(b + 0x24C, self.lock_variable_ptr);

        // --- FrontEnd / LFH ---
        mem.write_dword(b + 0x0F0, self.front_end_heap);
        mem.write_byte(b + 0x0F8, self.front_end_heap_type);

        // --- BlocksIndex (Win11 at +0x150) ---
        for (i, &v) in self.blocks_index.iter().enumerate() {
            mem.write_byte(b + 0x150 + i as u64, v);
        }
    }

    pub fn load(base_addr: u32, mem: &Mem64) -> Self {
        let b = mem.get_base();

        let mut entry = [0u8; 8];
        for (i, v) in entry.iter_mut().enumerate() {
            *v = mem.read_byte(b + i as u64);
        }
        let mut blocks_index = [0u8; 0x10];
        for (i, v) in blocks_index.iter_mut().enumerate() {
            *v = mem.read_byte(b + 0x150 + i as u64);
        }
        let mut encoding = [0u8; 8];
        for (i, v) in encoding.iter_mut().enumerate() {
            *v = mem.read_byte(b + 0x050 + i as u64);
        }

        Self {
            entry,
            segment_signature: mem.read_dword(b + 0x008),
            segment_flags: mem.read_dword(b + 0x00C),
            heap_self_ptr: mem.read_dword(b + 0x018),
            base_address: mem.read_dword(b + 0x01C),
            number_of_pages: mem.read_dword(b + 0x020),
            first_entry: mem.read_dword(b + 0x024),
            last_valid_entry: mem.read_dword(b + 0x028),
            flags: mem.read_dword(b + 0x040),
            force_flags: mem.read_dword(b + 0x044),
            encode_flag_mask: mem.read_dword(b + 0x04C),
            encoding,
            virtual_memory_threshold: mem.read_dword(b + 0x05C),
            signature: mem.read_dword(b + 0x060),
            total_free_size: mem.read_dword(b + 0x074),
            maximum_allocation_size: mem.read_dword(b + 0x078),
            process_heaps_list_index: mem.read_word(b + 0x07C),
            segment_list_flink: mem.read_dword(b + 0x0A4),
            segment_list_blink: mem.read_dword(b + 0x0A8),
            blocks_index,
            free_lists_flink: mem.read_dword(b + 0x1AC),
            free_lists_blink: mem.read_dword(b + 0x1B0),
            lock_variable_ptr: mem.read_dword(b + 0x24C),
            front_end_heap: mem.read_dword(b + 0x0F0),
            front_end_heap_type: mem.read_byte(b + 0x0F8),
        }
    }
}
