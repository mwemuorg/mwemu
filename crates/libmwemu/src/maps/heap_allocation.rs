use crate::utils::helpers::{likely, unlikely};
use std::cell::RefCell;
use std::cmp;
use std::collections::HashMap;
use std::ffi::c_char;
use std::rc::{Rc, Weak};

type PRFrag = Option<Rc<RefCell<Fragment>>>;
type PWFrag = Option<Weak<RefCell<Fragment>>>;
type RCFrag = Rc<RefCell<Fragment>>;
const NUM_BINS_MAX: usize = usize::BITS as usize * c_char::BITS as usize;
const O1HEAP_ALIGNMENT: usize = 256; // Must be a power of 2
const FRAGMENT_SIZE_MIN: usize = 256;
const FRAGMENT_SIZE_MAX: usize = (usize::MAX >> 1) + 1;

#[derive(Clone)]
pub struct O1HeapDiagnostics {
    pub capacity: usize,
    pub allocated: usize,
    pub peak_allocated: usize,
    pub peak_request_size: usize,
    pub oom_count: usize,
}

/// Plain-data copy of an `O1Heap`: the fragment list in address order plus
/// the offsets indexed by `hashes`. Rebuilding from it gives a heap that
/// shares no nodes with the original (an `Rc` clone would share them).
#[derive(Clone)]
pub struct O1HeapSnapshot {
    base: u64,
    diagnostics: O1HeapDiagnostics,
    fragments: Vec<(u32, u32, bool)>, // (offset, size, used)
    hash_keys: Vec<u32>,
}

/// Result of a successful reallocation.
///
/// The allocator owns fragment metadata, while the guest bytes live in the
/// emulator's memory maps. `new_addr` is therefore returned separately from
/// the copy operation. `copy_size == 0` means the allocation stayed in place;
/// otherwise the caller must copy `copy_size` bytes from the old address to
/// `new_addr` using an overlap-safe copy.
pub struct ReallocResult {
    /// Address at which the resized allocation is now located.
    pub new_addr: u64,
    /// Number of old allocation bytes that must be copied to `new_addr`.
    pub copy_size: usize,
}

const UNDEFINE_OFFSET: u32 = 0xffffffffu32;

struct Fragment {
    offset: u32,
    size: u32,
    used: bool,
    // Using u32 as offset from the base to the fragment
    next: PRFrag,
    prev: PWFrag,
    // Free list links
    next_free: PRFrag,
    prev_free: PWFrag,
}

impl Fragment {
    fn new(offset: u32, size: u32) -> Self {
        Self {
            offset,
            size,
            used: false,
            next: None,
            prev: None,
            next_free: None,
            prev_free: None,
        }
    }
}

pub struct O1Heap {
    base: u64,
    bins: Vec<PRFrag>, // bins store the offset to the fragment, but not the pointer
    hashes: HashMap<u32, RCFrag, nohash_hasher::BuildNoHashHasher<u32>>,
    nonempty_bin_mask: usize,
    diagnostics: O1HeapDiagnostics,
}

impl Clone for O1Heap {
    /// Deep copy: the clone shares no fragment nodes with `self`.
    fn clone(&self) -> Self {
        Self::from_snapshot(&self.snapshot())
    }
}

impl O1Heap {
    fn log2_floor(&self, x: usize) -> usize {
        if x == 0 {
            return 0;
        }
        (usize::BITS - 1 - x.leading_zeros()) as usize
    }

    fn round_up_to_power_of_2(&self, x: usize) -> usize {
        if x <= 1 {
            return 1;
        }
        let highest = 1usize << (usize::BITS - 1);
        if x > highest {
            return 0;
        }
        if x.is_power_of_two() {
            return x;
        }
        1usize << (usize::BITS - x.leading_zeros())
    }

    fn log2_ceil(&self, x: usize) -> usize {
        if x <= 1 {
            return 0;
        }
        let floor = self.log2_floor(x - 1);
        floor + 1
    }
    /// Return the absolute base address of this heap arena.
    pub fn base(&self) -> u64 {
        self.base
    }

    pub const MIN_ARENA_SIZE: usize = {
        let instance_size = std::mem::size_of::<O1Heap>();
        let padded = (instance_size + O1HEAP_ALIGNMENT - 1) & !(O1HEAP_ALIGNMENT - 1);
        padded + FRAGMENT_SIZE_MIN
    };

    /// Create a new heap instance with the specified capacity
    ///
    /// # Arguments
    ///
    /// * `capacity` - The total size of the heap in bytes
    ///
    /// # Returns
    ///
    /// * `Some(O1Heap)` - A new heap instance if successful
    /// * `None` - If the capacity is less than the minimum required size
    pub fn new(base: u64, size: u32) -> Result<Self, &'static str> {
        if size < Self::MIN_ARENA_SIZE as u32 {
            return Err("size is less than the min arena size");
        }

        let hashes: HashMap<u32, RCFrag, nohash_hasher::BuildNoHashHasher<u32>> =
            HashMap::default();
        let mut heap = Self {
            base,
            bins: vec![None; NUM_BINS_MAX],
            hashes,
            nonempty_bin_mask: 0,
            diagnostics: O1HeapDiagnostics {
                capacity: size as usize,
                allocated: 0,
                peak_allocated: 0,
                peak_request_size: 0,
                oom_count: 0,
            },
        };

        let initial_fragment = Rc::new(RefCell::new(Fragment::new(0, size)));
        heap.hashes.insert(0, initial_fragment.clone());
        heap.rebin(initial_fragment);

        Ok(heap)
    }

    fn rebin(&mut self, fragment: Rc<RefCell<Fragment>>) {
        let size = fragment.borrow().size;
        if size < FRAGMENT_SIZE_MIN as u32 {
            return;
        }

        let idx = self.log2_floor(size as usize / FRAGMENT_SIZE_MIN);
        if idx >= NUM_BINS_MAX {
            return;
        }

        // Add to beginning of bin list
        fragment.borrow_mut().next_free = self.bins[idx].take();
        fragment.borrow_mut().prev_free = None;

        if let Some(ref next) = fragment.borrow().next_free {
            next.borrow_mut().prev_free = Some(Rc::downgrade(&fragment));
        }

        self.bins[idx] = Some(fragment);
        self.nonempty_bin_mask |= 1 << idx;
    }

    /// Any live fragment: used ones stay in `hashes`, free ones sit in a bin.
    fn any_fragment(&self) -> Option<RCFrag> {
        self.hashes
            .values()
            .next()
            .cloned()
            .or_else(|| self.bins.iter().flatten().next().cloned())
    }

    /// Lowest-address fragment, found by walking `prev` links.
    fn first_fragment(&self) -> Option<RCFrag> {
        let mut cur = self.any_fragment()?;
        loop {
            let prev = cur.borrow().prev.as_ref().and_then(Weak::upgrade);
            match prev {
                Some(p) => cur = p,
                None => return Some(cur),
            }
        }
    }

    /// Capture the heap bookkeeping as plain data.
    pub fn snapshot(&self) -> O1HeapSnapshot {
        let mut fragments = Vec::new();
        let mut cur = self.first_fragment();
        while let Some(frag) = cur {
            let f = frag.borrow();
            fragments.push((f.offset, f.size, f.used));
            cur = f.next.clone();
        }
        let mut hash_keys: Vec<u32> = self.hashes.keys().copied().collect();
        hash_keys.sort_unstable();
        O1HeapSnapshot {
            base: self.base,
            diagnostics: self.diagnostics.clone(),
            fragments,
            hash_keys,
        }
    }

    /// Rebuild a heap from a snapshot. Free fragments are re-binned, so the
    /// order inside a bin may differ from the source; that can change which
    /// address a later `allocate` picks, never whether it succeeds.
    pub fn from_snapshot(snap: &O1HeapSnapshot) -> Self {
        let mut heap = Self {
            base: snap.base,
            bins: vec![None; NUM_BINS_MAX],
            hashes: HashMap::default(),
            nonempty_bin_mask: 0,
            diagnostics: snap.diagnostics.clone(),
        };
        let mut by_offset: HashMap<u32, RCFrag> = HashMap::new();
        let mut prev: Option<RCFrag> = None;
        for &(offset, size, used) in &snap.fragments {
            let frag = Rc::new(RefCell::new(Fragment::new(offset, size)));
            frag.borrow_mut().used = used;
            if let Some(p) = &prev {
                p.borrow_mut().next = Some(frag.clone());
                frag.borrow_mut().prev = Some(Rc::downgrade(p));
            }
            if !used {
                heap.rebin(frag.clone());
            }
            by_offset.insert(offset, frag.clone());
            prev = Some(frag);
        }
        // Keep the same index; keys whose node left the list were stale.
        for key in &snap.hash_keys {
            if let Some(frag) = by_offset.get(key) {
                heap.hashes.insert(*key, frag.clone());
            }
        }
        heap
    }

    /// Allocate a block of memory
    ///
    /// Allocates a block of memory of at least the specified size. The actual
    /// allocated size will be rounded up to the next power of 2.
    ///
    /// # Arguments
    ///
    /// * `amount` - The minimum number of bytes to allocate
    ///
    /// # Returns
    ///
    /// * `Some(usize)` - An offset into the heap's memory arena if successful
    /// * `None` - If there is insufficient memory or the request is invalid
    pub fn allocate(&mut self, amount: usize) -> Option<u64> {
        if unlikely(amount == 0) {
            return None;
        }

        // Update peak request size
        // Calculate fragment size (power of 2).
        let fragment_size = self.round_up_to_power_of_2(amount);
        if fragment_size == 0 || fragment_size > self.diagnostics.capacity {
            self.diagnostics.oom_count += 1;
            return None;
        }

        let optimal_bin_index = self.log2_ceil(fragment_size / FRAGMENT_SIZE_MIN);
        let candidate_bin_mask = !((1 << optimal_bin_index) - 1);
        let suitable_bins = self.nonempty_bin_mask & candidate_bin_mask;

        // Find smallest suitable bin
        if likely(suitable_bins != 0) {
            let smallest_bin_index = suitable_bins.trailing_zeros() as usize;

            if smallest_bin_index < NUM_BINS_MAX {
                // Get fragment from bin
                let frag_rc = self.bins[smallest_bin_index].take().unwrap();
                self.unbin(&frag_rc);

                let frag_size = frag_rc.borrow().size;
                let frag_offset = frag_rc.borrow().offset;
                frag_rc.borrow_mut().size = fragment_size as u32;
                // Split if necessary
                let leftover = frag_size - fragment_size as u32;
                if likely(leftover >= FRAGMENT_SIZE_MIN as u32) {
                    let new_frag = Rc::new(RefCell::new(Fragment::new(
                        frag_offset + fragment_size as u32,
                        leftover,
                    )));
                    // Link the new fragment in the chain
                    let next_rc = frag_rc.borrow().next.clone();
                    new_frag.borrow_mut().next = next_rc.clone();
                    new_frag.borrow_mut().prev = Some(Rc::downgrade(&frag_rc));

                    if let Some(ref next) = next_rc {
                        next.borrow_mut().prev = Some(Rc::downgrade(&new_frag));
                    }

                    frag_rc.borrow_mut().next = Some(new_frag.clone());
                    self.hashes
                        .insert(frag_offset + fragment_size as u32, new_frag.clone());
                    // Add the new fragment to the appropriate bin
                    self.rebin(new_frag);
                }

                // Mark as used
                frag_rc.borrow_mut().used = true;
                frag_rc.borrow_mut().size = fragment_size as u32;

                self.diagnostics.allocated += fragment_size;
                self.diagnostics.peak_allocated =
                    cmp::max(self.diagnostics.peak_allocated, self.diagnostics.allocated);

                // Return "pointer" (offset in our case)
                return Some(frag_offset as u64 + self.base);
            }
        }

        None
    }

    // remove fragment from the bin
    fn unbin(&mut self, fragment: &Rc<RefCell<Fragment>>) {
        let size = fragment.borrow().size;
        if unlikely(size < FRAGMENT_SIZE_MIN as u32) {
            return;
        }

        let idx = self.log2_floor(size as usize / FRAGMENT_SIZE_MIN);
        if unlikely(idx >= NUM_BINS_MAX) {
            return;
        }

        // Remove from free list
        if let Some(ref next) = fragment.borrow().next_free {
            next.borrow_mut().prev_free = fragment.borrow().prev_free.clone();
        }

        if let Some(ref prev) = fragment.borrow().prev_free {
            if let Some(prev_rc) = prev.upgrade() {
                prev_rc.borrow_mut().next_free = fragment.borrow().next_free.clone();
            }
        } else {
            // Was first in list
            self.bins[idx] = fragment.borrow().next_free.clone();
            if self.bins[idx].is_none() {
                self.nonempty_bin_mask &= !(1 << idx);
            }
        }
    }

    fn find_fragment_by_offset(&self, offset: u32) -> Option<Rc<RefCell<Fragment>>> {
        // In a real implementation, we'd have a more efficient way to find fragments
        // For now, we'll search through our fragments collection
        self.hashes.get(&offset).cloned()
    }

    /// Return whether an address falls inside the arena and corresponds to an
    /// active allocated fragment. Returns false for addresses below the arena
    /// base, unmapped pointers, free fragments, or stale hash entries.
    pub fn check_fragment_exists(&self, addr: u64) -> bool {
        if addr < self.base {
            return false;
        }
        let offset = (addr - self.base) as u32;
        match self.hashes.get(&offset) {
            Some(frag) => frag.borrow().used,
            None => false,
        }
    }

    /// Return the active allocated fragment size in bytes for the given
    /// address, or `None` if the address is below the arena base, not tracked,
    /// or already freed. The returned size is the rounded fragment size, which
    /// is the maximum valid copy range for the allocation.
    pub fn allocation_size(&self, addr: u64) -> Option<usize> {
        if addr < self.base {
            return None;
        }
        let offset = (addr - self.base) as u32;
        let frag = self.hashes.get(&offset)?;
        if !frag.borrow().used {
            return None;
        }
        Some(frag.borrow().size as usize)
    }

    pub fn free(&mut self, address: u64) {
        let offset = (address - self.base) as u32;
        let frag_rc = match self.find_fragment_by_offset(offset) {
            Some(frag) => frag,
            None => return, // Fragment not found
        };

        if !frag_rc.borrow().used {
            return; // Already freed
        }

        let frag_size = frag_rc.borrow().size;
        if frag_size < FRAGMENT_SIZE_MIN as u32
            || frag_size > self.diagnostics.capacity as u32
            || frag_size % FRAGMENT_SIZE_MIN as u32 != 0
        {
            return; // Invalid fragment
        }

        // Update the diagnostics before merging because the merge invalidates
        // the fragment size information. Underflow indicates heap corruption.
        if self.diagnostics.allocated < frag_size as usize {
            return;
        }
        self.diagnostics.allocated -= frag_size as usize;

        // Even if we're going to drop the fragment later, mark it free anyway
        // to prevent double-free.
        frag_rc.borrow_mut().used = false;
        self.hashes.remove(&frag_rc.borrow().offset);

        // Merge with siblings and insert the returned fragment into the
        // appropriate bin and update metadata.
        let join_left = {
            if let Some(ref prev_weak) = frag_rc.borrow().prev {
                if let Some(prev_rc) = prev_weak.upgrade() {
                    !prev_rc.borrow().used
                } else {
                    false
                }
            } else {
                false
            }
        };

        let join_right = {
            if let Some(ref next_rc) = frag_rc.borrow().next {
                !next_rc.borrow().used
            } else {
                false
            }
        };

        if join_left && join_right {
            // [ prev ][ this ][ next ] => [ ------- prev ------- ]
            let prev_rc = frag_rc.borrow().prev.as_ref().unwrap().upgrade().unwrap();
            let next_rc = frag_rc.borrow().next.as_ref().unwrap().clone();

            self.unbin(&prev_rc);
            self.unbin(&next_rc);

            prev_rc.borrow_mut().size += frag_rc.borrow().size + next_rc.borrow().size;
            frag_rc.borrow_mut().size = 0; // Invalidate to prevent double-free
            next_rc.borrow_mut().size = 0; // Invalidate to prevent double-free

            // Link prev to next's next
            let next_next = next_rc.borrow().next.clone();
            prev_rc.borrow_mut().next = next_next.clone();
            if let Some(ref nn) = next_next {
                nn.borrow_mut().prev = Some(Rc::downgrade(&prev_rc));
            }

            self.rebin(prev_rc);
        } else if join_left {
            // [ prev ][ this ][ next ] => [ --- prev --- ][ next ]
            let prev_rc = frag_rc.borrow().prev.as_ref().unwrap().upgrade().unwrap();

            self.unbin(&prev_rc);

            prev_rc.borrow_mut().size += frag_rc.borrow().size;
            frag_rc.borrow_mut().size = 0; // Invalidate to prevent double-free

            // Link prev to next
            let next_rc = frag_rc.borrow().next.clone();
            prev_rc.borrow_mut().next = next_rc.clone();
            if let Some(ref next) = next_rc {
                next.borrow_mut().prev = Some(Rc::downgrade(&prev_rc));
            }

            self.rebin(prev_rc);
        } else if join_right {
            // [ prev ][ this ][ next ] => [ prev ][ --- this --- ]
            let next_rc = frag_rc.borrow().next.as_ref().unwrap().clone();

            self.unbin(&next_rc);

            frag_rc.borrow_mut().size += next_rc.borrow().size;
            next_rc.borrow_mut().size = 0; // Invalidate to prevent double-free

            // Link frag to next's next
            let next_next = next_rc.borrow().next.clone();
            frag_rc.borrow_mut().next = next_next.clone();
            if let Some(ref nn) = next_next {
                nn.borrow_mut().prev = Some(Rc::downgrade(&frag_rc));
            }

            self.rebin(frag_rc);
        } else {
            // No merging needed
            self.rebin(frag_rc);
        }
    }

    /// Resize one active allocation while preserving its fragment metadata.
    ///
    /// The algorithm follows four ordered cases:
    ///
    /// 1. A zero request delegates to `free` and returns no allocation.
    /// 2. A smaller/equal rounded size stays at the same address; a large
    ///    enough remainder becomes a new free fragment.
    /// 3. A larger request first tries to consume a free right neighbor, so
    ///    the guest bytes remain in place.
    /// 4. If the right side is insufficient, a free left neighbor may absorb
    ///    the allocation. This moves bytes backward and reports `copy_size`.
    ///
    /// If neither in-place option fits, the final fallback allocates a new
    /// fragment and frees the old one. The caller performs the reported copy.
    /// All fragment sizes are rounded powers of two, matching `allocate`.
    pub fn reallocate(&mut self, address: u64, new_amount: usize) -> Option<ReallocResult> {
        if unlikely(new_amount == 0) {
            self.free(address);
            return None;
        }
        self.diagnostics.peak_request_size =
            cmp::max(self.diagnostics.peak_request_size, new_amount);
        // Convert the guest address into the arena-relative hash key. A
        // below-base, overflowing, unknown, or freed address is invalid.
        let offset = match address.checked_sub(self.base) {
            Some(o) if o <= u32::MAX as u64 => o as u32,
            _ => return None,
        };
        let frag = self.find_fragment_by_offset(offset)?;
        if !frag.borrow().used {
            return None;
        }

        // Existing fragments are power-of-two-sized arena blocks. Reject
        // corrupt metadata before using it in diagnostics or pointer math.
        let frag_size = frag.borrow().size as usize;
        if frag_size < FRAGMENT_SIZE_MIN
            || frag_size > self.diagnostics.capacity
            || !frag_size.is_multiple_of(FRAGMENT_SIZE_MIN)
        {
            return None;
        }

        // The allocator reserves the next power-of-two fragment size. A
        // zero result means the rounding operation would overflow usize.
        let new_frag_size = self.round_up_to_power_of_2(new_amount);
        if new_frag_size == 0
            || new_amount > self.diagnostics.capacity
            || new_frag_size > self.diagnostics.capacity
        {
            self.diagnostics.oom_count += 1;
            return None;
        }

        // Snapshot both neighbors before changing links or bin membership.
        // `prev` is weak because the address-order graph owns forward links.
        let prev_rc = frag.borrow().prev.as_ref().and_then(|w| w.upgrade());
        let prev_free = match &prev_rc {
            Some(p) => !p.borrow().used,
            None => false,
        };
        let next_rc = frag.borrow().next.clone();
        let next_free = match &next_rc {
            Some(n) => !n.borrow().used,
            None => false,
        };
        let prev_size = prev_rc
            .as_ref()
            .map(|p| p.borrow().size as usize)
            .unwrap_or(0);
        let next_size = next_rc
            .as_ref()
            .map(|n| n.borrow().size as usize)
            .unwrap_or(0);

        // Case 1 — shrink or keep the same rounded size. The user pointer
        // stays unchanged. If the released tail is large enough to be a
        // legal fragment, unlink any adjacent free fragment, create the new
        // free tail, update the address-order links, and rebin that tail.
        // A sub-minimum tail is deliberately left as internal slack.
        if new_frag_size <= frag_size {
            let leftover = frag_size - new_frag_size;
            if likely(leftover >= FRAGMENT_SIZE_MIN) {
                if self.diagnostics.allocated < leftover {
                    return None;
                }
                self.diagnostics.allocated -= leftover;
                let next_next = if next_free {
                    let next = next_rc.as_ref().unwrap();
                    self.unbin(next);
                    let next_offset = next.borrow().offset;
                    let next_next = next.borrow().next.clone();
                    next.borrow_mut().size = 0;
                    self.hashes.remove(&next_offset);
                    next_next
                } else {
                    frag.borrow().next.clone()
                };
                let new_frag = Rc::new(RefCell::new(Fragment::new(
                    offset + new_frag_size as u32,
                    (if next_free {
                        leftover + next_size
                    } else {
                        leftover
                    }) as u32,
                )));
                new_frag.borrow_mut().next = next_next.clone();
                new_frag.borrow_mut().prev = Some(Rc::downgrade(&frag));
                if let Some(ref nn) = next_next {
                    nn.borrow_mut().prev = Some(Rc::downgrade(&new_frag));
                }
                frag.borrow_mut().next = Some(new_frag.clone());
                frag.borrow_mut().size = new_frag_size as u32;
                self.hashes
                    .insert(offset + new_frag_size as u32, new_frag.clone());
                self.rebin(new_frag);
            }
            return Some(ReallocResult {
                new_addr: address,
                copy_size: 0,
            });
        }

        // Case 2 — grow forward in place. Consume the free fragment on the
        // right, then either split its remainder into a new free tail or
        // absorb it completely. No guest-byte copy is needed.
        if next_free && (frag_size + next_size) >= new_frag_size {
            let next = next_rc.as_ref().unwrap();
            self.unbin(next);
            let next_offset = next.borrow().offset;
            let next_next = next.borrow().next.clone();
            next.borrow_mut().size = 0;
            self.hashes.remove(&next_offset);
            let leftover = frag_size + next_size - new_frag_size;
            if likely(leftover >= FRAGMENT_SIZE_MIN) {
                let new_frag = Rc::new(RefCell::new(Fragment::new(
                    offset + new_frag_size as u32,
                    leftover as u32,
                )));
                new_frag.borrow_mut().next = next_next.clone();
                new_frag.borrow_mut().prev = Some(Rc::downgrade(&frag));
                if let Some(ref nn) = next_next {
                    nn.borrow_mut().prev = Some(Rc::downgrade(&new_frag));
                }
                frag.borrow_mut().next = Some(new_frag.clone());
                frag.borrow_mut().size = new_frag_size as u32;
                self.hashes
                    .insert(offset + new_frag_size as u32, new_frag.clone());
                self.rebin(new_frag);
                self.diagnostics.allocated += new_frag_size - frag_size;
            } else {
                frag.borrow_mut().next = next_next.clone();
                if let Some(ref nn) = next_next {
                    nn.borrow_mut().prev = Some(Rc::downgrade(&frag));
                }
                frag.borrow_mut().size = (frag_size + next_size) as u32;
                self.diagnostics.allocated += next_size;
            }
            self.diagnostics.peak_allocated =
                cmp::max(self.diagnostics.peak_allocated, self.diagnostics.allocated);
            return Some(ReallocResult {
                new_addr: address,
                copy_size: 0,
            });
        }

        // Case 3 — grow backward when the right side alone is insufficient.
        // The free predecessor becomes the resized allocation, optionally
        // consuming the free successor too. Metadata is rebuilt before the
        // caller moves the old payload backward to `prev_offset`.
        if prev_free && (prev_size + frag_size + next_size) >= new_frag_size {
            let prev = prev_rc.as_ref().unwrap();
            self.unbin(prev);
            let prev_offset = prev.borrow().offset;
            if next_free {
                self.unbin(next_rc.as_ref().unwrap());
            }
            let after = if next_free {
                next_rc.as_ref().unwrap().borrow().next.clone()
            } else {
                frag.borrow().next.clone()
            };
            let leftover = prev_size + frag_size + next_size - new_frag_size;
            prev.borrow_mut().used = true;
            if likely(leftover >= FRAGMENT_SIZE_MIN) {
                let new_frag = Rc::new(RefCell::new(Fragment::new(
                    prev_offset + new_frag_size as u32,
                    leftover as u32,
                )));
                new_frag.borrow_mut().next = after.clone();
                new_frag.borrow_mut().prev = Some(Rc::downgrade(prev));
                if let Some(ref nn) = after {
                    nn.borrow_mut().prev = Some(Rc::downgrade(&new_frag));
                }
                prev.borrow_mut().next = Some(new_frag.clone());
                prev.borrow_mut().size = new_frag_size as u32;
                self.hashes
                    .insert(prev_offset + new_frag_size as u32, new_frag.clone());
                self.rebin(new_frag);
                self.diagnostics.allocated += new_frag_size - frag_size;
            } else {
                prev.borrow_mut().next = after.clone();
                if let Some(ref nn) = after {
                    nn.borrow_mut().prev = Some(Rc::downgrade(prev));
                }
                prev.borrow_mut().size = (prev_size + frag_size + next_size) as u32;
                self.diagnostics.allocated += prev_size + next_size;
            }
            frag.borrow_mut().used = false;
            frag.borrow_mut().size = 0;
            self.hashes.remove(&offset);
            if next_free {
                let next = next_rc.as_ref().unwrap();
                let next_offset = next.borrow().offset;
                next.borrow_mut().size = 0;
                self.hashes.remove(&next_offset);
            }
            self.hashes.insert(prev_offset, prev.clone());
            self.diagnostics.peak_allocated =
                cmp::max(self.diagnostics.peak_allocated, self.diagnostics.allocated);
            return Some(ReallocResult {
                new_addr: self.base + prev_offset as u64,
                copy_size: frag_size,
            });
        }

        // Case 4 — neither adjacent layout can satisfy the request. Allocate
        // a separate fragment and release the old one. The metadata remains
        // valid immediately; the caller copies the old payload afterward.
        let new_addr = self.allocate(new_amount)?;
        self.free(address);
        Some(ReallocResult {
            new_addr,
            copy_size: frag_size,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn new_heap() -> O1Heap {
        O1Heap::new(0x1000, 0x10000).expect("O1Heap::new")
    }

    #[test]
    fn test_reallocate_in_place_grow_forward() {
        let mut h = new_heap();
        let a = h.allocate(0x100).expect("alloc");
        let r = h.reallocate(a, 0x400).expect("realloc");
        assert_eq!(r.new_addr, a);
        assert_eq!(r.copy_size, 0);
        assert_eq!(h.allocation_size(a), Some(0x400));
        assert!(h.check_fragment_exists(a));
        assert_eq!(h.diagnostics.allocated, 0x400);
    }

    #[test]
    fn test_reallocate_shrink_splits_leftover() {
        let mut h = new_heap();
        let a = h.allocate(0x400).expect("alloc");
        let r = h.reallocate(a, 0x100).expect("realloc");
        assert_eq!(r.new_addr, a);
        assert_eq!(r.copy_size, 0);
        assert_eq!(h.allocation_size(a), Some(0x100));
        let c = h.allocate(0x100).expect("alloc of leftover");
        assert_eq!(c, a + 0x100);
    }

    #[test]
    fn test_reallocate_grow_returns_result() {
        let mut h = new_heap();
        let a = h.allocate(0x100).expect("alloc A");
        let b = h.allocate(0x100).expect("alloc B");
        h.free(a);
        let r = h.reallocate(b, 0x400).expect("realloc");
        // Either the backward expansion (copy_size == old frag size) or the
        // forward expansion (copy_size == 0) may be taken depending on
        // which neighbor absorbed. Both paths must produce a live 0x400
        // allocation at the returned address.
        assert!(r.copy_size == 0 || r.copy_size == 0x100);
        assert_eq!(h.allocation_size(r.new_addr), Some(0x400));
    }

    #[test]
    fn test_reallocate_fallback_moves() {
        let mut h = new_heap();
        let a = h.allocate(0x100).expect("alloc A");
        let b = h.allocate(0x100).expect("alloc B");
        let _big = h.allocate(0x8000).expect("alloc tail");
        let r = h.reallocate(b, 0x4000).expect("realloc");
        assert_ne!(r.new_addr, b);
        assert_eq!(r.copy_size, 0x100);
        assert!(h.allocation_size(b).is_none());
        assert!(h.check_fragment_exists(r.new_addr));
        assert_eq!(h.diagnostics.allocated, 0x100 + 0x4000 + 0x8000);
    }

    #[test]
    fn test_reallocate_oom() {
        let mut h = new_heap();
        let a = h.allocate(0x100).expect("alloc");
        let r = h.reallocate(a, 0x20000);
        assert!(r.is_none());
        assert_eq!(h.diagnostics.oom_count, 1);
        assert!(h.check_fragment_exists(a));
    }

    #[test]
    fn test_reallocate_zero_frees() {
        let mut h = new_heap();
        let a = h.allocate(0x100).expect("alloc");
        let r = h.reallocate(a, 0);
        assert!(r.is_none());
        assert!(!h.check_fragment_exists(a));
        assert_eq!(h.diagnostics.allocated, 0);
    }

    #[test]
    fn test_reallocate_invalid_address() {
        let mut h = new_heap();
        assert!(h.reallocate(0xff8, 0x100).is_none());
        assert!(h.reallocate(0x1040, 0x100).is_none());
        let a = h.allocate(0x100).expect("alloc");
        h.free(a);
        assert!(h.reallocate(a, 0x100).is_none());
    }

    #[test]
    fn test_round_up_overflow_returns_oom() {
        let mut h = new_heap();
        let r = h.allocate(usize::MAX - 1);
        assert!(r.is_none());
        assert_eq!(h.diagnostics.oom_count, 1);
    }

    #[test]
    fn test_reallocate_huge_amount_is_oom() {
        let mut h = new_heap();
        let a = h.allocate(0x100).expect("alloc");
        let r = h.reallocate(a, usize::MAX - 1);
        assert!(r.is_none());
        assert_eq!(h.diagnostics.oom_count, 1);
        assert!(h.check_fragment_exists(a));
    }

    /// Mixed heap: block at offset 0 freed, free holes between used blocks.
    fn fragmented_heap() -> (O1Heap, Vec<u64>) {
        let mut h = new_heap();
        let blocks: Vec<u64> = [256, 512, 256, 1024, 256]
            .iter()
            .map(|n| h.allocate(*n).unwrap())
            .collect();
        h.free(blocks[0]); // offset 0
        h.free(blocks[2]); // hole between used blocks
        let live = vec![blocks[1], blocks[3], blocks[4]];
        (h, live)
    }

    fn overlaps(a: u64, a_len: usize, b: u64, b_len: usize) -> bool {
        a < b + b_len as u64 && b < a + a_len as u64
    }

    #[test]
    fn test_clone_keeps_live_allocation_sizes() {
        let (h, live) = fragmented_heap();
        let c = h.clone();
        for addr in &live {
            assert_eq!(c.allocation_size(*addr), h.allocation_size(*addr));
        }
        assert_eq!(c.diagnostics.allocated, h.diagnostics.allocated);
    }

    #[test]
    fn test_clone_frees_pre_snapshot_pointer() {
        let (h, live) = fragmented_heap();
        let mut c = h.clone();
        let before = c.diagnostics.allocated;
        c.free(live[0]);
        assert!(c.diagnostics.allocated < before);
        assert!(c.allocation_size(live[0]).is_none());
    }

    #[test]
    fn test_clone_never_allocates_over_live_blocks() {
        let (h, live) = fragmented_heap();
        let mut c = h.clone();
        let sizes: Vec<usize> = live
            .iter()
            .map(|a| h.allocation_size(*a).unwrap())
            .collect();
        while let Some(n) = c.allocate(256) {
            for (addr, len) in live.iter().zip(&sizes) {
                assert!(
                    !overlaps(n, 256, *addr, *len),
                    "0x{:x} overlaps 0x{:x}",
                    n,
                    addr
                );
            }
        }
    }

    #[test]
    fn test_clone_is_independent_of_source() {
        let (mut h, live) = fragmented_heap();
        let c = h.clone();
        h.free(live[1]);
        assert!(h.allocation_size(live[1]).is_none());
        assert!(c.allocation_size(live[1]).is_some());
    }

    #[test]
    fn test_clone_preserves_free_space() {
        let (mut h, _) = fragmented_heap();
        let mut c = h.clone();
        let fill = |x: &mut O1Heap| std::iter::from_fn(|| x.allocate(256)).count();
        assert_eq!(fill(&mut c), fill(&mut h));
    }
}
