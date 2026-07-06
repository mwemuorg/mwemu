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

pub struct O1HeapDiagnostics {
    pub capacity: usize,
    pub allocated: usize,
    pub peak_allocated: usize,
    pub peak_request_size: usize,
    pub oom_count: usize,
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

impl O1Heap {
    fn log2_floor(&self, x: usize) -> usize {
        if x == 0 {
            return 0;
        }
        (usize::BITS - 1 - x.leading_zeros()) as usize
    }

    fn round_up_to_power_of_2(&self, x: usize) -> usize {
        if x == 0 {
            return 1;
        }
        if x.is_power_of_two() {
            return x;
        }
        1 << (usize::BITS - x.leading_zeros())
    }

    fn log2_ceil(&self, x: usize) -> usize {
        if x <= 1 {
            return 0;
        }
        let floor = self.log2_floor(x - 1);
        floor + 1
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
        if likely(self.diagnostics.peak_request_size < amount) {
            self.diagnostics.peak_request_size = amount;
        }

        // Calculate fragment size (power of 2)
        let fragment_size = self.round_up_to_power_of_2(amount);
        if fragment_size > self.diagnostics.capacity {
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

    pub fn check_fragment_exists(&self, addr: u64) -> bool {
        let offset = (addr - self.base) as u32;
        self.hashes.get(&offset).is_some()
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

        // Update the diagnostics. It must be done before merging because it
        // invalidates the fragment size information.
        if self.diagnostics.allocated < frag_size as usize {
            // Heap corruption
            return;
        }
        self.diagnostics.allocated -= frag_size as usize;

        // Update the diagnostics. It must be done before merging because it
        // invalidates the fragment size information.
        if self.diagnostics.allocated < frag_size as usize {
            // Heap corruption
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
}
