use crate::{emu::Emu, threading::context::ThreadContext};

impl Emu {
    // Thread management helper methods
    pub fn current_thread(&self) -> &ThreadContext {
        &self.threads[self.current_thread_id]
    }

    pub fn current_thread_mut(&mut self) -> &mut ThreadContext {
        &mut self.threads[self.current_thread_id]
    }
}
