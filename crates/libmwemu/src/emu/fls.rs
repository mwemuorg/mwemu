use crate::{emu::Emu, threading::context::ArchThreadState};

impl Emu {
    pub fn fls(&self) -> &Vec<u32> {
        match &self.threads[self.current_thread_id].arch {
            ArchThreadState::X86 { fls, .. } => fls,
            _ => panic!("fls() called on aarch64 emu"),
        }
    }

    pub fn fls_mut(&mut self) -> &mut Vec<u32> {
        match &mut self.threads[self.current_thread_id].arch {
            ArchThreadState::X86 { fls, .. } => fls,
            _ => panic!("fls_mut() called on aarch64 emu"),
        }
    }
}
