use crate::{emu::Emu, threading::context::ArchThreadState};

impl Emu {
    pub fn call_stack(&self) -> &Vec<(u64, u64)> {
        match &self.threads[self.current_thread_id].arch {
            ArchThreadState::X86 { call_stack, .. } => call_stack,
            _ => panic!("call_stack() called on aarch64 emu"),
        }
    }

    pub fn call_stack_mut(&mut self) -> &mut Vec<(u64, u64)> {
        match &mut self.threads[self.current_thread_id].arch {
            ArchThreadState::X86 { call_stack, .. } => call_stack,
            _ => panic!("call_stack_mut() called on aarch64 emu"),
        }
    }
}
