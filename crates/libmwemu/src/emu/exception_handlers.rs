use crate::windows::constants::exceptions::*;

use crate::{
    console::Console,
    context::context64::Context64,
    emu::Emu,
    exception::handlers::{self, HandlerKind},
    exception::types::{ExceptionRecord, ExceptionType, MemoryAccess},
    maps::mem64::Permission,
    threading::context::ArchThreadState,
};

fn ctx_reg_offset(index: u8) -> Option<u64> {
    const OFFSETS: [u64; 16] = [
        120, 128, 136, 144, 152, 160, 168, 176, 184, 192, 200, 208, 216, 224, 232, 240,
    ];
    OFFSETS.get(index as usize).copied()
}

impl Emu {
    pub fn veh(&self) -> u64 {
        self.veh_list().first().copied().unwrap_or(0)
    }

    pub fn veh_list(&self) -> &[u64] {
        match &self.threads[0].arch {
            ArchThreadState::X86 { veh, .. } => veh,
            _ => &[],
        }
    }

    pub fn set_veh(&mut self, value: u64) {
        if let ArchThreadState::X86 { veh, .. } = &mut self.threads[0].arch {
            veh.clear();
            self.veh_registration_ids.clear();
            if value != 0 {
                veh.push(value);
                self.veh_registration_ids.push(0);
            }
        }
    }

    pub fn add_veh(&mut self, first: bool, handler: u64) {
        self.insert_veh(first, handler, 0);
    }

    fn insert_veh(&mut self, first: bool, handler: u64, id: u32) {
        if let ArchThreadState::X86 { veh, .. } = &mut self.threads[0].arch {
            let index = if first { 0 } else { veh.len() };
            self.veh_registration_ids.resize(veh.len(), 0);
            veh.insert(index, handler);
            self.veh_registration_ids.insert(index, id);
        }
    }

    pub fn remove_veh(&mut self, handler: u64) {
        while let Some(i) = self.veh_list().iter().position(|&h| h == handler) {
            self.remove_veh_at(i);
        }
    }

    fn remove_veh_at(&mut self, index: usize) {
        if let ArchThreadState::X86 { veh, .. } = &mut self.threads[0].arch {
            self.veh_registration_ids.resize(veh.len(), 0);
            veh.remove(index);
            self.veh_registration_ids.remove(index);
        }
    }

    pub(crate) fn register_veh(&mut self, first: bool, handler: u64) -> u64 {
        if handler == 0 {
            return 0;
        }
        let Some(id) = self.last_veh_id.checked_add(1) else {
            return 0;
        };
        self.last_veh_id = id;
        self.insert_veh(first, handler, id);
        u64::from(id)
    }

    pub(crate) fn unregister_veh(&mut self, handle: u64) -> bool {
        if handle == 0 {
            return false;
        }
        let index = self
            .veh_registration_ids
            .iter()
            .position(|&id| u64::from(id) == handle)
            .or_else(|| {
                self.veh_list()
                    .iter()
                    .zip(&self.veh_registration_ids)
                    .position(|(&callback, &id)| id == 0 && callback == handle)
            });
        if let Some(i) = index {
            self.remove_veh_at(i);
            true
        } else {
            false
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

    /// Walk the call stack via .pdata to find a language-specific exception
    /// handler (UNW_FLAG_EHANDLER). Returns `(handler_va, establisher_frame)`.
    /// Resolve an address to its RUNTIME_FUNCTION range via .pdata.
    /// Returns `(begin_va, end_va)` of the function containing `addr`.
    pub fn resolve_function_range(&self, addr: u64) -> Option<(u64, u64)> {
        let (base, entry) = self.runtime_function_entry(addr)?;
        let begin = self.maps.read_dword(entry)?;
        let end = self.maps.read_dword(entry.checked_add(4)?)?;
        Some((
            base.checked_add(u64::from(begin))?,
            base.checked_add(u64::from(end))?,
        ))
    }

    pub(crate) fn runtime_function_entry(&self, addr: u64) -> Option<(u64, u64)> {
        let (pe_base, _) = self.maps.find_pe_image_info(addr)?;
        let e_lfanew = self.maps.read_dword(pe_base.checked_add(0x3C)?)? as u64;
        let num_rva_sizes = self
            .maps
            .read_dword(pe_base.checked_add(e_lfanew + 0x84)?)?;
        if num_rva_sizes < 4 {
            return None;
        }
        let exc_dir_rva = self
            .maps
            .read_dword(pe_base.checked_add(e_lfanew + 0xA0)?)? as u64;
        let exc_dir_size = self
            .maps
            .read_dword(pe_base.checked_add(e_lfanew + 0xA4)?)? as u64;
        if exc_dir_rva == 0 || exc_dir_size < 12 {
            return None;
        }
        let pdata_base = pe_base.checked_add(exc_dir_rva)?;
        let num_entries = exc_dir_size / 12;
        let addr_rva = u32::try_from(addr.checked_sub(pe_base)?).ok()?;
        let mut lo: u64 = 0;
        let mut hi: u64 = num_entries;
        while lo < hi {
            let mid = lo + (hi - lo) / 2;
            let entry_addr = pdata_base.checked_add(mid * 12)?;
            let begin = self.maps.read_dword(entry_addr)?;
            let end = self.maps.read_dword(entry_addr.checked_add(4)?)?;
            if addr_rva < begin {
                hi = mid;
            } else if addr_rva >= end {
                lo = mid + 1;
            } else {
                return Some((pe_base, entry_addr));
            }
        }
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

    pub(crate) fn dispatch_structured_exception(&mut self, record: &ExceptionRecord) {
        if self.cfg.is_x64() {
            if !self.dispatch_x64_seh(record) {
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

    /// Allocate guest-side EXCEPTION_RECORD + CONTEXT, populate them, and call
    /// `dispatch_x64_seh_frames` which walks frames calling each EHANDLER via
    /// `call64`. Returns true when a handler took the exception.
    fn dispatch_x64_seh(&mut self, record: &ExceptionRecord) -> bool {
        const BLOCK_SIZE: u64 = 0x1000;
        const RECORD_OFF: u64 = 0x10;
        const CONTEXT_OFF: u64 = 0x100;

        let Some(block) = self.maps.alloc(BLOCK_SIZE) else {
            return false;
        };
        let map_name = format!("seh_disp_{:x}", block);
        if self
            .maps
            .create_map(&map_name, block, BLOCK_SIZE, Permission::READ_WRITE)
            .is_err()
        {
            return false;
        }

        let record_addr = block + RECORD_OFF;
        let context_addr = block + CONTEXT_OFF;

        record.save(&mut self.maps, record_addr, true);
        let ctx = Context64::new(self.regs());
        ctx.save(context_addr, &mut self.maps);
        let eflags = self.flags().dump();
        self.maps.write_dword(
            context_addr + Context64::FLAGS_OFFSET,
            CONTEXT_CONTROL | CONTEXT_INTEGER,
        );
        self.maps
            .write_dword(context_addr + Context64::EFLAGS_OFFSET, eflags);

        let handled = self.dispatch_x64_seh_frames(record_addr, context_addr);

        if handled {
            let saved = *self.regs();
            let mut ctx = Context64::new(&saved);
            ctx.load(context_addr, &mut self.maps);
            ctx.sync(self.regs_mut());
        }

        self.maps.free(&map_name);
        handled
    }

    /// Core x64 SEH dispatch: walk frames from the CONTEXT's RIP/RSP, call each
    /// language-specific handler via `call64`, and return true when one returns
    /// ContinueExecution (or performs a non-local transfer via RtlUnwindEx).
    /// Both `record_addr` and `context_addr` must point to populated structures
    /// in guest memory. Uses a scratch CONTEXT for frame advancement so that
    /// all UWOP codes (including SET_FPREG) are handled correctly.
    ///
    /// EstablisherFrame is computed by applying UWOP codes *before* calling the
    /// handler, matching real Windows RtlVirtualUnwind behaviour.
    ///
    /// Two-pass strategy:
    ///   Pass 1 — EHANDLER only (correct per Windows spec).
    ///   Pass 2 — UHANDLER fallback.  Some binaries (Enigma/Delphi) register
    ///            only UHANDLER entries and rely on a VEH to dispatch exceptions.
    ///            When no VEH is registered in the emulator the UHANDLER pass
    ///            fills that role.
    pub(crate) fn dispatch_x64_seh_frames(&mut self, record_addr: u64, context_addr: u64) -> bool {
        const WORK_SIZE: u64 = 0x600;
        const SCRATCH_CTX_OFF: u64 = 0;
        const DISP_CTX_OFF: u64 = 0x500;

        let Some(work) = self.maps.alloc(WORK_SIZE) else {
            return false;
        };
        let work_map = format!("seh_work_{:x}", work);
        if self
            .maps
            .create_map(&work_map, work, WORK_SIZE, Permission::READ_WRITE)
            .is_err()
        {
            return false;
        }

        let scratch_ctx = work + SCRATCH_CTX_OFF;
        let disp_ctx = work + DISP_CTX_OFF;

        // Pass 1: EHANDLER only (standard Windows first-pass dispatch).
        let handled = self.walk_seh_frames(
            record_addr,
            context_addr,
            scratch_ctx,
            disp_ctx,
            UNW_FLAG_EHANDLER,
        );

        if !handled {
            // Pass 2: UHANDLER fallback — simulates VEH dispatch for binaries
            // that only register termination handlers (e.g. Delphi/Enigma).
            let handled = self.walk_seh_frames(
                record_addr,
                context_addr,
                scratch_ctx,
                disp_ctx,
                UNW_FLAG_UHANDLER,
            );
            self.maps.free(&work_map);
            return handled;
        }

        self.maps.free(&work_map);
        handled
    }

    fn walk_seh_frames(
        &mut self,
        record_addr: u64,
        context_addr: u64,
        scratch_ctx: u64,
        disp_ctx: u64,
        flag_mask: u8,
    ) -> bool {
        const MAX_FRAMES: usize = 256;
        const MAX_CHAIN: usize = 16;

        // Copy the original CONTEXT into the scratch CONTEXT for walking.
        for off in (0..Context64::INTEGER_CONTROL_SIZE).step_by(8) {
            let val = self.maps.read_qword(context_addr + off).unwrap_or(0);
            self.maps.write_qword(scratch_ctx + off, val);
        }

        let mut handled = false;
        let saved_rip = self.regs().rip;
        let saved_rsp = self.regs().rsp;

        for _ in 0..MAX_FRAMES {
            let current_rip = self
                .maps
                .read_qword(scratch_ctx + Context64::RIP_OFFSET)
                .unwrap_or(0);
            let current_rsp = self
                .maps
                .read_qword(scratch_ctx + Context64::RSP_OFFSET)
                .unwrap_or(0);

            if current_rip == 0 {
                break;
            }

            log::trace!(
                "  frame walk: rip=0x{:x} rsp=0x{:x}",
                current_rip,
                current_rsp,
            );

            let Some((pe_base, entry_addr)) = self.runtime_function_entry(current_rip) else {
                let Some(ret) = self.maps.read_qword(current_rsp) else {
                    log::trace!(
                        "  frame walk: cannot read stack at 0x{:x} — stop",
                        current_rsp
                    );
                    break;
                };
                log::trace!("  frame walk: no .pdata, pop ret=0x{:x}", ret);
                self.maps
                    .write_qword(scratch_ctx + Context64::RSP_OFFSET, current_rsp + 8);
                self.maps
                    .write_qword(scratch_ctx + Context64::RIP_OFFSET, ret);
                continue;
            };

            let Some(unwind_rva_raw) = self.maps.read_dword(entry_addr + 8) else {
                break;
            };
            let ui_addr = pe_base + unwind_rva_raw as u64;

            let machframe = match self.apply_frame_codes_on_context(ui_addr, scratch_ctx) {
                Some(mf) => mf,
                None => break,
            };

            let establisher_frame = self
                .maps
                .read_qword(scratch_ctx + Context64::RSP_OFFSET)
                .unwrap_or(0);

            // Follow CHAININFO and look for a handler matching flag_mask.
            let mut chain_rva = unwind_rva_raw as u64;
            let mut found_handler: Option<(u64, u64)> = None;

            for _ in 0..MAX_CHAIN {
                let chain_ui = pe_base + chain_rva;
                let Some(version_flags) = self.maps.read_byte(chain_ui) else {
                    break;
                };
                let flags = (version_flags >> 3) & 0x1F;
                let Some(count_of_codes) = self.maps.read_byte(chain_ui + 2) else {
                    break;
                };
                let codes_end = chain_ui + 4 + count_of_codes as u64 * 2;
                let handler_offset = (codes_end + 3) & !3;

                if flags & UNW_FLAG_CHAININFO != 0 {
                    if let Some(rva) = self.maps.read_dword(handler_offset + 8) {
                        chain_rva = rva as u64;
                        continue;
                    }
                    break;
                }

                if flags & flag_mask != 0
                    && let Some(handler_rva) = self.maps.read_dword(handler_offset)
                {
                    let handler_data = handler_offset + 4;
                    found_handler = Some((pe_base + handler_rva as u64, handler_data));
                }
                break;
            }

            if let Some((handler_va, handler_data)) = found_handler {
                self.maps.write_qword(disp_ctx, current_rip);
                self.maps.write_qword(disp_ctx + 0x08, pe_base);
                self.maps.write_qword(disp_ctx + 0x10, entry_addr);
                self.maps.write_qword(disp_ctx + 0x18, establisher_frame);
                self.maps.write_qword(disp_ctx + 0x20, 0);
                self.maps.write_qword(disp_ctx + 0x28, context_addr);
                self.maps.write_qword(disp_ctx + 0x30, handler_va);
                self.maps.write_qword(disp_ctx + 0x38, handler_data);
                self.maps.write_qword(disp_ctx + 0x40, 0);
                self.maps.write_dword(disp_ctx + 0x48, 0);

                log::trace!(
                    "SEH dispatch: handler 0x{:x} establisher=0x{:x} rip=0x{:x} flags=0x{:x}",
                    handler_va,
                    establisher_frame,
                    current_rip,
                    flag_mask,
                );
                // Dump scope table for diagnostics.
                if let Some(scope_count) = self.maps.read_dword(handler_data) {
                    let control_rva = current_rip.wrapping_sub(pe_base) as u32;
                    log::trace!(
                        "  scope_table at 0x{:x}: count={} control_rva=0x{:x}",
                        handler_data,
                        scope_count,
                        control_rva,
                    );
                    for i in 0..scope_count.min(16) {
                        let base = handler_data + 4 + i as u64 * 16;
                        let begin = self.maps.read_dword(base).unwrap_or(0);
                        let end = self.maps.read_dword(base + 4).unwrap_or(0);
                        let filter = self.maps.read_dword(base + 8).unwrap_or(0);
                        let target = self.maps.read_dword(base + 12).unwrap_or(0);
                        log::trace!(
                            "  scope[{}]: begin=0x{:x} end=0x{:x} filter=0x{:x} target=0x{:x} {}",
                            i,
                            begin,
                            end,
                            filter,
                            target,
                            if control_rva >= begin && control_rva < end {
                                "<<MATCH>>"
                            } else {
                                ""
                            },
                        );
                    }
                }

                match self.call64(
                    handler_va,
                    &[record_addr, establisher_frame, context_addr, disp_ctx],
                ) {
                    Ok(disposition) => {
                        let d = disposition as u32;
                        if d == ExceptionDisposition::ContinueExecution as u32 {
                            handled = true;
                            break;
                        }
                        if d == ExceptionDisposition::ContinueSearch as u32 {
                            if flag_mask == UNW_FLAG_UHANDLER
                                && let Some(r) = self.scope_table_dispatch(
                                    pe_base,
                                    current_rip,
                                    handler_data,
                                    record_addr,
                                    context_addr,
                                    establisher_frame,
                                )
                            {
                                if r {
                                    handled = true;
                                }
                                break;
                            }
                        } else {
                            log::warn!(
                                "unexpected disposition 0x{:x} from handler 0x{:x}",
                                d,
                                handler_va,
                            );
                            break;
                        }
                    }
                    Err(e) => {
                        if self.regs().rip != saved_rip || self.regs().rsp != saved_rsp {
                            handled = true;
                        } else if flag_mask == UNW_FLAG_UHANDLER {
                            // Handler crashed — try scope table directly.
                            log::trace!("handler call64 error: {} — trying scope table", e,);
                            if let Some(true) = self.scope_table_dispatch(
                                pe_base,
                                current_rip,
                                handler_data,
                                record_addr,
                                context_addr,
                                establisher_frame,
                            ) {
                                handled = true;
                            }
                        }
                        break;
                    }
                }
            }

            // Pop return address to advance to the next frame.
            if !machframe {
                let rsp_val = self
                    .maps
                    .read_qword(scratch_ctx + Context64::RSP_OFFSET)
                    .unwrap_or(0);
                if let Some(ret_addr) = self.maps.read_qword(rsp_val) {
                    self.maps
                        .write_qword(scratch_ctx + Context64::RIP_OFFSET, ret_addr);
                    self.maps
                        .write_qword(scratch_ctx + Context64::RSP_OFFSET, rsp_val + 8);
                } else {
                    break;
                }
            }
        }

        handled
    }

    /// Interpret HandlerData as a __C_specific_handler scope table and try to
    /// find a matching except clause for `control_pc`.  When a match with a
    /// filter that returns EXCEPTION_EXECUTE_HANDLER is found, transfer control
    /// to the except block and return `Some(true)`.  `Some(false)` means the
    /// scope table was valid but no entry matched.  `None` means the data does
    /// not look like a scope table (count too large, bad RVAs) so the caller
    /// should keep searching the next frame.
    fn scope_table_dispatch(
        &mut self,
        pe_base: u64,
        control_pc: u64,
        handler_data: u64,
        record_addr: u64,
        context_addr: u64,
        establisher_frame: u64,
    ) -> Option<bool> {
        let count = self.maps.read_dword(handler_data)?;
        if count == 0 || count > 64 {
            return None;
        }
        let control_rva = control_pc.wrapping_sub(pe_base) as u32;
        log::trace!(
            "scope_table_dispatch: count={} control_rva=0x{:x}",
            count,
            control_rva,
        );

        for i in 0..count {
            let base = handler_data + 4 + i as u64 * 16;
            let begin = self.maps.read_dword(base)?;
            let end = self.maps.read_dword(base + 4)?;
            let filter_rva = self.maps.read_dword(base + 8)?;
            let target_rva = self.maps.read_dword(base + 12)?;

            if control_rva < begin || control_rva >= end || target_rva == 0 {
                continue;
            }

            let filter_result = if filter_rva == 1 {
                1i64 // EXCEPTION_EXECUTE_HANDLER (catch-all)
            } else if filter_rva == 0 {
                continue; // finally block, not an except handler
            } else {
                let filter_va = pe_base + filter_rva as u64;
                // Build EXCEPTION_POINTERS on the stack for the filter.
                let ptrs = establisher_frame.wrapping_sub(0x10);
                self.maps.write_qword(ptrs, record_addr);
                self.maps.write_qword(ptrs + 8, context_addr);
                log::trace!("scope_table_dispatch: filter at 0x{:x}", filter_va);
                let was_terminated = self.process_terminated;
                match self.call64(filter_va, &[ptrs, establisher_frame]) {
                    Ok(v) => {
                        log::trace!("scope_table_dispatch: filter returned {}", v as i64);
                        v as i64
                    }
                    Err(e) => {
                        log::trace!("scope_table_dispatch: filter crashed: {}", e);
                        if !was_terminated && self.process_terminated {
                            // The filter itself crashed (emulation artifact).
                            // On real Windows the filter would succeed; treat
                            // the matching scope entry as a catch-all.
                            self.process_terminated = false;
                            self.is_running
                                .store(1, std::sync::atomic::Ordering::Relaxed);
                            1 // EXCEPTION_EXECUTE_HANDLER
                        } else {
                            continue;
                        }
                    }
                }
            };

            if filter_result == 1 {
                // EXCEPTION_EXECUTE_HANDLER — transfer to the except block.
                let target = pe_base + target_rva as u64;
                log::info!(
                    "scope_table_dispatch: handled 0x{:x} → except at 0x{:x}",
                    control_pc,
                    target,
                );
                self.regs_mut().rip = target;
                self.regs_mut().rsp = establisher_frame;
                return Some(true);
            }
        }
        Some(false)
    }

    /// Apply UWOP codes from the UNWIND_INFO at `ui_addr` to the CONTEXT at
    /// `ctx` in guest memory, following CHAININFO links. Does NOT pop the
    /// return address — the caller is responsible for that step.
    /// Returns `Some(true)` if MACHFRAME already set RIP/RSP (no pop needed),
    /// `Some(false)` if codes applied normally (caller must pop), `None` on error.
    fn apply_frame_codes_on_context(&mut self, mut ui_addr: u64, ctx: u64) -> Option<bool> {
        const MAX_CHAIN: usize = 16;
        const CTX_RSP: u64 = Context64::RSP_OFFSET;
        const CTX_RIP: u64 = Context64::RIP_OFFSET;

        for _ in 0..MAX_CHAIN {
            let version_flags = self.maps.read_byte(ui_addr)?;
            let flags = (version_flags >> 3) & 0x1F;
            let count_of_codes = self.maps.read_byte(ui_addr + 2)? as u64;
            let frame_reg_off = self.maps.read_byte(ui_addr + 3).unwrap_or(0);
            let frame_register = frame_reg_off & 0x0F;
            let frame_offset = ((frame_reg_off >> 4) & 0x0F) as u64 * 16;

            let codes_base = ui_addr + 4;
            let mut i: u64 = 0;

            while i < count_of_codes {
                let code_word = self.maps.read_word(codes_base + i * 2)?;
                let op = ((code_word >> 8) & 0x0F) as u8;
                let info = ((code_word >> 12) & 0x0F) as u8;

                match op {
                    0 => {
                        // UWOP_PUSH_NONVOL
                        let rsp_val = self.maps.read_qword(ctx + CTX_RSP).unwrap_or(0);
                        if let Some(val) = self.maps.read_qword(rsp_val)
                            && let Some(off) = ctx_reg_offset(info)
                        {
                            self.maps.write_qword(ctx + off, val);
                        }
                        self.maps.write_qword(ctx + CTX_RSP, rsp_val + 8);
                        i += 1;
                    }
                    1 => {
                        // UWOP_ALLOC_LARGE
                        let rsp_val = self.maps.read_qword(ctx + CTX_RSP).unwrap_or(0);
                        if info == 0 {
                            let next =
                                self.maps.read_word(codes_base + (i + 1) * 2).unwrap_or(0) as u64;
                            self.maps.write_qword(ctx + CTX_RSP, rsp_val + next * 8);
                            i += 2;
                        } else {
                            let lo =
                                self.maps.read_word(codes_base + (i + 1) * 2).unwrap_or(0) as u32;
                            let hi =
                                self.maps.read_word(codes_base + (i + 2) * 2).unwrap_or(0) as u32;
                            let size = ((hi << 16) | lo) as u64;
                            self.maps.write_qword(ctx + CTX_RSP, rsp_val + size);
                            i += 3;
                        }
                    }
                    2 => {
                        // UWOP_ALLOC_SMALL
                        let rsp_val = self.maps.read_qword(ctx + CTX_RSP).unwrap_or(0);
                        self.maps
                            .write_qword(ctx + CTX_RSP, rsp_val + (info as u64) * 8 + 8);
                        i += 1;
                    }
                    3 => {
                        // UWOP_SET_FPREG: RSP = frame_reg - frame_offset
                        if frame_register != 0
                            && let Some(off) = ctx_reg_offset(frame_register)
                        {
                            let fp_val = self.maps.read_qword(ctx + off).unwrap_or(0);
                            self.maps
                                .write_qword(ctx + CTX_RSP, fp_val.wrapping_sub(frame_offset));
                        }
                        i += 1;
                    }
                    4 => {
                        // UWOP_SAVE_NONVOL
                        let rsp_val = self.maps.read_qword(ctx + CTX_RSP).unwrap_or(0);
                        let offset =
                            self.maps.read_word(codes_base + (i + 1) * 2).unwrap_or(0) as u64 * 8;
                        if let Some(val) = self.maps.read_qword(rsp_val + offset)
                            && let Some(off) = ctx_reg_offset(info)
                        {
                            self.maps.write_qword(ctx + off, val);
                        }
                        i += 2;
                    }
                    5 => {
                        // UWOP_SAVE_NONVOL_FAR
                        let rsp_val = self.maps.read_qword(ctx + CTX_RSP).unwrap_or(0);
                        let lo = self.maps.read_word(codes_base + (i + 1) * 2).unwrap_or(0) as u32;
                        let hi = self.maps.read_word(codes_base + (i + 2) * 2).unwrap_or(0) as u32;
                        let offset = ((hi << 16) | lo) as u64;
                        if let Some(val) = self.maps.read_qword(rsp_val + offset)
                            && let Some(off) = ctx_reg_offset(info)
                        {
                            self.maps.write_qword(ctx + off, val);
                        }
                        i += 3;
                    }
                    8 => i += 2, // UWOP_SAVE_XMM128 (skip)
                    9 => i += 3, // UWOP_SAVE_XMM128_FAR (skip)
                    10 => {
                        // UWOP_PUSH_MACHFRAME
                        let mut rsp_val = self.maps.read_qword(ctx + CTX_RSP).unwrap_or(0);
                        if info == 1 {
                            rsp_val += 8;
                        }
                        let new_rip = self.maps.read_qword(rsp_val).unwrap_or(0);
                        let new_rsp = self.maps.read_qword(rsp_val + 24).unwrap_or(0);
                        self.maps.write_qword(ctx + CTX_RIP, new_rip);
                        self.maps.write_qword(ctx + CTX_RSP, new_rsp);
                        return Some(true);
                    }
                    _ => {
                        log::warn!("unknown UWOP code {} at 0x{:x}", op, codes_base + i * 2);
                        return None;
                    }
                }
            }

            // Follow CHAININFO.
            let codes_end = codes_base + count_of_codes * 2;
            let aligned = (codes_end + 3) & !3;
            if flags & UNW_FLAG_CHAININFO != 0 {
                if let Some(rva) = self.maps.read_dword(aligned + 8) {
                    let (pe_base, _) = self.maps.find_pe_image_info(ui_addr)?;
                    ui_addr = pe_base + rva as u64;
                    continue;
                }
                return None;
            }

            return Some(false);
        }

        log::warn!("UNWIND_INFO chain depth exceeded at 0x{:x}", ui_addr);
        None
    }

    /// Apply UWOP codes and pop the return address in one step.
    fn unwind_one_frame_on_context(&mut self, ui_addr: u64, ctx: u64) -> bool {
        match self.apply_frame_codes_on_context(ui_addr, ctx) {
            Some(true) => true,
            Some(false) => {
                let rsp_val = self
                    .maps
                    .read_qword(ctx + Context64::RSP_OFFSET)
                    .unwrap_or(0);
                if let Some(ret_addr) = self.maps.read_qword(rsp_val) {
                    self.maps.write_qword(ctx + Context64::RIP_OFFSET, ret_addr);
                    self.maps
                        .write_qword(ctx + Context64::RSP_OFFSET, rsp_val + 8);
                    true
                } else {
                    false
                }
            }
            None => false,
        }
    }

    pub(crate) fn dispatch_unhandled_exception(&mut self, record: &ExceptionRecord) {
        if self.kernel_driver_active() {
            self.kernel_abort(format!(
                "unhandled exception 0x{:x} at 0x{:x}",
                record.code, record.address
            ));
            return;
        }
        if self.uef() != 0 {
            self.call_exception_handler(record, self.uef(), HandlerKind::Uef, 0);
        } else {
            log::warn!(
                "no handler for exception 0x{:x} at 0x{:x}",
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
