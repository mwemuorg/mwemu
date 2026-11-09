use crate::windows::constants::exceptions::*;

use crate::{
    console::Console,
    emu::Emu,
    exception::handlers::{self, HandlerKind},
    exception::types::{ExceptionRecord, ExceptionType, MemoryAccess},
    maps::mem64::Permission,
    threading::context::ArchThreadState,
};

impl Emu {
    pub fn veh(&self) -> u64 {
        match &self.threads[self.current_thread_id].arch {
            ArchThreadState::X86 { veh, .. } => veh.first().copied().unwrap_or(0),
            _ => unreachable!("veh() called on aarch64 emu"),
        }
    }

    pub fn veh_list(&self) -> &[u64] {
        match &self.threads[self.current_thread_id].arch {
            ArchThreadState::X86 { veh, .. } => veh.as_slice(),
            _ => unreachable!("veh_list() called on aarch64 emu"),
        }
    }

    pub fn set_veh(&mut self, value: u64) {
        match &mut self.threads[self.current_thread_id].arch {
            ArchThreadState::X86 { veh, .. } => {
                veh.clear();
                if value != 0 {
                    veh.push(value);
                }
            }
            _ => unreachable!("set_veh() called on aarch64 emu"),
        }
    }

    pub fn add_veh(&mut self, first: bool, handler: u64) {
        match &mut self.threads[self.current_thread_id].arch {
            ArchThreadState::X86 { veh, .. } => {
                if first {
                    veh.insert(0, handler);
                } else {
                    veh.push(handler);
                }
            }
            _ => unreachable!("add_veh() called on aarch64 emu"),
        }
    }

    pub fn remove_veh(&mut self, handler: u64) {
        match &mut self.threads[self.current_thread_id].arch {
            ArchThreadState::X86 { veh, .. } => veh.retain(|&h| h != handler),
            _ => unreachable!("remove_veh() called on aarch64 emu"),
        }
    }

    pub fn uef(&self) -> u64 {
        match &self.threads[self.current_thread_id].arch {
            ArchThreadState::X86 { uef, .. } => *uef,
            _ => unreachable!("uef() called on aarch64 emu"),
        }
    }

    pub fn set_uef(&mut self, value: u64) {
        match &mut self.threads[self.current_thread_id].arch {
            ArchThreadState::X86 { uef, .. } => *uef = value,
            _ => unreachable!("set_uef() called on aarch64 emu"),
        }
    }

    pub fn eh_ctx(&self) -> u64 {
        match &self.threads[self.current_thread_id].arch {
            ArchThreadState::X86 { eh_ctx, .. } => *eh_ctx,
            _ => unreachable!("eh_ctx() called on aarch64 emu"),
        }
    }

    pub fn set_eh_ctx(&mut self, value: u64) {
        match &mut self.threads[self.current_thread_id].arch {
            ArchThreadState::X86 { eh_ctx, .. } => *eh_ctx = value,
            _ => unreachable!("set_eh_ctx() called on aarch64 emu"),
        }
    }

    pub fn seh(&self) -> u64 {
        match &self.threads[self.current_thread_id].arch {
            ArchThreadState::X86 { seh, .. } => *seh,
            _ => unreachable!("seh() called on aarch64 emu"),
        }
    }

    pub fn set_seh(&mut self, value: u64) {
        match &mut self.threads[self.current_thread_id].arch {
            ArchThreadState::X86 { seh, .. } => *seh = value,
            _ => unreachable!("set_seh() called on aarch64 emu"),
        }
    }

    /// Walk the .pdata (Exception directory) of the PE64 module containing
    /// `rip` to find a language-specific exception handler via
    /// RUNTIME_FUNCTION / UNWIND_INFO.
    fn find_x64_seh_handler(&self, rip: u64) -> Option<u64> {
        let (pe_base, _) = self.maps.find_pe_image_info(rip)?;

        let e_lfanew = self.maps.read_dword(pe_base + 0x3C)? as u64;

        // NumberOfRvaAndSizes must include the exception directory (index 3).
        let num_rva_sizes = self.maps.read_dword(pe_base + e_lfanew + 0x84)?;
        if num_rva_sizes < 4 {
            return None;
        }

        // DataDirectory[3] = Exception (.pdata)
        let exc_dir_rva = self.maps.read_dword(pe_base + e_lfanew + 0xA0)? as u64;
        let exc_dir_size = self.maps.read_dword(pe_base + e_lfanew + 0xA4)? as u64;

        if exc_dir_rva == 0 || exc_dir_size < 12 {
            return None;
        }

        let pdata_base = pe_base + exc_dir_rva;
        let num_entries = exc_dir_size / 12;
        let rip_rva = rip.checked_sub(pe_base)? as u32;

        // Binary search the sorted RUNTIME_FUNCTION array.
        let mut lo: u64 = 0;
        let mut hi: u64 = num_entries;
        let mut found_unwind_rva: Option<u32> = None;

        while lo < hi {
            let mid = lo + (hi - lo) / 2;
            let entry_addr = pdata_base + mid * 12;
            let begin = self.maps.read_dword(entry_addr)?;
            let end = self.maps.read_dword(entry_addr + 4)?;

            if rip_rva < begin {
                hi = mid;
            } else if rip_rva >= end {
                lo = mid + 1;
            } else {
                found_unwind_rva = Some(self.maps.read_dword(entry_addr + 8)?);
                break;
            }
        }

        let mut unwind_rva = found_unwind_rva?;

        const MAX_CHAIN_DEPTH: u32 = 16;

        for _ in 0..MAX_CHAIN_DEPTH {
            let ui_addr = pe_base + unwind_rva as u64;
            let version_flags = self.maps.read_byte(ui_addr)?;
            let flags = (version_flags >> 3) & 0x1F;
            let count_of_codes = self.maps.read_byte(ui_addr + 2)? as u64;

            // Handler/chain data sits after the unwind codes, 4-byte aligned.
            let codes_end = ui_addr + 4 + count_of_codes * 2;
            let handler_offset = (codes_end + 3) & !3;

            if flags & UNW_FLAG_CHAININFO != 0 {
                unwind_rva = self.maps.read_dword(handler_offset + 8)?;
                continue;
            }

            if flags & UNW_FLAG_EHANDLER != 0 {
                let handler_rva = self.maps.read_dword(handler_offset)? as u64;
                return Some(pe_base + handler_rva);
            }

            return None;
        }

        log::warn!(
            "x64 SEH: UNWIND_INFO chain depth exceeded at rip 0x{:x}",
            rip
        );
        None
    }

    /// Trigger an exception while preserving the diagnostic hook interface.
    pub fn exception(&mut self, ex_type: ExceptionType) {
        let record = ExceptionRecord::new(ex_type, self.pc());
        self.dispatch_exception(ex_type, record);
    }

    pub(crate) fn memory_exception(
        &mut self,
        ex_type: ExceptionType,
        address: u64,
        size: usize,
        access: MemoryAccess,
    ) {
        let permission = match access {
            MemoryAccess::Read => Permission::READ,
            MemoryAccess::Write => Permission::WRITE,
            MemoryAccess::Execute => Permission::EXECUTE,
        };
        let fault_address = self
            .maps
            .first_inaccessible_address(address, size, permission)
            .unwrap_or(address);
        let record = ExceptionRecord::access_violation(self.pc(), fault_address, access);
        self.dispatch_exception(ex_type, record);
    }

    fn dispatch_exception(&mut self, ex_type: ExceptionType, mut record: ExceptionRecord) {
        self.fault_count += 1;
        if let Some(mut hook) = self.hooks.hook_on_exception.take() {
            let handle = hook(self, self.pc(), ex_type);
            self.hooks.hook_on_exception = Some(hook);
            if !handle {
                log::trace!("cancelled exception handling from hook");
                return;
            }
        }
        if self.cfg.arch.is_aarch64() {
            log::error!(
                "unhandled aarch64 exception at 0x{:x}: {:?}",
                self.pc(),
                ex_type
            );
            self.stop();
            return;
        }
        record.nested = handlers::active_record(self).unwrap_or(0);
        self.dispatch_vectored_exception(&record, 0);
    }

    pub(crate) fn dispatch_vectored_exception(&mut self, record: &ExceptionRecord, index: usize) {
        if let Some(&address) = self.veh_list().get(index) {
            self.call_exception_handler(record, address, HandlerKind::Veh, index);
        } else {
            self.dispatch_structured_exception(record);
        }
    }

    fn dispatch_structured_exception(&mut self, record: &ExceptionRecord) {
        if self.cfg.is_x64() {
            if let Some(address) = self.find_x64_seh_handler(record.address) {
                self.call_exception_handler(record, address, HandlerKind::Seh, 0);
            } else {
                self.dispatch_unhandled_exception(record);
            }
            return;
        }
        if self.seh() == 0 || self.seh() == EXCEPTION_CHAIN_END as u64 {
            self.dispatch_unhandled_exception(record);
            return;
        }
        let Some(address) = self.maps.read_dword(self.seh() + size_of::<u32>() as u64) else {
            log::error!("unreadable x86 SEH registration at 0x{:x}", self.seh());
            self.stop();
            return;
        };
        if self.maps.read_dword(self.seh()).is_none() {
            log::error!("invalid x86 SEH registration at 0x{:x}", self.seh());
            self.stop();
            return;
        }
        if !self.running_script {
            let console = Console::new();
            console.print("jump the exception pointer (y/n)?");
            if console.cmd() != "y" {
                return;
            }
        }
        self.call_exception_handler(record, address as u64, HandlerKind::Seh, 0);
    }

    pub(crate) fn dispatch_unhandled_exception(&mut self, record: &ExceptionRecord) {
        if self.uef() != 0 {
            self.call_exception_handler(record, self.uef(), HandlerKind::Uef, 0);
        } else {
            log::error!(
                "unhandled exception 0x{:x} at 0x{:x}",
                record.code,
                record.address
            );
            self.stop();
        }
    }

    fn call_exception_handler(
        &mut self,
        record: &ExceptionRecord,
        address: u64,
        kind: HandlerKind,
        veh_index: usize,
    ) {
        if self
            .maps
            .get_mem_by_addr(address)
            .is_none_or(|mem| mem.read_code(address, 1).is_empty())
        {
            log::error!("inaccessible exception handler at 0x{:x}", address);
            self.stop();
            return;
        }
        handlers::enter_record(self, record, kind, veh_index);
        if !self.process_terminated {
            self.regs_mut().rip = address;
            self.force_reload = true;
        }
    }
}
