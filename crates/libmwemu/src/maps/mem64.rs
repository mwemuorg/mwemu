mod protection;
mod storage;

use crate::emu::emu_context;
use crate::maps::scalar::{LittleEndianScalar, ScalarTrace, read_le};
use crate::maps::shared::{SharedStorage, SharedView};
use bytemuck::cast_slice;
use md5;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::collections::BTreeMap;
use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use std::io::SeekFrom;
use std::io::Write;
use std::io::prelude::*;

/// Memory region permission flags (read/write/execute).
#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Permission(u8);

// Bit flags for permissions
impl Permission {
    pub const NONE: Permission = Permission(0b000);
    pub const READ: Permission = Permission(0b001);
    pub const WRITE: Permission = Permission(0b010);
    pub const EXECUTE: Permission = Permission(0b100);

    // Common combinations
    pub const READ_WRITE: Permission = Permission(0b011);
    pub const READ_EXECUTE: Permission = Permission(0b101);
    pub const WRITE_EXECUTE: Permission = Permission(0b110);
    pub const READ_WRITE_EXECUTE: Permission = Permission(0b111);

    /// Create a new Permission from raw bits
    #[inline]
    pub const fn from_bits(bits: u8) -> Self {
        Permission(bits & 0b111) // Only use the lower 3 bits
    }

    /// Get the raw bits
    #[inline]
    pub const fn bits(&self) -> u8 {
        self.0
    }

    /// Create permission from individual flags
    #[inline]
    pub fn from_flags(read: bool, write: bool, execute: bool) -> Self {
        let mut bits = 0;
        if read {
            bits |= 0b001;
        }
        if write {
            bits |= 0b010;
        }
        if execute {
            bits |= 0b100;
        }
        Permission(bits)
    }

    /// Check if read access is allowed
    #[inline]
    pub const fn can_read(&self) -> bool {
        (self.0 & 0b001) != 0
    }

    /// Check if write access is allowed
    #[inline]
    pub const fn can_write(&self) -> bool {
        (self.0 & 0b010) != 0
    }

    /// Check if execute access is allowed
    #[inline]
    pub const fn can_execute(&self) -> bool {
        (self.0 & 0b100) != 0
    }

    /// Check if any access is allowed
    #[inline]
    pub const fn is_accessible(&self) -> bool {
        self.0 != 0
    }

    /// Add a permission (union)
    #[inline]
    pub const fn add(&self, other: Permission) -> Self {
        Permission(self.0 | other.0)
    }

    /// Remove a permission
    pub const fn remove(&self, other: Permission) -> Self {
        Permission(self.0 & !other.0)
    }

    /// Check if this permission contains another permission
    #[inline]
    pub const fn contains(&self, other: Permission) -> bool {
        (self.0 & other.0) == other.0
    }

    /// Get individual permission flags
    #[inline]
    pub const fn to_flags(&self) -> (bool, bool, bool) {
        (self.can_read(), self.can_write(), self.can_execute())
    }
}

// Implement bitwise operations for Permission
impl std::ops::BitOr for Permission {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self {
        Permission(self.0 | rhs.0)
    }
}

impl std::ops::BitAnd for Permission {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        Permission(self.0 & rhs.0)
    }
}

impl std::ops::BitXor for Permission {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self {
        Permission(self.0 ^ rhs.0)
    }
}

impl std::ops::Not for Permission {
    type Output = Self;

    fn not(self) -> Self {
        Permission(!self.0 & 0b111)
    }
}

/// A single contiguous memory region with base address, permissions, and byte storage.
#[derive(Clone, Serialize, Deserialize)]
pub struct Mem64 {
    mem_name: String,
    base_addr: u64,
    bottom_addr: u64,
    permission: Permission,
    #[serde(default)]
    permission_overrides: BTreeMap<u64, Permission>,
    mem: Vec<u8>,
    #[serde(default)]
    view: Option<SharedView>,
    #[serde(skip)]
    shared: Option<SharedStorage>,
}

impl Default for Mem64 {
    fn default() -> Self {
        Mem64 {
            mem_name: "".to_string(),
            base_addr: 0,
            bottom_addr: 0,
            permission: Permission::from_flags(true, true, false),
            permission_overrides: BTreeMap::new(),
            mem: Vec::new(),
            view: None,
            shared: None,
        }
    }
}

impl Mem64 {
    pub fn new(
        mem_name: String,
        base_addr: u64,
        bottom_addr: u64,
        mem: Vec<u8>,
        permission: Permission,
    ) -> Mem64 {
        Mem64 {
            mem_name,
            base_addr,
            bottom_addr,
            permission,
            permission_overrides: BTreeMap::new(),
            mem,
            view: None,
            shared: None,
        }
    }

    pub fn permission(&self) -> Permission {
        self.permission_at(self.base_addr)
    }

    /// Set new permissions
    pub fn set_permission(&mut self, permission: Permission) {
        self.permission = permission;
        self.permission_overrides.clear();
    }

    /// Add permissions
    pub fn add_permission(&mut self, permission: Permission) {
        self.permission = self.permission.add(permission);
        for value in self.permission_overrides.values_mut() {
            *value = value.add(permission);
        }
    }

    /// Remove permissions
    pub fn remove_permission(&mut self, permission: Permission) {
        self.permission = self.permission.remove(permission);
        for value in self.permission_overrides.values_mut() {
            *value = value.remove(permission);
        }
    }

    /// Check if read access is allowed for this memory region
    pub fn can_read(&self) -> bool {
        self.permission().can_read()
    }

    /// Check if write access is allowed for this memory region
    pub fn can_write(&self) -> bool {
        self.permission().can_write()
    }

    /// Check if execute access is allowed for this memory region
    pub fn can_execute(&self) -> bool {
        self.permission().can_execute()
    }

    pub fn clear(&mut self) {
        self.mem_name = "".to_string();
        self.base_addr = 0;
        self.bottom_addr = 0;
        self.mem.clear();
        self.view = None;
        self.shared = None;
        self.permission_overrides.clear();
    }

    #[inline(always)]
    pub fn get_name(&self) -> &str {
        self.mem_name.as_str()
    }

    #[inline(always)]
    pub fn set_name(&mut self, name: &str) {
        self.mem_name = name.to_string();
    }

    #[inline(always)]
    pub fn get_mem(&self) -> Vec<u8> {
        self.get_bytes().into_owned()
    }

    #[inline(always)]
    pub fn alloc(&mut self, amount: usize) {
        self.view = None;
        self.shared = None;
        self.mem = vec![0; amount];
        self.permission_overrides.clear();
    }

    pub fn extend(&mut self, amount: usize) {
        self.detach_shared();
        self.mem.resize(self.mem.len() + amount, 0);
        self.bottom_addr += amount as u64;
    }

    #[inline(always)]
    pub fn size(&self) -> usize {
        if self.view.is_some() {
            (self.bottom_addr - self.base_addr) as usize
        } else {
            self.mem.len()
        }
    }

    /// Drop the first `n` bytes so the map starts at `base + n` (munmap of a head).
    pub fn drop_front(&mut self, n: usize) {
        let permission = self.permission_at(self.base_addr + n as u64);
        if let Some(view) = self.view.as_mut() {
            view.offset += n;
        } else {
            self.mem.drain(..n);
        }
        self.base_addr += n as u64;
        self.permission = permission;
        self.permission_overrides
            .retain(|addr, _| *addr > self.base_addr);
    }

    /// Keep only the first `n` bytes (munmap of a tail).
    pub fn truncate(&mut self, n: usize) {
        if self.view.is_none() {
            self.mem.truncate(n);
        }
        self.bottom_addr = self.base_addr + n as u64;
        self.permission_overrides
            .retain(|addr, _| *addr < self.bottom_addr);
    }

    /// Cut the map at absolute address `at`, returning the bytes from there on
    /// (munmap of a hole: the caller re-creates the tail as its own map).
    pub fn split_off(&mut self, at: u64) -> Vec<u8> {
        let tail = if self.view.is_some() {
            self.backing_bytes(
                (at - self.base_addr) as usize,
                (self.bottom_addr - at) as usize,
            )
            .into_owned()
        } else {
            self.mem.split_off((at - self.base_addr) as usize)
        };
        self.bottom_addr = at;
        self.permission_overrides.retain(|addr, _| *addr < at);
        tail
    }

    #[inline(always)]
    pub fn get_base(&self) -> u64 {
        self.base_addr
    }

    #[inline(always)]
    pub fn get_bottom(&self) -> u64 {
        self.bottom_addr
    }

    pub fn memcpy(&mut self, ptr: &[u8], sz: usize) {
        if self.size() < sz {
            log::error!(
                "Try memcpy at mem but size bigger than allocated size: addr {}, size {}",
                self.base_addr,
                sz
            );
            return;
        }
        // TODO: log trace read write?
        self.write_backing(0, &ptr[..sz]);
    }

    #[inline]
    pub fn inside(&self, addr: u64) -> bool {
        addr >= self.base_addr && addr < self.bottom_addr
    }

    #[inline(always)]
    pub fn set_base(&mut self, base_addr: u64) {
        self.permission_overrides.clear();
        self.base_addr = base_addr;
        self.bottom_addr = base_addr;
    }

    #[inline(always)]
    pub fn update_base(&mut self, base_addr: u64) {
        let previous = self.base_addr;
        self.permission_overrides = std::mem::take(&mut self.permission_overrides)
            .into_iter()
            .map(|(addr, permission)| (base_addr + (addr - previous), permission))
            .collect();
        self.base_addr = base_addr;
    }

    pub fn set_bottom(&mut self, bottom_addr: u64) {
        self.bottom_addr = bottom_addr;
        let size = self.bottom_addr - self.base_addr;
        self.alloc(size as usize);
    }

    #[inline(always)]
    pub fn update_bottom(&mut self, bottom_addr: u64) {
        self.bottom_addr = bottom_addr;
    }

    pub fn try_set_size(&mut self, size: u64) -> Result<(), String> {
        let end = self
            .base_addr
            .checked_add(size)
            .ok_or("memory range overflow")?;
        let len = usize::try_from(size).map_err(|_| "memory size overflow")?;
        // One zeroed allocation: the allocator returns untouched zero pages.
        // Reserving and then `resize`-ing writes every byte instead, which for
        // a 1 MB stack costs milliseconds in a debug build.
        let bytes: Vec<u8> =
            bytemuck::try_zeroed_vec(len).map_err(|_| "backing allocation failed")?;
        self.view = None;
        self.shared = None;
        self.mem = bytes;
        self.bottom_addr = end;
        self.permission_overrides.clear();
        Ok(())
    }

    pub(crate) fn fill_backing(&mut self, addr: u64, value: u8, size: usize) {
        let offset = (addr - self.base_addr) as usize;
        if let (Some(view), Some(shared)) = (self.view, &self.shared) {
            shared.bytes.borrow_mut()[view.offset + offset..view.offset + offset + size]
                .fill(value);
            shared.dirty.set(true);
        } else {
            self.mem[offset..offset + size].fill(value);
        }
    }

    pub(crate) fn zero_backing(&mut self, addr: u64, size: usize) {
        self.fill_backing(addr, 0, size);
    }

    pub fn set_size(&mut self, size: u64) {
        self.bottom_addr = self.base_addr + size;
        self.alloc(size as usize);
    }

    #[inline(always)]
    pub fn build_addresses(&self, addr: u64, sz: usize) -> Vec<u64> {
        (addr..addr + sz as u64).collect()
    }

    #[inline(always)]
    pub fn read_from(&self, addr: u64) -> Cow<'_, [u8]> {
        self.read_bytes(addr, usize::MAX)
    }

    #[inline(always)]
    pub fn read_bytes(&self, addr: u64, sz: usize) -> Cow<'_, [u8]> {
        let len = self.accessible_prefix(addr, sz, Permission::READ);
        if len == 0 {
            return Cow::Borrowed(&[]);
        }
        let idx = (addr - self.base_addr) as usize;
        let r = self.backing_bytes(idx, len);
        if cfg!(feature = "log_mem_read") && emu_context::mem_trace_enabled() {
            log::trace!(
                "mem_trace: read_bytes: 0x{:x?} = {:x?}",
                self.build_addresses(addr, r.len()),
                r
            );
        }
        r
    }

    #[inline(always)]
    pub fn get_bytes(&self) -> Cow<'_, [u8]> {
        self.backing_bytes(0, self.size())
    }

    #[inline(always)]
    pub fn read_byte(&self, addr: u64) -> u8 {
        if !self.can_read_range(addr, 1) {
            if emu_context::mem_trace_enabled() {
                log::trace!("FAILED doesn't have permission: read_from: 0x{:x?}", addr);
            }
            {
                log::warn!("mem: read w/o permission at 0x{:x}", addr);
                return 0;
            }
        }

        let idx = (addr - self.base_addr) as usize;
        let r = self.with_backing(|bytes| bytes[idx]);
        if cfg!(feature = "log_mem_read") && emu_context::mem_trace_enabled() {
            log::trace!(
                "mem_trace: read_byte: 0x{:x?} = 0x{:x}",
                self.build_addresses(addr, 1),
                r
            );
        }
        r
    }

    #[inline(always)]
    pub fn read_word(&self, addr: u64) -> u16 {
        self.read_scalar(addr, ScalarTrace::ReadWord)
    }

    #[inline(always)]
    pub fn read_dword(&self, addr: u64) -> u32 {
        self.read_scalar(addr, ScalarTrace::ReadDword)
    }

    #[inline(always)]
    pub fn read_qword(&self, addr: u64) -> u64 {
        self.read_scalar(addr, ScalarTrace::ReadQword)
    }

    pub fn read_oword(&self, addr: u64) -> u128 {
        self.read_scalar(addr, ScalarTrace::ReadQword)
    }

    #[inline(always)]
    pub fn force_write_byte(&mut self, addr: u64, value: u8) {
        let idx = (addr - self.base_addr) as usize;
        self.write_backing(idx, &[value]);
        if cfg!(feature = "log_mem_write") && emu_context::mem_trace_enabled() {
            log::trace!(
                "mem: force_write_byte: 0x{:x?} = 0x{:x}",
                self.build_addresses(addr, 1),
                value
            );
        }
    }

    #[inline(always)]
    pub fn force_write_bytes(&mut self, addr: u64, bs: &[u8]) {
        let idx = (addr - self.base_addr) as usize;
        self.write_backing(idx, bs);
        if cfg!(feature = "log_mem_write") && emu_context::mem_trace_enabled() {
            log::trace!(
                "mem: force_write_bytes: 0x{:x?} = {:?}",
                self.build_addresses(addr, bs.len()),
                bs
            );
        }
    }

    #[inline(always)]
    pub fn force_write_word(&mut self, addr: u64, value: u16) {
        self.force_write_scalar(addr, value, ScalarTrace::ForceWriteWord);
    }

    #[inline(always)]
    pub fn force_write_dword(&mut self, addr: u64, value: u32) {
        self.force_write_scalar(addr, value, ScalarTrace::ForceWriteDword);
    }

    #[inline(always)]
    pub fn force_write_qword(&mut self, addr: u64, value: u64) {
        self.force_write_scalar(addr, value, ScalarTrace::ForceWriteQword);
    }

    #[inline(always)]
    pub fn force_write_oword(&mut self, addr: u64, value: u128) {
        self.force_write_scalar(addr, value, ScalarTrace::ForceWriteOword);
    }

    #[inline(always)]
    pub fn force_write_string(&mut self, addr: u64, s: &str) {
        let mut v = s.as_bytes().to_vec();
        v.push(0);
        self.force_write_bytes(addr, &v);

        if cfg!(feature = "log_mem_write") && emu_context::mem_trace_enabled() {
            log::trace!(
                "mem: force_write_string: 0x{:x?} = {:?}",
                self.build_addresses(addr, s.len() + 1),
                s
            );
        }
    }

    #[inline(always)]
    pub fn write_byte(&mut self, addr: u64, value: u8) {
        if !self.can_write_range(addr, 1) {
            {
                log::warn!("mem: write w/o permission at 0x{:x}", addr);
                return;
            }
        }

        let idx = (addr - self.base_addr) as usize;
        self.write_backing(idx, &[value]);
        if cfg!(feature = "log_mem_write") && emu_context::mem_trace_enabled() {
            log::trace!(
                "mem_trace: write_byte: 0x{:x?} = 0x{:x}",
                self.build_addresses(addr, 1),
                value
            );
        }
    }

    #[inline(always)]
    pub fn write_bytes(&mut self, addr: u64, bs: &[u8]) {
        if !self.can_write_range(addr, bs.len()) {
            {
                log::warn!("mem: write w/o permission at 0x{:x}", addr);
                return;
            }
        }

        let idx = (addr - self.base_addr) as usize;
        self.write_backing(idx, bs);
        if cfg!(feature = "log_mem_write") && emu_context::mem_trace_enabled() {
            log::trace!(
                "mem_trace: write_bytes: 0x{:x?} = {:x?}",
                self.build_addresses(addr, bs.len()),
                bs
            );
        }
    }

    #[inline(always)]
    pub fn write_word(&mut self, addr: u64, value: u16) {
        self.write_scalar(addr, value, ScalarTrace::WriteWord);
    }

    #[inline(always)]
    pub fn write_dword(&mut self, addr: u64, value: u32) {
        self.write_scalar(addr, value, ScalarTrace::WriteDword);
    }

    #[inline(always)]
    pub fn write_qword(&mut self, addr: u64, value: u64) {
        self.write_scalar(addr, value, ScalarTrace::WriteQword);
    }

    #[inline(always)]
    pub fn write_oword(&mut self, addr: u64, value: u128) {
        self.write_scalar(addr, value, ScalarTrace::WriteOword);
    }

    #[inline(always)]
    pub fn write_string(&mut self, addr: u64, s: &str) {
        let mut v = s.as_bytes().to_vec();
        if v.last() != Some(&0) {
            v.push(0);
        }
        self.write_bytes(addr, &v);

        if cfg!(feature = "log_mem_write") && emu_context::mem_trace_enabled() {
            log::trace!(
                "mem_trace: write_string: 0x{:x?} = {} ({:x?})",
                self.build_addresses(addr, v.len()),
                s,
                v
            );
        }
    }

    #[inline(always)]
    pub fn read_string(&self, addr: u64) -> String {
        if !self.can_read_range(addr, 1) {
            if emu_context::mem_trace_enabled() {
                log::trace!("FAILED doesn't have permission: read_from: 0x{:x?}", addr);
            }
            {
                log::warn!("mem: read w/o permission at 0x{:x}", addr);
                return String::new();
            }
        }

        // Scan the backing slice directly for the NUL terminator instead of a
        // per-byte `read_byte` (each of which is a bounds check + branch). This
        // is a very hot path (export-name / registry-name reads), so it avoids
        // the growing `Vec<u8>` too. The string is bounded by this map's end.
        const MAX_SIZE_STR: usize = 1_000_000;
        let start = (addr - self.base_addr) as usize;
        let avail = self.accessible_prefix(addr, MAX_SIZE_STR, Permission::READ);
        let region = self.backing_bytes(start, avail);
        let end = region.iter().position(|&b| b == 0).unwrap_or(avail);
        let s = &region[..end];
        if cfg!(feature = "log_mem_read") && emu_context::mem_trace_enabled() {
            log::trace!(
                "mem_trace: read_string: 0x{:x?} = {:x?}",
                self.build_addresses(addr, s.len() + 1),
                s
            );
        }
        String::from_utf8_lossy(s).into_owned()
    }

    #[inline(always)]
    pub fn write_wide_string(&mut self, addr: u64, s: &str) {
        let mut wide_string: Vec<u16> = s.encode_utf16().collect();
        if wide_string.last() != Some(&0) {
            wide_string.push(0);
        }
        let wide_string_byte_slice: &[u8] = cast_slice(&wide_string);
        self.write_bytes(addr, wide_string_byte_slice);

        if cfg!(feature = "log_mem_write") && emu_context::mem_trace_enabled() {
            log::trace!(
                "mem_trace: write_wide_string: 0x{:x?} = {} ({:x?})",
                self.build_addresses(addr, wide_string_byte_slice.len()),
                s,
                wide_string_byte_slice
            );
        }
    }

    fn read_scalar<T: LittleEndianScalar + Into<u128>>(&self, addr: u64, trace: ScalarTrace) -> T {
        if !self.can_read_range(addr, T::SIZE) {
            if emu_context::mem_trace_enabled() {
                log::trace!("FAILED doesn't have permission: read_from: 0x{:x?}", addr);
            }
            log::warn!("mem: read w/o permission at 0x{:x}, returning 0", addr);
            let zeros = vec![0u8; T::SIZE];
            return read_le(&zeros).expect("incorrect length");
        }

        // Bounds check: callers via `Maps::read_scalar` already verify this, but
        // direct `Mem64::read_word`/`read_dword`/`read_qword` callers don't —
        // and reading past the slice would panic. Treat out-of-range reads as
        // zero (matches common emulator semantics for uninitialised memory) so
        // a stray read at a section boundary doesn't kill the whole run.
        let idx = (addr - self.base_addr) as usize;
        if idx + T::SIZE > self.size() {
            log::warn!(
                "Mem64::read_scalar out-of-bounds: addr=0x{:x} idx={} +{} > len={} ({})",
                addr,
                idx,
                T::SIZE,
                self.size(),
                trace.label()
            );
            let zeros = vec![0u8; T::SIZE];
            return read_le(&zeros).expect("incorrect length");
        }
        let r: T = self
            .with_backing(|bytes| read_le(&bytes[idx..idx + T::SIZE]).expect("incorrect length"));
        if cfg!(feature = "log_mem_read") && emu_context::mem_trace_enabled() {
            log::trace!(
                "mem_trace: {}: 0x{:x?} = 0x{:x}",
                trace.label(),
                self.build_addresses(addr, T::SIZE),
                r.into()
            );
        }
        r
    }

    fn write_scalar<T: LittleEndianScalar + Into<u128>>(
        &mut self,
        addr: u64,
        value: T,
        trace: ScalarTrace,
    ) {
        if !self.can_write_range(addr, T::SIZE) {
            if emu_context::mem_trace_enabled() {
                log::trace!("FAILED doesn't have permission: write_from: 0x{:x?}", addr);
            }
            {
                log::warn!("mem: write w/o permission at 0x{:x}", addr);
                return;
            }
        }
        let idx = (addr - self.base_addr) as usize;
        let mut buf = [0u8; 16];
        value.write_le(&mut buf);
        self.write_backing(idx, &buf[..T::SIZE]);

        if cfg!(feature = "log_mem_write") && emu_context::mem_trace_enabled() {
            log::trace!(
                "mem_trace: {}: 0x{:x?} = 0x{:x}",
                trace.label(),
                self.build_addresses(addr, T::SIZE),
                value.into()
            );
        }
    }

    fn force_write_scalar<T: LittleEndianScalar + Into<u128>>(
        &mut self,
        addr: u64,
        value: T,
        trace: ScalarTrace,
    ) {
        let idx = (addr - self.base_addr) as usize;
        let mut buf = [0u8; 16];
        value.write_le(&mut buf);
        self.write_backing(idx, &buf[..T::SIZE]);

        if cfg!(feature = "log_mem_write") && emu_context::mem_trace_enabled() {
            log::trace!(
                "mem: {}: 0x{:x?} = 0x{:x}",
                trace.label(),
                self.build_addresses(addr, T::SIZE),
                value.into()
            );
        }
    }

    #[inline]
    pub fn read_wide_string(&self, addr: u64) -> String {
        if !self.can_read_range(addr, 2) {
            if emu_context::mem_trace_enabled() {
                log::trace!("FAILED doesn't have permission: read_from: 0x{:x?}", addr);
            }
            {
                log::warn!("mem: read w/o permission at 0x{:x}", addr);
                return String::new();
            }
        }

        let MAX_SIZE_STR = 1_000_000;
        let mut s: Vec<u16> = Vec::new();
        let mut idx = addr;
        while idx < addr + MAX_SIZE_STR {
            let b = self.read_word(idx);
            if b == 0 {
                break;
            }
            s.push(b);
            idx += 2;
        }
        if cfg!(feature = "log_mem_read") && emu_context::mem_trace_enabled() {
            log::trace!(
                "mem_trace: read_wide_string: 0x{:x?} = {:x?}",
                self.build_addresses(addr, s.len() + 1),
                s
            );
        }

        match String::from_utf16(&s) {
            Ok(s) => s,
            Err(_) => "".to_string(),
        }
    }

    pub fn read_wide_string_n(&self, addr: u64, max_chars: usize) -> String {
        if !self.can_read_range(addr, 2) {
            if emu_context::mem_trace_enabled() {
                log::trace!("FAILED doesn't have permission: read_from: 0x{:x?}", addr);
            }
            {
                log::warn!("mem: read w/o permission at 0x{:x}", addr);
                return String::new();
            }
        }

        let mut s: Vec<u16> = Vec::new();
        let mut idx = addr;
        for _ in 0..max_chars {
            let b = self.read_word(idx);
            if b == 0 {
                break;
            }
            s.push(b);
            idx += 2;
        }
        if cfg!(feature = "log_mem_read") && emu_context::mem_trace_enabled() {
            log::trace!(
                "mem_trace: read_wide_string_n: 0x{:x?} = {:x?}",
                self.build_addresses(addr, s.len() + 1),
                s
            );
        }
        String::from_utf16_lossy(&s)
    }

    pub fn print_bytes(&self) {
        log::trace!("---mem---");
        for b in self.get_bytes().iter() {
            print!("{}", b);
        }
        log::trace!("---");
    }

    pub fn print_dwords(&self) {
        self.print_dwords_from_to(self.get_base(), self.get_bottom());
    }

    pub fn print_dwords_from_to(&self, from: u64, to: u64) {
        log::trace!("---mem---");
        for addr in (from..to).step_by(4) {
            log::trace!("0x{:x}", self.read_dword(addr))
        }

        log::trace!("---");
    }

    pub fn md5(&self) -> md5::Digest {
        md5::compute(self.get_bytes())
    }

    pub fn load_at(&mut self, base_addr: u64) {
        if self.view.is_some() {
            return;
        }
        self.set_base(base_addr);
        let mut name: String = String::from(&self.mem_name);
        name.push_str(".bin");
        self.load(name.as_str());
    }

    pub fn load_chunk(&mut self, filename: &str, off: u64, sz: usize) -> bool {
        if self.view.is_some() {
            return false;
        }
        // log::trace!("loading chunk: {} {} {}", filename, off, sz);
        let mut f = match File::open(filename) {
            Ok(f) => f,
            Err(_) => {
                return false;
            }
        };
        f.seek(SeekFrom::Start(off));
        let mut reader = BufReader::new(&f);
        self.mem.clear();
        for i in 0..sz {
            self.mem.push(0);
        }
        if let Err(e) = reader.read_exact(&mut self.mem) {
            log::error!("cannot load chunk of file: {}", e);
            return false;
        }
        f.sync_all(); // thanks, Alberto Segura
        true
    }

    pub fn load(&mut self, filename: &str) -> bool {
        // log::trace!("loading map: {}", filename);
        let f = match File::open(filename) {
            Ok(f) => f,
            Err(_) => {
                return false;
            }
        };
        let len = match f.metadata() {
            Ok(m) => m.len(),
            Err(e) => {
                log::error!("cannot read metadata for '{}': {}", filename, e);
                return false;
            }
        };
        self.bottom_addr = self.base_addr + len;
        let mut reader = BufReader::new(&f);
        self.mem.clear();
        if let Err(e) = reader.read_to_end(&mut self.mem) {
            log::error!("cannot load map file '{}': {}", filename, e);
            return false;
        }
        f.sync_all(); // thanks, Alberto Segura
        true
    }

    pub fn save(&self, addr: u64, size: usize, filename: String) {
        let idx = (addr - self.base_addr) as usize;
        let sz2 = idx + size;
        if sz2 > self.size() {
            log::error!("size too big, map size is {}  sz2:{}", self.size(), sz2);
            return;
        }

        let mut f = match File::create(&filename) {
            Ok(f) => f,
            Err(e) => {
                log::error!("cannot create the file {}", e);
                return;
            }
        };

        let bytes = self.get_bytes();
        let Some(blob) = bytes.get(idx..sz2) else {
            log::error!(
                "save: invalid range {}..{} for mem len {}",
                idx,
                sz2,
                self.mem.len()
            );
            return;
        };

        match f.write_all(blob) {
            Ok(_) => log::debug!(
                "saved. addr: 0x{:x} size: {} filename: {}",
                addr,
                size,
                filename
            ),
            Err(_) => log::error!("couldn't save the file"),
        }

        let _ = f.sync_all();
    }

    pub fn save_all(&self, filename: &str) -> bool {
        let mut f = match File::create(filename) {
            Ok(f) => f,
            Err(e) => {
                return false;
            }
        };

        let bytes = self.get_bytes();
        let blob = bytes.as_ref();

        let res = f.write_all(blob).is_ok();

        let _ = f.sync_all();

        res
    }
}
