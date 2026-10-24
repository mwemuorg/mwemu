use crate::{
    emu::Emu, regs_aarch64::RegsAarch64, regs64::Regs64, threading::context::ArchThreadState,
};

impl Emu {
    /// x86/x64 register file of the current thread.
    pub fn regs(&self) -> &Regs64 {
        match &self.threads[self.current_thread_id].arch {
            ArchThreadState::X86 { regs, .. } => regs,
            _ => unreachable!("regs() called on aarch64 emu"),
        }
    }

    /// Mutable x86/x64 register file of the current thread.
    pub fn regs_mut(&mut self) -> &mut Regs64 {
        match &mut self.threads[self.current_thread_id].arch {
            ArchThreadState::X86 { regs, .. } => regs,
            _ => unreachable!("regs_mut() called on aarch64 emu"),
        }
    }

    /// AArch64 register file of the current thread.
    pub fn regs_aarch64(&self) -> &RegsAarch64 {
        match &self.threads[self.current_thread_id].arch {
            ArchThreadState::AArch64 { regs, .. } => regs,
            _ => unreachable!("regs_aarch64 called on non-aarch64 emu"),
        }
    }

    /// Mutable AArch64 register file of the current thread.
    pub fn regs_aarch64_mut(&mut self) -> &mut RegsAarch64 {
        match &mut self.threads[self.current_thread_id].arch {
            ArchThreadState::AArch64 { regs, .. } => regs,
            _ => unreachable!("regs_aarch64_mut called on non-aarch64 emu"),
        }
    }

    /// Current program counter (rip on x86, pc on AArch64).
    pub fn pc(&self) -> u64 {
        match &self.threads[self.current_thread_id].arch {
            ArchThreadState::X86 { regs, .. } => regs.rip,
            ArchThreadState::AArch64 { regs, .. } => regs.pc,
        }
    }

    /// Set the program counter.
    pub fn set_pc(&mut self, addr: u64) {
        match &mut self.threads[self.current_thread_id].arch {
            ArchThreadState::X86 { regs, .. } => regs.rip = addr,
            ArchThreadState::AArch64 { regs, .. } => regs.pc = addr,
        }
    }

    /// Current stack pointer (rsp on x86, sp on AArch64).
    pub fn sp(&self) -> u64 {
        match &self.threads[self.current_thread_id].arch {
            ArchThreadState::X86 { regs, .. } => regs.rsp,
            ArchThreadState::AArch64 { regs, .. } => regs.sp,
        }
    }

    /// Set the stack pointer.
    pub fn set_sp(&mut self, addr: u64) {
        match &mut self.threads[self.current_thread_id].arch {
            ArchThreadState::X86 { regs, .. } => regs.rsp = addr,
            ArchThreadState::AArch64 { regs, .. } => regs.sp = addr,
        }
    }

    pub fn set_pre_op_regs(&mut self, new_regs: Regs64) {
        self.trace_mut_x86().pre_regs = new_regs;
    }

    pub fn set_post_op_regs(&mut self, new_regs: Regs64) {
        self.trace_mut_x86().post_regs = new_regs;
    }

    pub fn pre_op_regs(&self) -> &Regs64 {
        &self.trace_ref_x86().pre_regs
    }

    pub fn pre_op_regs_mut(&mut self) -> &mut Regs64 {
        &mut self.trace_mut_x86().pre_regs
    }

    pub fn post_op_regs(&self) -> &Regs64 {
        &self.trace_ref_x86().post_regs
    }

    pub fn post_op_regs_mut(&mut self) -> &mut Regs64 {
        &mut self.trace_mut_x86().post_regs
    }

    pub fn pre_op_regs_aarch64(&self) -> &RegsAarch64 {
        &self.trace_ref_aarch64().pre_regs
    }
    pub fn pre_op_regs_aarch64_mut(&mut self) -> &mut RegsAarch64 {
        &mut self.trace_mut_aarch64().pre_regs
    }

    pub fn post_op_regs_aarch64_mut(&mut self) -> &mut RegsAarch64 {
        &mut self.trace_mut_aarch64().post_regs
    }

    pub fn post_op_regs_aarch64(&self) -> &RegsAarch64 {
        &self.trace_ref_aarch64().post_regs
    }

    #[inline(always)]
    fn trace_ref_x86(&self) -> &crate::threading::context::X86TraceSnapshot {
        match &self.threads[self.current_thread_id].arch {
            crate::threading::context::ArchThreadState::X86 { .. } => {}
            _ => unreachable!("x86 trace snapshot requested on aarch64 thread"),
        }
        self.threads[self.current_thread_id].arch.x86_trace_ref()
    }

    fn trace_mut_x86(&mut self) -> &mut crate::threading::context::X86TraceSnapshot {
        match &mut self.threads[self.current_thread_id].arch {
            crate::threading::context::ArchThreadState::X86 { .. } => {}
            _ => unreachable!("x86 trace snapshot requested on aarch64 thread"),
        }
        self.threads[self.current_thread_id].arch.x86_trace_mut()
    }

    #[inline(always)]
    fn trace_mut_aarch64(&mut self) -> &mut crate::threading::context::AArch64TraceSnapshot {
        match &mut self.threads[self.current_thread_id].arch {
            crate::threading::context::ArchThreadState::AArch64 { .. } => {}
            _ => unreachable!("aarch64 trace snapshot requested on x86 thread"),
        }
        self.threads[self.current_thread_id]
            .arch
            .aarch64_trace_mut()
    }

    #[inline(always)]
    fn trace_ref_aarch64(&self) -> &crate::threading::context::AArch64TraceSnapshot {
        match &self.threads[self.current_thread_id].arch {
            crate::threading::context::ArchThreadState::AArch64 { .. } => {}
            _ => unreachable!("aarch64 trace snapshot requested on x86 thread"),
        }
        self.threads[self.current_thread_id]
            .arch
            .aarch64_trace_ref()
    }
}
