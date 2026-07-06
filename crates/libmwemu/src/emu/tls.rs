use crate::{emu::Emu, threading::context::ArchThreadState};

impl Emu {
    pub fn tls32(&self) -> &Vec<u32> {
        match &self.threads[self.current_thread_id].arch {
            ArchThreadState::X86 { tls32, .. } => tls32,
            _ => panic!("tls32() called on aarch64 emu"),
        }
    }

    pub fn tls32_mut(&mut self) -> &mut Vec<u32> {
        match &mut self.threads[self.current_thread_id].arch {
            ArchThreadState::X86 { tls32, .. } => tls32,
            _ => panic!("tls32_mut() called on aarch64 emu"),
        }
    }

    pub fn tls64(&self) -> &Vec<u64> {
        match &self.threads[self.current_thread_id].arch {
            ArchThreadState::X86 { tls64, .. } => tls64,
            _ => panic!("tls64() called on aarch64 emu"),
        }
    }

    pub fn tls64_mut(&mut self) -> &mut Vec<u64> {
        match &mut self.threads[self.current_thread_id].arch {
            ArchThreadState::X86 { tls64, .. } => tls64,
            _ => panic!("tls64_mut() called on aarch64 emu"),
        }
    }
}
