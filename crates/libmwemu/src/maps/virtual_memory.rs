//! Allocation identity and page state shared by virtual-memory APIs.
use crate::maps::Maps;
use crate::maps::mem64::Permission;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

pub const PAGE_SIZE: u64 = 0x1000;
pub const ALLOCATION_GRANULARITY: u64 = 0x10000;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MemoryError {
    InvalidParameter,
    Conflict,
    Unmapped,
    NotCommitted,
    NoMemory,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum MemoryKind {
    Private,
    Mapped,
    Image,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PageState {
    pub committed: bool,
    pub protection: u32,
    pub permission: Permission,
}

impl PageState {
    pub fn reserved() -> Self {
        Self {
            committed: false,
            protection: 0,
            permission: Permission::NONE,
        }
    }

    pub fn committed(protection: u32, permission: Permission) -> Self {
        Self {
            committed: true,
            protection,
            permission,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Allocation {
    pub base: u64,
    pub size: u64,
    pub initial_protection: u32,
    pub kind: MemoryKind,
    pub maximum_permission: Option<Permission>,
    states: BTreeMap<u64, PageState>,
}

impl Allocation {
    pub fn new(base: u64, size: u64, protection: u32, kind: MemoryKind, state: PageState) -> Self {
        Self {
            base,
            size,
            initial_protection: protection,
            kind,
            maximum_permission: None,
            states: BTreeMap::from([(base, state)]),
        }
    }

    pub fn end(&self) -> u64 {
        self.base + self.size
    }

    pub fn contains(&self, address: u64, size: u64) -> bool {
        address >= self.base
            && address
                .checked_add(size)
                .is_some_and(|end| end <= self.end())
    }

    /// Missing page metadata is treated as reserved, without access permissions.
    pub fn state(&self, address: u64) -> PageState {
        self.states
            .range(..=address)
            .next_back()
            .map_or_else(PageState::reserved, |(_, state)| *state)
    }

    pub fn runs(&self, address: u64, size: u64) -> Vec<(u64, u64, PageState)> {
        let end = address + size;
        let mut cursor = address;
        let mut state = self.state(address);
        let mut runs = Vec::new();
        for (&next, &value) in self.states.range((address + 1)..end) {
            runs.push((cursor, next - cursor, state));
            cursor = next;
            state = value;
        }
        runs.push((cursor, end - cursor, state));
        runs
    }

    pub(crate) fn set_state(&mut self, address: u64, size: u64, value: PageState) {
        let end = address + size;
        let after = self.state(end);
        self.states.retain(|key, _| *key < address || *key >= end);
        self.states.insert(address, value);
        if end < self.end() {
            self.states.insert(end, after);
        }
        let mut previous = None;
        self.states.retain(|_, value| {
            let keep = previous != Some(*value);
            previous = Some(*value);
            keep
        });
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MemoryInfo {
    pub base: u64,
    pub size: u64,
    pub allocation_base: u64,
    pub allocation_protection: u32,
    pub state: Option<PageState>,
    pub kind: MemoryKind,
}

pub fn align_up(value: u64, alignment: u64) -> Option<u64> {
    value
        .checked_add(alignment - 1)
        .map(|n| n & !(alignment - 1))
}

pub fn page_range(address: u64, size: u64) -> Result<(u64, u64), MemoryError> {
    if size == 0 {
        return Err(MemoryError::InvalidParameter);
    }
    let start = address & !(PAGE_SIZE - 1);
    let end = address
        .checked_add(size)
        .and_then(|n| align_up(n, PAGE_SIZE))
        .ok_or(MemoryError::InvalidParameter)?;
    Ok((start, end - start))
}

pub fn protection_for(permission: Permission) -> u32 {
    match permission.to_flags() {
        (_, true, true) => 0x40,
        (true, false, true) => 0x20,
        (false, false, true) => 0x10,
        (_, true, false) => 0x04,
        (true, false, false) => 0x02,
        _ => 0x01,
    }
}

impl Maps {
    pub fn allocation_at(&self, address: u64) -> Option<&Allocation> {
        self.allocations
            .range(..=address)
            .next_back()
            .map(|(_, a)| a)
            .filter(|a| address < a.end())
    }

    pub fn register_allocation(&mut self, allocation: Allocation) {
        self.allocations.insert(allocation.base, allocation);
    }

    pub fn reserve_private(
        &mut self,
        base: u64,
        size: u64,
        protection: u32,
        permission: Permission,
        commit: bool,
    ) -> Result<(), MemoryError> {
        let state = if commit {
            PageState::committed(protection, permission)
        } else {
            PageState::reserved()
        };
        let name = self.unique_map_name(&format!("alloc_{base:x}"));
        self.create_map(&name, base, size, state.permission)
            .map_err(|_| MemoryError::Conflict)?;
        self.register_allocation(Allocation::new(
            base,
            size,
            protection,
            MemoryKind::Private,
            state,
        ));
        Ok(())
    }

    fn checked_allocation(&self, address: u64, size: u64) -> Result<Allocation, MemoryError> {
        let end = address
            .checked_add(size)
            .ok_or(MemoryError::InvalidParameter)?;
        if size == 0 {
            return Err(MemoryError::InvalidParameter);
        }
        if let Some(a) = self.allocation_at(address) {
            return if a.contains(address, size) {
                Ok(a.clone())
            } else {
                Err(MemoryError::Conflict)
            };
        }
        let mem = self.get_mem_by_addr(address).ok_or(MemoryError::Unmapped)?;
        let (base, length, kind) = self.find_pe_image_info(address).map_or(
            (mem.get_base(), mem.size() as u64, MemoryKind::Private),
            |(base, length)| (base, length, MemoryKind::Image),
        );
        if address < base || end > base + length {
            return Err(MemoryError::Conflict);
        }
        let protection = protection_for(mem.permission());
        let mut allocation = Allocation::new(base, length, protection, kind, PageState::reserved());
        let mut cursor = base;
        while cursor < base + length {
            if let Some(region) = self.get_mem_by_addr(cursor) {
                let count = (base + length).min(region.get_bottom()) - cursor;
                for (at, len, permissions) in region.permission_runs(cursor, count) {
                    allocation.set_state(
                        at,
                        len,
                        PageState::committed(protection_for(permissions), permissions),
                    );
                }
                cursor += count;
            } else {
                cursor = self
                    .next_mapped_addr(cursor)
                    .unwrap_or(base + length)
                    .min(base + length);
            }
        }
        Ok(allocation)
    }

    fn set_page_state(
        &mut self,
        allocation: &mut Allocation,
        address: u64,
        size: u64,
        state: PageState,
        zero: bool,
    ) -> Result<(), MemoryError> {
        let end = address
            .checked_add(size)
            .ok_or(MemoryError::InvalidParameter)?;
        let mut cursor = address;
        // Validate the complete backing range before mutation.
        while cursor < end {
            let mem = self.get_mem_by_addr(cursor).ok_or(MemoryError::Unmapped)?;
            cursor = end.min(mem.get_bottom());
        }
        cursor = address;
        while cursor < end {
            let mem = self
                .get_mem_by_addr_mut(cursor)
                .ok_or(MemoryError::Unmapped)?;
            let len = end.min(mem.get_bottom()) - cursor;
            if zero {
                mem.zero_backing(cursor, len as usize);
            }
            mem.set_permission_range(cursor, len, state.permission);
            cursor += len;
        }
        allocation.set_state(address, size, state);
        self.note_code_write(address, size);
        Ok(())
    }

    pub fn commit_memory(
        &mut self,
        address: u64,
        size: u64,
        protection: u32,
        permission: Permission,
    ) -> Result<(), MemoryError> {
        let mut allocation = self.checked_allocation(address, size)?;
        if allocation.kind != MemoryKind::Private {
            return Err(MemoryError::InvalidParameter);
        }
        if !self.can_access_range(address, size as usize, Permission::NONE) {
            return Err(MemoryError::Unmapped);
        }
        for (at, len, state) in allocation.runs(address, size) {
            self.set_page_state(
                &mut allocation,
                at,
                len,
                PageState::committed(protection, permission),
                !state.committed,
            )?;
        }
        self.register_allocation(allocation);
        Ok(())
    }

    pub fn decommit_memory(&mut self, address: u64, size: u64) -> Result<(), MemoryError> {
        let mut allocation = self.checked_allocation(address, size)?;
        if allocation.kind != MemoryKind::Private {
            return Err(MemoryError::InvalidParameter);
        }
        self.set_page_state(&mut allocation, address, size, PageState::reserved(), true)?;
        self.register_allocation(allocation);
        Ok(())
    }

    pub fn validate_protection(
        &self,
        address: u64,
        size: u64,
        permission: Permission,
    ) -> Result<u32, MemoryError> {
        let allocation = self.checked_allocation(address, size)?;
        if allocation
            .maximum_permission
            .is_some_and(|maximum| !maximum.contains(permission))
        {
            return Err(MemoryError::InvalidParameter);
        }
        if allocation
            .runs(address, size)
            .iter()
            .any(|(_, _, state)| !state.committed)
        {
            return Err(MemoryError::NotCommitted);
        }
        if !self.can_access_range(address, size as usize, Permission::NONE) {
            return Err(MemoryError::Unmapped);
        }
        Ok(allocation.state(address).protection)
    }

    pub fn protect_memory(
        &mut self,
        address: u64,
        size: u64,
        protection: u32,
        permission: Permission,
    ) -> Result<u32, MemoryError> {
        let previous = self.validate_protection(address, size, permission)?;
        let mut allocation = self.checked_allocation(address, size)?;
        self.set_page_state(
            &mut allocation,
            address,
            size,
            PageState::committed(protection, permission),
            false,
        )?;
        self.register_allocation(allocation);
        Ok(previous)
    }

    pub fn release_memory(&mut self, address: u64) -> Result<(), MemoryError> {
        let allocation = self.checked_allocation(address, 1)?;
        if address != allocation.base || allocation.kind != MemoryKind::Private {
            return Err(MemoryError::InvalidParameter);
        }
        self.unmap_range(allocation.base, allocation.size);
        self.allocations.remove(&allocation.base);
        Ok(())
    }

    pub fn query_memory(&self, address: u64) -> MemoryInfo {
        let base = address & !(PAGE_SIZE - 1);
        if let Some(allocation) = self.allocation_at(address) {
            let state = allocation.state(address);
            let end = allocation
                .states
                .range((address + 1)..)
                .next()
                .map_or(allocation.end(), |(&next, _)| next);
            return MemoryInfo {
                base,
                size: end - base,
                allocation_base: allocation.base,
                allocation_protection: allocation.initial_protection,
                state: Some(state),
                kind: allocation.kind,
            };
        }
        if let Some(mem) = self.get_mem_by_addr(address) {
            let (allocation_base, kind) = self
                .find_pe_image_info(address)
                .map_or((mem.get_base(), MemoryKind::Private), |(base, _)| {
                    (base, MemoryKind::Image)
                });
            let (at, len, permission) = mem.permission_runs(address, mem.get_bottom() - address)[0];
            return MemoryInfo {
                base,
                size: at + len - base,
                allocation_base,
                allocation_protection: protection_for(mem.permission()),
                state: Some(PageState::committed(protection_for(permission), permission)),
                kind,
            };
        }
        let end = self.next_mapped_addr(base).unwrap_or(if self.is_64bits {
            0x8000_0000_0000
        } else {
            0x1_0000_0000
        });
        MemoryInfo {
            base,
            size: end.saturating_sub(base),
            allocation_base: 0,
            allocation_protection: 0,
            state: None,
            kind: MemoryKind::Private,
        }
    }

    pub fn find_free_range(&self, size: u64, alignment: u64, bottom: u64, top: u64) -> Option<u64> {
        if size == 0 || !alignment.is_power_of_two() {
            return None;
        }
        let mut cursor = align_up(bottom, alignment)?;
        for (&base, &id) in self.maps.range(..top) {
            let mem = self.mem_slab.get(id)?;
            if mem.get_bottom() <= cursor {
                continue;
            }
            if cursor.checked_add(size)? <= base {
                return Some(cursor);
            }
            cursor = align_up(mem.get_bottom(), alignment)?;
        }
        (cursor.checked_add(size)? <= top).then_some(cursor)
    }

    pub fn find_free_range_top_down(
        &self,
        size: u64,
        alignment: u64,
        bottom: u64,
        top: u64,
    ) -> Option<u64> {
        if size == 0 || !alignment.is_power_of_two() {
            return None;
        }
        let mut end = top;
        for (&base, &id) in self.maps.range(..top).rev() {
            let mem = self.mem_slab.get(id)?;
            if mem.get_bottom() <= bottom {
                break;
            }
            let candidate = end.checked_sub(size)? & !(alignment - 1);
            if candidate >= mem.get_bottom().max(bottom) {
                return Some(candidate);
            }
            end = base;
        }
        let candidate = end.checked_sub(size)? & !(alignment - 1);
        (candidate >= bottom).then_some(candidate)
    }
}
