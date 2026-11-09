//! Breakpoint trait implementations for GDB stub
//!
//! This module implements the various breakpoint-related extensions
//! required by gdbstub.

use gdbstub::target::TargetResult;
use gdbstub::target::ext::breakpoints::{
    Breakpoints, HwWatchpoint, HwWatchpointOps, SwBreakpoint, SwBreakpointOps, WatchKind,
};

use super::target::{MwemuTarget32, MwemuTarget64, MwemuTargetAarch64};

// ========== 64-bit Target Implementations ==========

impl Breakpoints for MwemuTarget64<'_> {
    #[inline(always)]
    fn support_sw_breakpoint(&mut self) -> Option<SwBreakpointOps<'_, Self>> {
        Some(self)
    }

    #[inline(always)]
    fn support_hw_watchpoint(&mut self) -> Option<HwWatchpointOps<'_, Self>> {
        Some(self)
    }
}

impl SwBreakpoint for MwemuTarget64<'_> {
    fn add_sw_breakpoint(&mut self, addr: u64, _kind: usize) -> TargetResult<bool, Self> {
        log::debug!("GDB: Adding software breakpoint at 0x{:x}", addr);
        self.emu.bp.add_bp(addr);
        Ok(true)
    }

    fn remove_sw_breakpoint(&mut self, addr: u64, _kind: usize) -> TargetResult<bool, Self> {
        log::debug!("GDB: Removing software breakpoint at 0x{:x}", addr);
        Ok(self.emu.bp.remove_bp_by_addr(addr))
    }
}

impl HwWatchpoint for MwemuTarget64<'_> {
    fn add_hw_watchpoint(
        &mut self,
        addr: u64,
        _len: u64,
        kind: WatchKind,
    ) -> TargetResult<bool, Self> {
        log::debug!(
            "GDB: Adding hardware watchpoint at 0x{:x}, kind={:?}",
            addr,
            kind
        );
        match kind {
            WatchKind::Read => {
                self.emu.bp.add_bp_mem_read(addr);
            }
            WatchKind::Write => {
                self.emu.bp.add_bp_mem_write(addr);
            }
            WatchKind::ReadWrite => {
                self.emu.bp.add_bp_mem_read(addr);
                self.emu.bp.add_bp_mem_write(addr);
            }
        }
        Ok(true)
    }

    fn remove_hw_watchpoint(
        &mut self,
        addr: u64,
        _len: u64,
        kind: WatchKind,
    ) -> TargetResult<bool, Self> {
        log::debug!(
            "GDB: Removing hardware watchpoint at 0x{:x}, kind={:?}",
            addr,
            kind
        );
        let result = match kind {
            WatchKind::Read => self.emu.bp.remove_mem_read_by_addr(addr),
            WatchKind::Write => self.emu.bp.remove_mem_write_by_addr(addr),
            WatchKind::ReadWrite => {
                let r1 = self.emu.bp.remove_mem_read_by_addr(addr);
                let r2 = self.emu.bp.remove_mem_write_by_addr(addr);
                r1 || r2
            }
        };
        Ok(result)
    }
}

// ========== AArch64 Target Implementations ==========

impl Breakpoints for MwemuTargetAarch64<'_> {
    #[inline(always)]
    fn support_sw_breakpoint(&mut self) -> Option<SwBreakpointOps<'_, Self>> {
        Some(self)
    }

    #[inline(always)]
    fn support_hw_watchpoint(&mut self) -> Option<HwWatchpointOps<'_, Self>> {
        Some(self)
    }
}

impl SwBreakpoint for MwemuTargetAarch64<'_> {
    fn add_sw_breakpoint(&mut self, addr: u64, _kind: usize) -> TargetResult<bool, Self> {
        log::debug!("GDB: Adding software breakpoint at 0x{:x}", addr);
        self.emu.bp.add_bp(addr);
        Ok(true)
    }

    fn remove_sw_breakpoint(&mut self, addr: u64, _kind: usize) -> TargetResult<bool, Self> {
        log::debug!("GDB: Removing software breakpoint at 0x{:x}", addr);
        Ok(self.emu.bp.remove_bp_by_addr(addr))
    }
}

impl HwWatchpoint for MwemuTargetAarch64<'_> {
    fn add_hw_watchpoint(
        &mut self,
        addr: u64,
        _len: u64,
        kind: WatchKind,
    ) -> TargetResult<bool, Self> {
        log::debug!(
            "GDB: Adding hardware watchpoint at 0x{:x}, kind={:?}",
            addr,
            kind
        );
        match kind {
            WatchKind::Read => {
                self.emu.bp.add_bp_mem_read(addr);
            }
            WatchKind::Write => {
                self.emu.bp.add_bp_mem_write(addr);
            }
            WatchKind::ReadWrite => {
                self.emu.bp.add_bp_mem_read(addr);
                self.emu.bp.add_bp_mem_write(addr);
            }
        }
        Ok(true)
    }

    fn remove_hw_watchpoint(
        &mut self,
        addr: u64,
        _len: u64,
        kind: WatchKind,
    ) -> TargetResult<bool, Self> {
        log::debug!(
            "GDB: Removing hardware watchpoint at 0x{:x}, kind={:?}",
            addr,
            kind
        );
        let result = match kind {
            WatchKind::Read => self.emu.bp.remove_mem_read_by_addr(addr),
            WatchKind::Write => self.emu.bp.remove_mem_write_by_addr(addr),
            WatchKind::ReadWrite => {
                let r1 = self.emu.bp.remove_mem_read_by_addr(addr);
                let r2 = self.emu.bp.remove_mem_write_by_addr(addr);
                r1 || r2
            }
        };
        Ok(result)
    }
}

// ========== 32-bit Target Implementations ==========

impl Breakpoints for MwemuTarget32<'_> {
    #[inline(always)]
    fn support_sw_breakpoint(&mut self) -> Option<SwBreakpointOps<'_, Self>> {
        Some(self)
    }

    #[inline(always)]
    fn support_hw_watchpoint(&mut self) -> Option<HwWatchpointOps<'_, Self>> {
        Some(self)
    }
}

impl SwBreakpoint for MwemuTarget32<'_> {
    fn add_sw_breakpoint(&mut self, addr: u32, _kind: usize) -> TargetResult<bool, Self> {
        log::debug!("GDB: Adding software breakpoint at 0x{:x}", addr);
        self.emu.bp.add_bp(addr as u64);
        Ok(true)
    }

    fn remove_sw_breakpoint(&mut self, addr: u32, _kind: usize) -> TargetResult<bool, Self> {
        log::debug!("GDB: Removing software breakpoint at 0x{:x}", addr);
        Ok(self.emu.bp.remove_bp_by_addr(addr as u64))
    }
}

impl HwWatchpoint for MwemuTarget32<'_> {
    fn add_hw_watchpoint(
        &mut self,
        addr: u32,
        _len: u32,
        kind: WatchKind,
    ) -> TargetResult<bool, Self> {
        log::debug!(
            "GDB: Adding hardware watchpoint at 0x{:x}, kind={:?}",
            addr,
            kind
        );
        let addr = addr as u64;
        match kind {
            WatchKind::Read => {
                self.emu.bp.add_bp_mem_read(addr);
            }
            WatchKind::Write => {
                self.emu.bp.add_bp_mem_write(addr);
            }
            WatchKind::ReadWrite => {
                self.emu.bp.add_bp_mem_read(addr);
                self.emu.bp.add_bp_mem_write(addr);
            }
        }
        Ok(true)
    }

    fn remove_hw_watchpoint(
        &mut self,
        addr: u32,
        _len: u32,
        kind: WatchKind,
    ) -> TargetResult<bool, Self> {
        log::debug!(
            "GDB: Removing hardware watchpoint at 0x{:x}, kind={:?}",
            addr,
            kind
        );
        let addr = addr as u64;
        let result = match kind {
            WatchKind::Read => self.emu.bp.remove_mem_read_by_addr(addr),
            WatchKind::Write => self.emu.bp.remove_mem_write_by_addr(addr),
            WatchKind::ReadWrite => {
                let r1 = self.emu.bp.remove_mem_read_by_addr(addr);
                let r2 = self.emu.bp.remove_mem_write_by_addr(addr);
                r1 || r2
            }
        };
        Ok(result)
    }
}
