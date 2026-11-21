use crate::{emu::Emu, threading::context::ThreadContext};

impl Emu {
    // Thread management helper methods
    pub fn current_thread(&self) -> &ThreadContext {
        &self.threads[self.current_thread_id]
    }

    pub fn current_thread_mut(&mut self) -> &mut ThreadContext {
        &mut self.threads[self.current_thread_id]
    }

    /// TEB belonging to the selected Windows user-mode thread.
    pub(crate) fn windows_teb(&self) -> Option<u64> {
        if !self.os.is_windows() || self.kernel.is_some() {
            return None;
        }
        let base = self.current_thread().windows.teb;
        if base != 0 {
            return Some(base);
        }
        if self.current_thread_id == 0 {
            return self.maps.get_map_by_name("teb").map(|map| map.get_base());
        }
        None
    }
}
