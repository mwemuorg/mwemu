//! Bookkeeping for the emulated kernel allocators (slab, vmalloc, pages, pool).
//!
//! This half is pure data: it decides *addresses* and remembers the provenance
//! of every chunk. Creating and poisoning the actual maps lives on the `Emu`
//! side (`kernel::mod`), so the ledger stays testable on its own and has one
//! job: know what is live, what is freed, and who did it.
//!
//! Freed chunks are never unmapped and never recycled. That is deliberate: a
//! real slab hands the memory straight back out, which is precisely what makes
//! a use-after-free hard to see. Keeping the chunk in quarantine — poisoned,
//! still mapped, still attributed — is what lets [`crate::kernel::guard`] turn
//! a stale dereference into a report instead of silent corruption.

use crate::kernel::layout::HEAP_REDZONE;

/// Which allocator handed a chunk out. Determines the poison pattern and how
/// the chunk is described in a report.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Region {
    /// kmalloc / kmem_cache_alloc / ExAllocatePool.
    Slab,
    /// vmalloc / kvmalloc large path.
    Vmalloc,
    /// __get_free_pages / alloc_pages.
    Pages,
}

impl Region {
    pub fn label(self) -> &'static str {
        match self {
            Region::Slab => "slab",
            Region::Vmalloc => "vmalloc",
            Region::Pages => "pages",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChunkState {
    Live,
    Freed,
}

/// One allocation, alive or in quarantine.
#[derive(Debug, Clone)]
pub struct KernelChunk {
    pub addr: u64,
    /// Usable bytes (the requested size rounded up to the slab bucket).
    pub size: u64,
    /// Bytes the driver actually asked for; anything past this is a redzone.
    pub req_size: u64,
    pub region: Region,
    pub state: ChunkState,
    /// Slab cache the chunk came from, e.g. `kmalloc-64` or a named cache.
    pub cache: String,
    pub map_name: String,
    pub alloc_api: String,
    pub alloc_pos: u64,
    pub alloc_rip: u64,
    pub free_api: String,
    pub free_pos: u64,
    pub free_rip: u64,
}

impl KernelChunk {
    pub fn contains(&self, addr: u64) -> bool {
        addr >= self.addr && addr < self.addr + self.size
    }

    /// Byte offset of `addr` inside the chunk, for report messages.
    pub fn offset_of(&self, addr: u64) -> u64 {
        addr.wrapping_sub(self.addr)
    }

    pub fn is_freed(&self) -> bool {
        self.state == ChunkState::Freed
    }
}

/// The allocation ledger for one emulated kernel.
#[derive(Debug)]
pub struct KernelHeap {
    slab_cursor: u64,
    slab_end: u64,
    vmalloc_cursor: u64,
    vmalloc_end: u64,
    /// Chunks in ascending address order — the bump cursors guarantee it, so
    /// lookups are a binary search.
    chunks: Vec<KernelChunk>,
    serial: u64,
}

impl KernelHeap {
    pub fn new(slab_base: u64, slab_size: u64, vmalloc_base: u64, vmalloc_size: u64) -> KernelHeap {
        KernelHeap {
            slab_cursor: slab_base,
            slab_end: slab_base + slab_size,
            vmalloc_cursor: vmalloc_base,
            vmalloc_end: vmalloc_base + vmalloc_size,
            chunks: Vec::new(),
            serial: 0,
        }
    }

    /// Slab bucket a request of `size` bytes lands in, matching the kmalloc
    /// power-of-two caches. The rounded size is what the driver may touch
    /// without it counting as an overflow, exactly like the real slab.
    pub fn bucket(size: u64) -> u64 {
        const BUCKETS: [u64; 12] = [8, 16, 32, 64, 96, 128, 192, 256, 512, 1024, 2048, 4096];
        for b in BUCKETS {
            if size <= b {
                return b;
            }
        }
        size.next_multiple_of(0x1000)
    }

    /// Reserve address space for a new chunk and record it. Returns the chunk
    /// address, or `None` when the region is exhausted.
    #[allow(clippy::too_many_arguments)]
    pub fn record_alloc(
        &mut self,
        region: Region,
        req_size: u64,
        cache: &str,
        api: &str,
        pos: u64,
        rip: u64,
    ) -> Option<&KernelChunk> {
        let size = match region {
            Region::Slab => Self::bucket(req_size.max(1)),
            Region::Vmalloc | Region::Pages => req_size.max(1).next_multiple_of(0x1000),
        };

        let (cursor, end) = match region {
            Region::Slab => (&mut self.slab_cursor, self.slab_end),
            Region::Vmalloc | Region::Pages => (&mut self.vmalloc_cursor, self.vmalloc_end),
        };

        let addr = (*cursor).next_multiple_of(16);
        if addr + size + HEAP_REDZONE > end {
            return None;
        }
        *cursor = addr + size + HEAP_REDZONE;

        self.serial += 1;
        self.chunks.push(KernelChunk {
            addr,
            size,
            req_size,
            region,
            state: ChunkState::Live,
            cache: cache.to_string(),
            map_name: format!("{}#{}", cache, self.serial),
            alloc_api: api.to_string(),
            alloc_pos: pos,
            alloc_rip: rip,
            free_api: String::new(),
            free_pos: 0,
            free_rip: 0,
        });
        self.chunks.last()
    }

    /// Index of the chunk containing `addr`, if any.
    pub fn index_of(&self, addr: u64) -> Option<usize> {
        // `chunks` is sorted by `addr`; find the last chunk starting at or
        // below the address and check whether it actually covers it.
        let idx = self.chunks.partition_point(|c| c.addr <= addr);
        if idx == 0 {
            return None;
        }
        let c = &self.chunks[idx - 1];
        if c.contains(addr) {
            Some(idx - 1)
        } else {
            None
        }
    }

    /// The chunk exactly starting at `addr` — what a `free(ptr)` must find.
    pub fn index_of_base(&self, addr: u64) -> Option<usize> {
        self.chunks.iter().position(|c| c.addr == addr)
    }

    pub fn get(&self, idx: usize) -> &KernelChunk {
        &self.chunks[idx]
    }

    pub fn chunk_at(&self, addr: u64) -> Option<&KernelChunk> {
        self.index_of(addr).map(|i| &self.chunks[i])
    }

    /// Mark a chunk freed and stamp it with the free site.
    pub fn record_free(&mut self, idx: usize, api: &str, pos: u64, rip: u64) {
        let c = &mut self.chunks[idx];
        c.state = ChunkState::Freed;
        c.free_api = api.to_string();
        c.free_pos = pos;
        c.free_rip = rip;
    }

    pub fn chunks(&self) -> &[KernelChunk] {
        &self.chunks
    }

    /// Chunks still live at the end of a run — candidate leaks once the module
    /// exit path has run.
    pub fn live(&self) -> impl Iterator<Item = &KernelChunk> {
        self.chunks.iter().filter(|c| !c.is_freed())
    }

    pub fn live_bytes(&self) -> u64 {
        self.live().map(|c| c.size).sum()
    }
}
