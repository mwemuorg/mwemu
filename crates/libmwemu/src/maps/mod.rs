pub mod heap_allocation;
mod inspection;
pub mod mem64;
pub mod scalar;
mod search;
pub mod tlb;
mod utilities;

use crate::maps::mem64::Permission;
use crate::maps::scalar::{LittleEndianScalar, ScalarKind};
use crate::maps::tlb::LPF_OF;
use crate::windows::constants;
use ahash::AHashMap;
use mem64::Mem64;
use serde::{Deserialize, Serialize};
use slab::Slab;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::convert::TryInto;
use tlb::TLB;

#[derive(Clone, Serialize, Deserialize)]
pub struct Maps {
    pub banzai: bool,
    // adding slab so that it is easier to manage memory, now every other place contain the
    // key to the memory slab
    pub mem_slab: Slab<Mem64>,
    pub maps: BTreeMap<u64, usize>,
    pub name_map: AHashMap<String, usize>,
    pub is_64bits: bool,
    tlb: RefCell<TLB>,
    /// Maximum allocation size (default 0xffffff / ~16MB). Allocations larger than this are capped.
    pub max_alloc_size: u64,
}

impl Default for Maps {
    fn default() -> Self {
        Maps {
            mem_slab: Slab::with_capacity(200),
            maps: BTreeMap::<u64, usize>::default(),
            name_map: AHashMap::<String, usize>::with_capacity(200),
            is_64bits: false,
            banzai: false,
            tlb: RefCell::new(TLB::new()),
            max_alloc_size: 0xffffff,
        }
    }
}

impl Maps {
    const DEFAULT_ALIGNMENT: u64 = 0x1000; //16;

    pub fn new(
        mem_slab: Slab<Mem64>,
        maps: BTreeMap<u64, usize>,
        name_map: AHashMap<String, usize>,
        is_64bits: bool,
        banzai: bool,
        tlb: RefCell<TLB>,
    ) -> Maps {
        Maps {
            banzai,
            mem_slab,
            maps,
            name_map,
            is_64bits,
            tlb,
            max_alloc_size: 0xffffff,
        }
    }

    pub fn set_banzai(&mut self, banzai: bool) {
        self.banzai = banzai;
    }

    pub fn clear(&mut self) {
        self.mem_slab.clear();
        self.maps.clear();
        self.name_map.clear();
    }

    pub fn get_base(&self) -> Option<u64> {
        self.mem_slab
            .iter()
            .find(|map| map.1.get_name().ends_with(".pe"))
            .map(|map| map.1.get_base())
    }

    pub fn overflow_predicted(&self, addr: u64, amount: u64) -> bool {
        self.maps
            .range(..=addr)
            .next_back()
            .and_then(|(start, region_key)| {
                let region = self.mem_slab.get(*region_key)?;
                let start = *start;
                let size = region.size() as u64;

                if addr >= start && addr < start + size {
                    if addr + amount < start + size {
                        Some(false)
                    } else {
                        Some(true)
                    }
                } else {
                    Some(true)
                }
            })
            .unwrap_or(true)
    }

    #[inline(always)]
    pub fn exists_mapname(&self, name: &str) -> bool {
        self.name_map.contains_key(name)
    }

    // slow, better hold the object
    pub fn get_map_by_name(&self, name: &str) -> Option<&Mem64> {
        self.name_map.get(name).and_then(|v| self.mem_slab.get(*v))
    }

    pub fn get_map_by_name_mut(&mut self, name: &str) -> Option<&mut Mem64> {
        let name_key = self.name_map.get(name)?;
        self.mem_slab.get_mut(*name_key)
    }

    pub fn get_mem_size(&self, addr: u64) -> Option<usize> {
        self.maps
            .range(..=addr)
            .next_back()
            .and_then(|(start, region_key)| {
                let region = self.mem_slab.get(*region_key)?;
                let start = *start;
                let size = region.size() as u64;
                if addr >= start && addr < start + size {
                    Some(region.size())
                } else {
                    None
                }
            })
    }

    pub fn create_map(
        &mut self,
        name: &str,
        base: u64,
        size: u64,
        permission: Permission,
    ) -> Result<&mut Mem64, String> {
        //if size == 0 {
        //    return Err(format!("map size cannot be 0"));
        //}

        if self.get_mem_by_addr_mut(base).is_some() {
            return Err(format!("this map address 0x{:x} already exists!", base));
        }

        if self.exists_mapname(name) {
            self.show_maps();
            return Err(format!("this map name {} already exists!", name));
        }

        let mut mem = Mem64::default();
        mem.set_name(name);
        mem.set_base(base);
        mem.set_size(size);
        mem.set_permission(permission);

        let base_key = self.mem_slab.insert(mem);
        self.name_map.insert(name.to_string(), base_key);
        self.maps.insert(base, base_key);
        // Creating a map can reuse a freed slab slot, leaving stale TLB entries
        // that still point at the recycled key — flush so lookups re-resolve.
        self.tlb.get_mut().flush();
        Ok(self.mem_slab.get_mut(base_key).unwrap())
    }

    fn read_scalar<T: LittleEndianScalar>(&self, addr: u64, kind: ScalarKind) -> Option<T> {
        let end_addr = addr + T::SIZE as u64 - 1;
        let banzai = self.banzai;
        match self.get_mem_by_addr(addr) {
            Some(mem) if mem.inside(end_addr) && mem.can_read() => {
                crate::maps::scalar::read_le(mem.read_bytes(addr, T::SIZE))
            }
            None if banzai => {
                log::warn!(
                    "Reading {} from unmapped region at 0x{:x}",
                    kind.label(),
                    addr
                );
                None
            }
            _ => None,
        }
    }

    fn write_scalar<T: LittleEndianScalar>(
        &mut self,
        addr: u64,
        value: T,
        kind: ScalarKind,
    ) -> bool {
        let end_addr = addr + T::SIZE as u64 - 1;
        match self.get_mem_by_addr_mut(addr) {
            Some(mem) if mem.inside(end_addr) && mem.can_write() => {
                // Stack buffer — avoids a heap allocation per memory write
                // (this is the hot path; `to_le_vec` here dominated CPU via
                // malloc/free churn).
                let mut buf = [0u8; 16];
                value.write_le(&mut buf);
                mem.write_bytes(addr, &buf[..T::SIZE]);
                true
            }
            Some(mem) => {
                // Lives in a mapped region but the write is rejected — either
                // the range spills past the map or the map is read-only. Show
                // the map name so the operator can tell which section the
                // caller was targeting.
                let name = mem.get_name();
                let base = mem.get_base();
                let size = mem.size() as u64;
                let perm = if mem.can_write() { "RW" } else { "RO" };
                log::warn!(
                    "Writing {} to non-writable/oob region at 0x{:x} [{} 0x{:x}..0x{:x} {}]",
                    kind.label(),
                    addr,
                    name,
                    base,
                    base + size,
                    perm,
                );
                false
            }
            None => {
                log::warn!(
                    "Writing {} to unmapped region at 0x{:x} (no map covers this address)",
                    kind.label(),
                    addr
                );
                false
            }
        }
    }

    pub fn write_byte(&mut self, addr: u64, value: u8) -> bool {
        self.write_scalar(addr, value, ScalarKind::Byte)
    }

    pub fn read_byte(&self, addr: u64) -> Option<u8> {
        self.read_scalar(addr, ScalarKind::Byte)
    }

    pub fn read_f64(&self, addr: u64) -> Option<f64> {
        self.read_qword(addr).map(f64::from_bits)
    }

    pub fn read_f32(&self, addr: u64) -> Option<f32> {
        self.read_dword(addr).map(f32::from_bits)
    }

    pub fn write_f64(&mut self, addr: u64, value: f64) -> bool {
        self.write_qword(addr, value.to_bits())
    }

    pub fn write_f32(&mut self, addr: u64, value: f32) -> bool {
        self.write_dword(addr, value.to_bits())
    }

    pub fn write_qword(&mut self, addr: u64, value: u64) -> bool {
        self.write_scalar(addr, value, ScalarKind::Qword)
    }

    pub fn write_dword(&mut self, addr: u64, value: u32) -> bool {
        self.write_scalar(addr, value, ScalarKind::Dword)
    }

    pub fn write_word(&mut self, addr: u64, value: u16) -> bool {
        self.write_scalar(addr, value, ScalarKind::Word)
    }

    pub fn write_bytes_slice(&mut self, addr: u64, data: &[u8]) -> bool {
        self.write_bytes(addr, data)
    }

    pub fn write_bytes(&mut self, addr: u64, data: &[u8]) -> bool {
        if data.is_empty() {
            return true;
        }

        let end_addr = addr + data.len() as u64 - 1;

        // Fast path: if all data fits in a single memory map, use bulk copy
        match self.get_mem_by_addr_mut(addr) {
            Some(mem) if mem.inside(end_addr) && mem.can_write() => {
                mem.write_bytes(addr, data);
                return true;
            }
            Some(_) => {
                // Data spans multiple maps, fall through to byte-by-byte
            }
            None => {
                log::warn!("Writing bytes to unmapped region at 0x{:x}", addr);
                return false;
            }
        }

        // Slow path: write byte by byte to handle boundary crossings
        for (i, &byte) in data.iter().enumerate() {
            if !self.write_byte(addr + i as u64, byte) {
                return false;
            }
        }

        true
    }

    pub fn write_128bits_le(&mut self, addr: u64, value: u128) -> bool {
        self.write_scalar(addr, value, ScalarKind::Oword)
    }

    pub fn write_128bits_be(&mut self, addr: u64, value: u128) -> bool {
        self.write_bytes(addr, &value.to_be_bytes())
    }

    pub fn read_128bits_be(&self, addr: u64) -> Option<u128> {
        let b = self.try_read_bytes(addr, 16)?;
        let arr: [u8; 16] = b.try_into().ok()?;
        Some(u128::from_be_bytes(arr))
    }

    pub fn read_128bits_le(&self, addr: u64) -> Option<u128> {
        self.read_scalar(addr, ScalarKind::Oword)
    }

    pub fn read_qword(&self, addr: u64) -> Option<u64> {
        self.read_scalar(addr, ScalarKind::Qword)
    }

    pub fn read_dword(&self, addr: u64) -> Option<u32> {
        self.read_scalar(addr, ScalarKind::Dword)
    }

    pub fn read_word(&self, addr: u64) -> Option<u16> {
        self.read_scalar(addr, ScalarKind::Word)
    }

    pub fn get_mem_ref(&self, name: &str) -> &Mem64 {
        self.get_map_by_name(name)
            .expect("incorrect memory map name")
    }

    // deprecated
    pub fn get_mem(&self, name: &str) -> &Mem64 {
        self.get_map_by_name(name)
            .expect("incorrect memory map name")
    }

    pub fn get_mem_mut(&mut self, name: &str) -> &mut Mem64 {
        self.get_map_by_name_mut(name)
            .expect("incorrect memory map name")
    }

    #[inline(always)]
    pub fn get_mem2(&mut self, name: &str) -> Option<&mut Mem64> {
        self.get_map_by_name_mut(name)
    }

    #[inline(always)]
    pub fn get_mem_by_addr_mut(&mut self, addr: u64) -> Option<&mut Mem64> {
        let tlb_entry_mut = self.tlb.get_mut().get_entry_of_mut(addr, 0);
        let mem_key = tlb_entry_mut.get_mem();
        match self.mem_slab.get(mem_key) {
            Some(mem) => {
                if mem.inside(addr) {
                    return self.mem_slab.get_mut(tlb_entry_mut.mem64); // Clone the &Mem64
                }
            }
            _ => {
                tlb_entry_mut.invalidate();
            } // Remove the tlb entry
        };

        // TLB miss now search in the maps
        let mem_key_option = self
            .maps
            .range(..=addr)
            .next_back()
            .map(|(_start_addr, &key)| key);

        let mem_key = mem_key_option?;
        let mem_ref_mut = self.mem_slab.get_mut(mem_key)?;
        if !mem_ref_mut.inside(addr) {
            return None;
        }

        // Update TLB
        tlb_entry_mut.lpf = LPF_OF(addr);
        tlb_entry_mut.mem64 = mem_key;

        // Return back the memref
        Some(mem_ref_mut)
    }

    #[inline(always)]
    pub fn get_mem_by_addr(&self, addr: u64) -> Option<&Mem64> {
        let mut binding = self.tlb.borrow_mut();
        let entry = binding.get_entry_of(addr, 0);

        let mem_key = entry.get_mem();
        match self.mem_slab.get(mem_key) {
            Some(mem)
                if mem.inside(addr) => {
                    return Some(mem); // Clone the &Mem64
                }
            _ => (), // TLB miss now search in maps
        };

        let mem_key_option = self.maps.range(..=addr).next_back().map(|(_k, &v)| v);

        let mem_key = mem_key_option?; // Return None if not found

        let mem_ref = self.mem_slab.get(mem_key)?;
        if !mem_ref.inside(addr) {
            return None;
        }

        // --- Update TLB ---
        let tlb_entry_mut = binding.get_entry_of_mut(addr, 0);
        tlb_entry_mut.lpf = LPF_OF(addr);
        tlb_entry_mut.mem64 = mem_key;
        Some(mem_ref)
    }

    pub fn print_maps_keyword(&self, kw: &str) {
        log::trace!("--- maps ---");
        for (mem_name, base) in self.name_map.iter() {
            let mem = self.get_map_by_name(mem_name).unwrap();
            let k = mem_name;

            let n = if k.len() < 20 { 20 - k.len() } else { 1 };
            let mut spcs: String = String::new();
            for i in 0..n {
                spcs.push(' ');
            }
            if k.contains(kw) {
                log::trace!(
                    "{}{}0x{:x} - 0x{:x} ({})",
                    k,
                    spcs,
                    mem.get_base(),
                    mem.get_bottom(),
                    mem.size()
                );
            }
        }
        log::trace!("memory usage: {} bytes", self.size());
        log::trace!("---");
    }

    pub fn print_maps(&self) {
        println!("print_maps");
        log::trace!("--- maps ---");
        for (mem_name, base) in self.name_map.iter() {
            let mem = self.get_map_by_name(mem_name).unwrap();
            let k = mem_name;

            let n = if k.len() < 20 { 20 - k.len() } else { 1 };
            let mut spcs: String = String::new();
            for i in 0..n {
                spcs.push(' ');
            }
            log::trace!(
                "{}{}0x{:x} - 0x{:x} ({})",
                k,
                spcs,
                mem.get_base(),
                mem.get_bottom(),
                mem.size()
            );
        }
        log::trace!("memory usage: {} bytes", self.size());
        log::trace!("---");
    }

    #[inline(always)]
    pub fn get_addr_base(&self, addr: u64) -> Option<u64> {
        self.get_mem_by_addr(addr).map(|mem| mem.get_base())
    }

    /// Resolve the **allocation base** for `NtFreeVirtualMemory(MEM_RELEASE)` when the guest passes
    /// an interior pointer, the exclusive end (`addr == Mem64::bottom`), or when `get_addr_base`
    /// misses due to edge cases — scans `alloc_*` maps (see `trace_LdrInitializeThunk` / Ldr heap).
    pub fn alloc_region_base_for_free(&self, addr: u64) -> Option<u64> {
        if let Some(b) = self.get_addr_base(addr) {
            return Some(b);
        }
        if addr > 0 {
            if let Some(b) = self.get_addr_base(addr - 1) {
                return Some(b);
            }
        }
        for (_, mem) in self.mem_slab.iter() {
            let name = mem.get_name();
            if !name.starts_with("alloc_") {
                continue;
            }
            if mem.inside(addr) || (addr > 0 && mem.inside(addr - 1)) {
                return Some(mem.get_base());
            }
            if addr == mem.get_bottom() {
                return Some(mem.get_base());
            }
        }
        None
    }

    /// Find the PE image (base, size_of_image) that contains `addr`.
    /// Iterates maps named `*.pe`, reads the PE optional-header `SizeOfImage`,
    /// and checks whether `addr` falls within `[base, base+size_of_image)`.
    pub fn find_pe_image_info(&self, addr: u64) -> Option<(u64, u64)> {
        for name in self.name_map.keys() {
            if !name.ends_with(".pe") {
                continue;
            }
            if let Some(pe_map) = self.get_map_by_name(name) {
                let pe_base = pe_map.get_base();
                let pe_hdr_off = self.read_dword(pe_base + 0x3c).unwrap_or(0) as u64;
                if pe_hdr_off == 0 {
                    continue;
                }
                let size_of_image =
                    self.read_dword(pe_base + pe_hdr_off + 0x50).unwrap_or(0) as u64;
                if size_of_image == 0 {
                    continue;
                }
                if addr >= pe_base && addr < pe_base + size_of_image {
                    return Some((pe_base, size_of_image));
                }
            }
        }
        None
    }

    /// Return the base address of the first mapped region that starts **after** `addr`,
    /// or `None` if no such region exists.  Used by `NtQueryVirtualMemory` to compute
    /// the size of a free (MEM_FREE) virtual-address range.
    pub fn next_mapped_addr(&self, addr: u64) -> Option<u64> {
        self.maps
            .range(addr.saturating_add(1)..)
            .next()
            .map(|(&start, _)| start)
    }

    #[inline(always)]
    pub fn is_mapped(&self, addr: u64) -> bool {
        self.get_mem_by_addr(addr).is_some()
    }

    #[inline(always)]
    pub fn is_allocated(&self, addr: u64) -> bool {
        self.get_mem_by_addr(addr).is_some()
    }

    #[inline(always)]
    pub fn is_valid_ptr(&self, addr: u64) -> bool {
        addr > 0 && self.get_mem_by_addr(addr).is_some()
    }

    #[inline(always)]
    pub fn show_addr_names(&self, addr: u64) {
        self.get_mem_by_addr(addr).map(|mem| mem.get_name());
    }

    #[inline(always)]
    pub fn get_addr_name(&self, addr: u64) -> Option<&str> {
        self.get_mem_by_addr(addr).map(|mem| mem.get_name())
    }

    #[inline(always)]
    pub fn get_addr_name_mut(&mut self, addr: u64) -> Option<&str> {
        self.get_mem_by_addr(addr).map(|mem| mem.get_name())
    }

    pub fn size(&self) -> usize {
        let mut sz: usize = 0;
        for (_, mem) in self.mem_slab.iter() {
            sz += mem.size();
        }
        sz
    }

    pub fn overlaps(&self, addr: u64, sz: u64) -> bool {
        for a in addr..addr + sz {
            if self.is_mapped(a) {
                return true;
            }
        }
        false
    }

    pub fn show_allocs(&self) {
        for (_, mem) in self.mem_slab.iter() {
            let name = mem.get_name();
            if name.starts_with("alloc_") || name.starts_with("valloc_") {
                log::trace!(
                    "{} 0x{:x} - 0x{:x} ({})",
                    name,
                    mem.get_base(),
                    mem.get_bottom(),
                    mem.size()
                );
            }
        }
    }

    pub fn show_maps(&self) {
        for (_, mem) in self.mem_slab.iter() {
            let name = mem.get_name();
            log::trace!(
                "{} 0x{:x} - 0x{:x} ({})",
                name,
                mem.get_base(),
                mem.get_bottom(),
                mem.size()
            );
        }
    }

    pub fn free(&mut self, name: &str) {
        let id = self
            .name_map
            .get(name)
            .unwrap_or_else(|| panic!("map name {} not found", name));
        let mem = self.mem_slab.get_mut(*id).unwrap();
        mem.clear();
        self.maps.remove(&mem.get_base());
        self.mem_slab.remove(*id);
        self.tlb.borrow_mut().flush();
        self.name_map.remove(name);
    }

    pub fn dealloc(&mut self, addr: u64) {
        let mem_key = match self.maps.get(&addr) {
            Some(key) => key,
            None => {
                log::trace!("dealloc: non mapped address 0x{:x}", addr);
                return;
            }
        };
        let mem = self.mem_slab.get_mut(*mem_key).unwrap();
        self.name_map.remove(mem.get_name());
        mem.clear();
        self.mem_slab.remove(*mem_key);
        self.tlb.borrow_mut().flush();
        self.maps.remove(&addr);
    }

    pub fn map(&mut self, name: &str, sz: u64, permission: Permission) -> u64 {
        let addr = self.alloc(sz).expect("emu.maps.map(sz) cannot allocate");
        self.create_map(name, addr, sz, permission)
            .expect("emu.maps.map(sz) cannot create map");
        addr
    }

    pub fn map_lib(&mut self, name: &str, sz: u64, permission: Permission) -> u64 {
        let addr = self.alloc(sz).expect("emu.maps.map(sz) cannot allocate");
        if self.is_64bits {
            let addr = self
                .lib64_alloc(sz)
                .expect("emu.maps.map_lib(sz) cannot allocate");
        } else {
            let addr = self
                .lib32_alloc(sz)
                .expect("emu.maps.map_lib(sz) cannot allocate");
        }
        self.create_map(name, addr, sz, permission)
            .expect("emu.maps.map_lib(sz) cannot create map");
        addr
    }

    pub fn lib64_alloc(&self, sz: u64) -> Option<u64> {
        self._alloc(sz, constants::LIBS64_MIN, constants::LIBS64_MAX, true)
    }

    pub fn lib32_alloc(&self, sz: u64) -> Option<u64> {
        self._alloc(sz, constants::LIBS32_MIN, constants::LIBS32_MAX, true)
    }

    // this alloc return an address but you have to map it
    pub fn alloc(&self, sz: u64) -> Option<u64> {
        if self.is_64bits {
            self._alloc(sz, constants::ALLOC64_MIN, constants::ALLOC64_MAX, false)
        } else {
            self._alloc(sz, constants::ALLOC32_MIN, constants::ALLOC32_MAX, false)
        }
    }

    fn _alloc(&self, mut sz: u64, bottom: u64, top: u64, lib: bool) -> Option<u64> {
        /*
         *  params:
         *    sz: size to allocate, this number will be aligned.
         *    bottom: minimum address to allocate
         *    top: max address
         *    lib: allocating a library?
         *  vars:
         *    prev: is an aligned address, start with bottom and iterates every map bottom.
         *    base: base address of specific map.
         */

        let mut prev: u64 = self.align_up(bottom, Self::DEFAULT_ALIGNMENT);
        let debug = false;

        if sz > self.max_alloc_size {
            sz = self.max_alloc_size;
        }

        // Round up size to alignment
        sz = self.align_up(sz, Self::DEFAULT_ALIGNMENT);

        if debug {
            log::trace!("allocating {} bytes from 0x{:x} to 0x{:x}", sz, bottom, top);
        }

        // Here we assume that we go from the bottom to the most
        for mem_key in self.maps.values() {
            let mem = self.mem_slab.get(*mem_key).unwrap();
            let base = mem.get_base();

            if lib && base < bottom {
                if debug {
                    log::trace!("skipping: 0x{:x}", base);
                }
                continue;
            }

            if debug {
                log::trace!("base: 0x{:x} prev: 0x{:x} sz: 0x{:x}", base, prev, sz);
            }
            if prev > base {
                // we shouldn't care about this we just skip this memory region
                continue;
                // panic!("alloc error prev:0x{:x} > base:0x{:x}", prev, base);
            }
            if debug {
                log::trace!("space: 0x{:x}", base - prev);
            }
            if (base - prev) > sz {
                if debug {
                    log::trace!("space found: 0x{:x}", prev);
                }
                return Some(prev);
            }

            prev = self.align_up(mem.get_bottom(), Self::DEFAULT_ALIGNMENT);
        }

        if top < prev {
            prev = self.align_up(top, Self::DEFAULT_ALIGNMENT);
        }
        if top - prev > sz {
            if debug {
                log::trace!("space found: 0x{:x} sz:{}", prev, sz);
            }
            return Some(prev);
        }

        log::trace!("no space found");
        None
    }

    fn align_up(&self, addr: u64, align: u64) -> u64 {
        (addr + (align - 1)) & !(align - 1)
    }

    fn align_down(&self, addr: u64, align: u64) -> u64 {
        addr & !(align - 1)
    }

    pub fn save_all_allocs(&mut self, path: String) {
        for (_, mem) in self.mem_slab.iter() {
            if mem.get_name().to_string().starts_with("alloc_") {
                let mut ppath = path.clone();
                ppath.push('/');
                ppath.push_str(mem.get_name());
                ppath.push_str(".bin");
                mem.save(mem.get_base(), mem.size(), ppath);
            }
        }
    }

    pub fn save_all(&self, path: String) {
        for (_, mem) in self.mem_slab.iter() {
            let mut ppath = path.clone();
            ppath.push('/');
            ppath.push_str(&format!("{:08x}-{}", mem.get_base(), mem.get_name()));
            ppath.push_str(".bin");
            mem.save(mem.get_base(), mem.size(), ppath);
        }
    }

    pub fn save(&mut self, addr: u64, size: u64, filename: String) {
        //TODO: return a boolean or option.
        match self.get_mem_by_addr_mut(addr) {
            Some(m) => {
                m.save(addr, size as usize, filename);
            }
            None => {
                log::trace!("this address is not mapped.");
            }
        }
    }

    pub fn mem_test(&self) -> bool {
        for (_, mem1) in self.mem_slab.iter() {
            let name1 = mem1.get_name();

            for (_, mem2) in self.mem_slab.iter() {
                let name2 = mem2.get_name();

                if name1 != name2 {
                    for addr1 in mem1.get_base()..mem1.get_bottom() {
                        if mem2.inside(addr1) {
                            log::trace!("/!\\ {} overlaps with {}", name1, name2);
                            log::trace!(
                                "/!\\ 0x{:x}-0x{:x} vs 0x{:x}-0x{:x}",
                                mem1.get_base(),
                                mem1.get_bottom(),
                                mem2.get_base(),
                                mem2.get_bottom()
                            );
                            return false;
                        }
                    }
                }
            }

            if (mem1.get_base() + (mem1.size() as u64)) != mem1.get_bottom() {
                log::trace!("/!\\ memory bottom dont match, mem: {}", name1);
                return false;
            }
        }

        true
    }
}
