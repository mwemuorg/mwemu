use crate::maps::mem64::{Mem64, Permission};
use std::borrow::Cow;

impl Mem64 {
    /// Protection at an address. Byte storage and allocation identity do not
    /// change when a subrange receives different permissions.
    #[inline]
    pub fn permission_at(&self, addr: u64) -> Permission {
        if self.permission_overrides.is_empty() {
            return self.permission;
        }
        self.permission_overrides
            .range(..=addr)
            .next_back()
            .map_or(self.permission, |(_, permission)| *permission)
    }

    /// Change a mapped subrange; callers such as VirtualProtect align pages.
    /// Boundaries describe runs, so an untouched large allocation stays cheap.
    pub fn set_permission_range(&mut self, addr: u64, len: u64, permission: Permission) -> bool {
        let Some(end) = addr.checked_add(len) else {
            return false;
        };
        if addr < self.base_addr || end > self.bottom_addr || len == 0 {
            return false;
        }
        if addr == self.base_addr && end == self.bottom_addr {
            self.set_permission(permission);
            return true;
        }
        let after = self.permission_at(end);
        self.permission_overrides
            .retain(|start, _| *start < addr || *start >= end);
        self.permission_overrides.insert(addr, permission);
        if end < self.bottom_addr {
            self.permission_overrides.insert(end, after);
        }
        let mut previous = self.permission;
        self.permission_overrides.retain(|_, value| {
            let changed = value.bits() != previous.bits();
            previous = *value;
            changed
        });
        true
    }

    /// Number of consecutive accessible bytes, bounded by this map and limit.
    pub fn accessible_prefix(&self, addr: u64, limit: usize, required: Permission) -> usize {
        self.accessible_prefix_matching(addr, limit, |p| p.contains(required))
    }

    fn accessible_prefix_matching(
        &self,
        addr: u64,
        limit: usize,
        allowed: impl Fn(Permission) -> bool,
    ) -> usize {
        if !self.inside(addr) || !allowed(self.permission_at(addr)) {
            return 0;
        }
        let end = addr.saturating_add(limit as u64).min(self.bottom_addr);
        for (&boundary, &permission) in self.permission_overrides.range(addr..end) {
            if !allowed(permission) {
                return (boundary - addr) as usize;
            }
        }
        (end - addr) as usize
    }

    #[inline]
    pub fn can_read_range(&self, addr: u64, len: usize) -> bool {
        self.can_access(addr, len, Permission::READ)
    }

    #[inline]
    pub fn can_write_range(&self, addr: u64, len: usize) -> bool {
        self.can_access(addr, len, Permission::WRITE)
    }

    #[inline]
    pub fn can_execute_range(&self, addr: u64, len: usize) -> bool {
        self.can_access(addr, len, Permission::EXECUTE)
    }

    #[inline]
    fn can_access(&self, addr: u64, len: usize, required: Permission) -> bool {
        let Some(end) = addr.checked_add(len as u64) else {
            return false;
        };
        if addr < self.base_addr || end > self.bottom_addr {
            return false;
        }
        if len == 0 {
            return true;
        }
        if self.permission_overrides.is_empty() {
            return self.permission.contains(required);
        }
        self.accessible_prefix(addr, len, required) == len
    }

    /// Whether any byte has the requested permission (cache invalidation).
    #[inline]
    pub fn has_permission(&self, required: Permission) -> bool {
        self.permission_at(self.base_addr).contains(required)
            || self
                .permission_overrides
                .values()
                .any(|p| p.contains(required))
    }

    /// Consecutive runs clipped to a mapped range, for unmapping and snapshots.
    pub fn permission_runs(&self, addr: u64, len: u64) -> Vec<(u64, u64, Permission)> {
        let start = addr.max(self.base_addr);
        let end = addr.saturating_add(len).min(self.bottom_addr);
        if start >= end {
            return Vec::new();
        }
        let mut runs = Vec::new();
        let mut cursor = start;
        let mut permission = self.permission_at(start);
        for (&boundary, &next) in self.permission_overrides.range((start + 1)..end) {
            runs.push((cursor, boundary - cursor, permission));
            cursor = boundary;
            permission = next;
        }
        runs.push((cursor, end - cursor, permission));
        runs
    }

    /// Accept executable bytes without granting guest data READ access.
    /// Retain the emulator's existing ability to execute readable pages.
    #[inline]
    pub fn read_code(&self, addr: u64, limit: usize) -> Cow<'_, [u8]> {
        let len = self.accessible_prefix_matching(addr, limit, |p| p.can_read() || p.can_execute());
        if len == 0 {
            return Cow::Borrowed(&[]);
        }
        let start = (addr - self.base_addr) as usize;
        self.backing_bytes(start, len)
    }
}
