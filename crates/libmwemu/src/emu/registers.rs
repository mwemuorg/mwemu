use crate::{
    emu::Emu, regs_aarch64::RegsAarch64, regs64::Regs64, threading::context::ArchThreadState,
};

impl Emu {
    // Forwarding methods for thread-specific fields
    pub fn regs(&self) -> &Regs64 {
        match &self.threads[self.current_thread_id].arch {
            ArchThreadState::X86 { regs, .. } => regs,
            _ => panic!("regs() called on aarch64 emu"),
        }
    }

    pub fn regs_mut(&mut self) -> &mut Regs64 {
        match &mut self.threads[self.current_thread_id].arch {
            ArchThreadState::X86 { regs, .. } => regs,
            _ => panic!("regs_mut() called on aarch64 emu"),
        }
    }

    // AArch64 register accessors
    pub fn regs_aarch64(&self) -> &RegsAarch64 {
        match &self.threads[self.current_thread_id].arch {
            ArchThreadState::AArch64 { regs, .. } => regs,
            _ => panic!("regs_aarch64 called on non-aarch64 emu"),
        }
    }

    pub fn regs_aarch64_mut(&mut self) -> &mut RegsAarch64 {
        match &mut self.threads[self.current_thread_id].arch {
            ArchThreadState::AArch64 { regs, .. } => regs,
            _ => panic!("regs_aarch64_mut called on non-aarch64 emu"),
        }
    }

    // Unified program counter for shared code paths
    pub fn pc(&self) -> u64 {
        match &self.threads[self.current_thread_id].arch {
            ArchThreadState::X86 { regs, .. } => regs.rip,
            ArchThreadState::AArch64 { regs, .. } => regs.pc,
        }
    }

    pub fn set_pc(&mut self, addr: u64) {
        match &mut self.threads[self.current_thread_id].arch {
            ArchThreadState::X86 { regs, .. } => regs.rip = addr,
            ArchThreadState::AArch64 { regs, .. } => regs.pc = addr,
        }
    }

    // Unified stack pointer
    pub fn sp(&self) -> u64 {
        match &self.threads[self.current_thread_id].arch {
            ArchThreadState::X86 { regs, .. } => regs.rsp,
            ArchThreadState::AArch64 { regs, .. } => regs.sp,
        }
    }

    pub fn set_sp(&mut self, addr: u64) {
        match &mut self.threads[self.current_thread_id].arch {
            ArchThreadState::X86 { regs, .. } => regs.rsp = addr,
            ArchThreadState::AArch64 { regs, .. } => regs.sp = addr,
        }
    }

    pub fn set_pre_op_regs(&mut self, new_regs: Regs64) {
        match &mut self.threads[self.current_thread_id].arch {
            ArchThreadState::X86 { pre_op_regs, .. } => *pre_op_regs = new_regs,
            _ => panic!("set_pre_op_regs called on aarch64 emu"),
        }
    }

    pub fn set_post_op_regs(&mut self, new_regs: Regs64) {
        match &mut self.threads[self.current_thread_id].arch {
            ArchThreadState::X86 { post_op_regs, .. } => *post_op_regs = new_regs,
            _ => panic!("set_post_op_regs called on aarch64 emu"),
        }
    }

    pub fn pre_op_regs(&self) -> &Regs64 {
        match &self.threads[self.current_thread_id].arch {
            ArchThreadState::X86 { pre_op_regs, .. } => pre_op_regs,
            _ => panic!("pre_op_regs called on aarch64 emu"),
        }
    }

    pub fn pre_op_regs_mut(&mut self) -> &mut Regs64 {
        match &mut self.threads[self.current_thread_id].arch {
            ArchThreadState::X86 { pre_op_regs, .. } => pre_op_regs,
            _ => panic!("pre_op_regs_mut called on aarch64 emu"),
        }
    }

    pub fn post_op_regs(&self) -> &Regs64 {
        match &self.threads[self.current_thread_id].arch {
            ArchThreadState::X86 { post_op_regs, .. } => post_op_regs,
            _ => panic!("post_op_regs called on aarch64 emu"),
        }
    }

    pub fn post_op_regs_mut(&mut self) -> &mut Regs64 {
        match &mut self.threads[self.current_thread_id].arch {
            ArchThreadState::X86 { post_op_regs, .. } => post_op_regs,
            _ => panic!("post_op_regs_mut called on aarch64 emu"),
        }
    }

    // AArch64 pre/post op register accessors
    pub fn pre_op_regs_aarch64(&self) -> &RegsAarch64 {
        match &self.threads[self.current_thread_id].arch {
            ArchThreadState::AArch64 { pre_op_regs, .. } => pre_op_regs,
            _ => panic!("pre_op_regs_aarch64 called on x86 emu"),
        }
    }

    pub fn post_op_regs_aarch64(&self) -> &RegsAarch64 {
        match &self.threads[self.current_thread_id].arch {
            ArchThreadState::AArch64 { post_op_regs, .. } => post_op_regs,
            _ => panic!("post_op_regs_aarch64 called on x86 emu"),
        }
    }
}
