use std::sync::atomic::Ordering;

use crate::{
    console::Console,
    emu::Emu,
    exception::handlers::{self, HandlerKind},
    exception::types::ExceptionType,
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

        const UNW_FLAG_EHANDLER: u8 = 0x01;
        const UNW_FLAG_UHANDLER: u8 = 0x02;
        const UNW_FLAG_CHAININFO: u8 = 0x04;
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

            if flags & (UNW_FLAG_EHANDLER | UNW_FLAG_UHANDLER) != 0 {
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

    /// Trigger an exception.
    /// If it has to be handled initiate contex tand jump to the programmed error routine.
    /// Support SEH, VEH and UEF
    pub fn exception(&mut self, ex_type: ExceptionType) {
        self.fault_count += 1;

        let addr: u64;
        let next: u64;

        // hook
        let handle_exception: bool = if let Some(mut hook_fn) = self.hooks.hook_on_exception.take()
        {
            let pc = self.pc();
            let result = hook_fn(self, pc, ex_type);
            self.hooks.hook_on_exception = Some(hook_fn);
            result
        } else {
            true
        };

        // aarch64 has no SEH/VEH/UEF
        if self.cfg.arch.is_aarch64() {
            log::error!(
                "exception on aarch64 (no SEH/VEH support). pos = {} pc = 0x{:x} type = {:?}",
                self.pos,
                self.pc(),
                ex_type,
            );
            self.process_terminated = true;
            self.is_running.store(0, Ordering::Relaxed);
            return;
        }

        // No handler installed (no SEH chain, no VEH list, no UEF callback).
        // Real Windows would invoke KiUserExceptionDispatcher → unhandled
        // exception filter → ExitProcess. We don't have a kernel side, so the
        // safest answer is to STOP emulation right here: silently dropping the
        // fault lets ntdll continue with corrupted state and the resulting
        // crash surfaces millions of instructions later in unrelated code,
        // which makes diagnosis nearly impossible. Surfacing the fault at its
        // original RIP gives the operator the exact frame that needs fixing.
        if self.seh() == 0 && self.veh() == 0 && self.uef() == 0 && !self.cfg.is_x64() {
            log::error!(
                "unhandled exception (no SEH/VEH/UEF configured). pos = {} rip = 0x{:x} type = {:?} — stopping emulation",
                self.pos,
                self.pc(),
                ex_type,
            );
            self.process_terminated = true;
            self.is_running.store(0, Ordering::Relaxed);
            return;
        }

        // hook replaced handler
        if !handle_exception {
            log::trace!("cancelled exception handling from hook.");
            return;
        }

        // VEH
        if self.veh() > 0 {
            addr = self.veh();

            handlers::enter_for_handler(self, ex_type, HandlerKind::Veh);

            if self.cfg.is_x64() {
                self.set_rip_with_check(addr, false);
            } else {
                self.set_eip(addr, false);
            }

        // x64 table-driven SEH (.pdata / RUNTIME_FUNCTION)
        } else if self.cfg.is_x64() {
            match self.find_x64_seh_handler(self.pc()) {
                Some(handler_addr) => {
                    log::trace!(
                        "x64 SEH: .pdata handler at 0x{:x} for rip 0x{:x}",
                        handler_addr,
                        self.pc(),
                    );
                    handlers::enter_for_handler(self, ex_type, HandlerKind::Seh);
                    self.set_rip_with_check(handler_addr, false);
                }
                None => {
                    if self.uef() > 0 {
                        let uef_addr = self.uef();
                        handlers::enter_for_handler(self, ex_type, HandlerKind::Uef);
                        self.set_rip_with_check(uef_addr, false);
                    } else {
                        log::error!(
                            "x64 unhandled exception: no .pdata handler and no UEF. \
                             pos = {} rip = 0x{:x} type = {:?} — stopping emulation",
                            self.pos,
                            self.pc(),
                            ex_type,
                        );
                        self.process_terminated = true;
                        self.is_running.store(0, Ordering::Relaxed);
                    }
                }
            }

        // x86 stack-chain SEH
        } else if self.seh() > 0 {
            next = match self.maps.read_dword(self.seh()) {
                Some(value) => value.into(),
                None => {
                    log::error!(
                        "SEH record at 0x{:x} unreadable (Next field). \
                         pos = {} rip = 0x{:x} — stopping emulation",
                        self.seh(),
                        self.pos,
                        self.pc(),
                    );
                    self.process_terminated = true;
                    self.is_running.store(0, Ordering::Relaxed);
                    return;
                }
            };

            addr = match self.maps.read_dword(self.seh() + 4) {
                Some(value) => value.into(),
                None => {
                    log::error!(
                        "SEH record at 0x{:x} unreadable (Handler field). \
                         pos = {} rip = 0x{:x} — stopping emulation",
                        self.seh(),
                        self.pos,
                        self.pc(),
                    );
                    self.process_terminated = true;
                    self.is_running.store(0, Ordering::Relaxed);
                    return;
                }
            };

            let con = Console::new();
            if self.running_script {
                self.set_seh(next);
                handlers::enter_for_handler(self, ex_type, HandlerKind::Seh);
                self.set_eip(addr, false);
                return;
            }
            con.print("jump the exception pointer (y/n)?");

            let cmd = con.cmd();
            if cmd == "y" {
                self.set_seh(next);
                handlers::enter_for_handler(self, ex_type, HandlerKind::Seh);
                self.set_eip(addr, false);
            }
        } else if self.uef() > 0 {
            // UEF

            addr = self.uef();

            handlers::enter_for_handler(self, ex_type, HandlerKind::Uef);
            if self.cfg.is_x64() {
                self.set_rip_with_check(addr, false);
            } else {
                self.set_eip(addr, false);
            }
        } else {
            unreachable!();
        }
    }
}
