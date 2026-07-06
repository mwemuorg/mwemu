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
            ArchThreadState::X86 { veh, .. } => *veh,
            _ => panic!("veh() called on aarch64 emu"),
        }
    }

    pub fn set_veh(&mut self, value: u64) {
        match &mut self.threads[self.current_thread_id].arch {
            ArchThreadState::X86 { veh, .. } => *veh = value,
            _ => panic!("set_veh() called on aarch64 emu"),
        }
    }

    pub fn uef(&self) -> u64 {
        match &self.threads[self.current_thread_id].arch {
            ArchThreadState::X86 { uef, .. } => *uef,
            _ => panic!("uef() called on aarch64 emu"),
        }
    }

    pub fn set_uef(&mut self, value: u64) {
        match &mut self.threads[self.current_thread_id].arch {
            ArchThreadState::X86 { uef, .. } => *uef = value,
            _ => panic!("set_uef() called on aarch64 emu"),
        }
    }

    pub fn eh_ctx(&self) -> u64 {
        match &self.threads[self.current_thread_id].arch {
            ArchThreadState::X86 { eh_ctx, .. } => *eh_ctx,
            _ => panic!("eh_ctx() called on aarch64 emu"),
        }
    }

    pub fn set_eh_ctx(&mut self, value: u64) {
        match &mut self.threads[self.current_thread_id].arch {
            ArchThreadState::X86 { eh_ctx, .. } => *eh_ctx = value,
            _ => panic!("set_eh_ctx() called on aarch64 emu"),
        }
    }

    pub fn seh(&self) -> u64 {
        match &self.threads[self.current_thread_id].arch {
            ArchThreadState::X86 { seh, .. } => *seh,
            _ => panic!("seh() called on aarch64 emu"),
        }
    }

    pub fn set_seh(&mut self, value: u64) {
        match &mut self.threads[self.current_thread_id].arch {
            ArchThreadState::X86 { seh, .. } => *seh = value,
            _ => panic!("set_seh() called on aarch64 emu"),
        }
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
        if self.seh() == 0 && self.veh() == 0 && self.uef() == 0 {
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
                self.set_rip(addr, false);
            } else {
                self.set_eip(addr, false);
            }

        // SEH
        } else if self.seh() > 0 {
            if self.cfg.is_x64() {
                // x64 SEH is table-driven via the loaded module's .pdata /
                // RUNTIME_FUNCTION entries (no per-thread chain like x86).
                // We don't model the unwind tables, so a configured `seh()`
                // here means somebody synthesised one — treat it as fatal
                // rather than silently returning, so the caller sees the
                // unsupported path.
                log::error!(
                    "x64 SEH unwind not implemented (no .pdata walker). \
                     pos = {} rip = 0x{:x} type = {:?} — stopping emulation",
                    self.pos,
                    self.pc(),
                    ex_type,
                );
                self.process_terminated = true;
                self.is_running.store(0, Ordering::Relaxed);
                return;
            } else {
                // 32bits seh
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
            }

            let con = Console::new();
            if self.running_script {
                self.set_seh(next);
                handlers::enter_for_handler(self, ex_type, HandlerKind::Seh);
                if self.cfg.is_x64() {
                    self.set_rip(addr, false);
                } else {
                    self.set_eip(addr, false);
                }
                return;
            }
            con.print("jump the exception pointer (y/n)?");

            let cmd = con.cmd();
            if cmd == "y" {
                self.set_seh(next);
                handlers::enter_for_handler(self, ex_type, HandlerKind::Seh);
                if self.cfg.is_x64() {
                    self.set_rip(addr, false);
                } else {
                    self.set_eip(addr, false);
                }
            }
        } else if self.uef() > 0 {
            // UEF

            addr = self.uef();

            handlers::enter_for_handler(self, ex_type, HandlerKind::Uef);
            if self.cfg.is_x64() {
                self.set_rip(addr, false);
            } else {
                self.set_eip(addr, false);
            }
        } else {
            unreachable!();
        }
    }
}
