use crate::maps::mem64::Mem64;

// Partial _HEAP structure for Windows 10/11 x64.
// Only the fields observed being read by ntdll!RtlAllocateHeap and
// anti-emulation checks (Enigma, Themida) are modeled here.
// See docs/ANTI_EMU_HEAP.md for the full analysis.

pub struct Heap64 {
    // +0x000
    pub segment_signature_pad: [u8; 0x10],
    // +0x010
    pub segment_signature: u32,
    // +0x014
    pub flags: u32,
    // +0x018  gap until +0x138
    pub pad_018: [u8; 0x138 - 0x18],
    // +0x138  SegmentList (LIST_ENTRY: Flink, Blink)
    pub segment_list_flink: u64,
    pub segment_list_blink: u64,
    // +0x148  gap until +0x2C0
    pub pad_148: [u8; 0x2C0 - 0x148],
    // +0x2C0  BlocksIndex region (~0x20 bytes)
    pub blocks_index: [u8; 0x20],
    // +0x2E0  gap until +0x384
    pub pad_2e0: [u8; 0x384 - 0x2E0],
    // +0x384  VirtualMemoryThreshold (WORD)
    pub virtual_memory_threshold: u16,
    // +0x386  gap until +0x3D8
    pub pad_386: [u8; 0x3D8 - 0x386],
    // +0x3D8  FreeLists / BlocksIndex pointer
    pub free_lists_ptr: u64,
    // +0x3E0  gap until +0x480
    pub pad_3e0: [u8; 0x480 - 0x3E0],
    // +0x480  LockVariable pointer
    pub lock_variable_ptr: u64,
}

impl Heap64 {
    pub fn new(base_addr: u64) -> Self {
        let mut h = Heap64 {
            segment_signature_pad: [0; 0x10],
            segment_signature: 0xDDEEDDEE,
            flags: 0,
            pad_018: [0; 0x138 - 0x18],
            segment_list_flink: base_addr + 0x138,
            segment_list_blink: base_addr + 0x138,
            pad_148: [0; 0x2C0 - 0x148],
            blocks_index: [0; 0x20],
            pad_2e0: [0; 0x384 - 0x2E0],
            virtual_memory_threshold: 0xFE00,
            pad_386: [0; 0x3D8 - 0x386],
            free_lists_ptr: base_addr + 0x400,
            pad_3e0: [0; 0x480 - 0x3E0],
            lock_variable_ptr: base_addr + 0x500,
        };
        // BlocksIndex: first WORD is the bucket count hint, rest is zeroed.
        // This avoids the worst crashes but the CFG indirect call into this
        // region is still an open problem (see docs/ANTI_EMU_HEAP.md §5).
        h.blocks_index[0] = 0x80;
        h
    }

    pub fn save(&self, mem: &mut Mem64) {
        let base = mem.get_base();
        mem.write_dword(base + 0x10, self.segment_signature);
        mem.write_dword(base + 0x14, self.flags);
        mem.write_qword(base + 0x138, self.segment_list_flink);
        mem.write_qword(base + 0x140, self.segment_list_blink);
        for (i, &b) in self.blocks_index.iter().enumerate() {
            mem.write_byte(base + 0x2C0 + i as u64, b);
        }
        mem.write_word(base + 0x384, self.virtual_memory_threshold);
        mem.write_qword(base + 0x3D8, self.free_lists_ptr);
        mem.write_qword(base + 0x480, self.lock_variable_ptr);
    }

    pub fn load(base_addr: u64, mem: &Mem64) -> Self {
        let base = mem.get_base();
        let mut blocks_index = [0u8; 0x20];
        for (i, b) in blocks_index.iter_mut().enumerate() {
            *b = mem.read_byte(base + 0x2C0 + i as u64);
        }
        Heap64 {
            segment_signature_pad: [0; 0x10],
            segment_signature: mem.read_dword(base + 0x10),
            flags: mem.read_dword(base + 0x14),
            pad_018: [0; 0x138 - 0x18],
            segment_list_flink: mem.read_qword(base + 0x138),
            segment_list_blink: mem.read_qword(base + 0x140),
            pad_148: [0; 0x2C0 - 0x148],
            blocks_index,
            pad_2e0: [0; 0x384 - 0x2E0],
            virtual_memory_threshold: mem.read_word(base + 0x384),
            pad_386: [0; 0x3D8 - 0x386],
            free_lists_ptr: mem.read_qword(base + 0x3D8),
            pad_3e0: [0; 0x480 - 0x3E0],
            lock_variable_ptr: mem.read_qword(base + 0x480),
        }
    }
}
