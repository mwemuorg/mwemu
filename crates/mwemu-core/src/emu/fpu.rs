use crate::{emu::Emu, fpu::FPU, threading::context::ArchThreadState};

impl Emu {
    pub fn sync_fpu_ip(&mut self) {
        let rip = self.regs().rip;
        if let ArchThreadState::X86 { fpu, .. } = &mut self.threads[self.current_thread_id].arch {
            fpu.set_ip(rip);
        }
    }

    pub fn fpu(&self) -> &FPU {
        match &self.threads[self.current_thread_id].arch {
            ArchThreadState::X86 { fpu, .. } => fpu,
            _ => unreachable!("fpu() called on aarch64 emu"),
        }
    }

    pub fn fpu_mut(&mut self) -> &mut FPU {
        match &mut self.threads[self.current_thread_id].arch {
            ArchThreadState::X86 { fpu, .. } => fpu,
            _ => unreachable!("fpu_mut() called on aarch64 emu"),
        }
    }
}
