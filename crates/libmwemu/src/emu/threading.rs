use crate::emu::Emu;

impl Emu {
    /// Enable multi-threading support for the emulator.
    /// When enabled, the emulator will use thread scheduling for multiple threads.
    /// When disabled (default), it runs in single-threaded mode for backward compatibility.
    pub fn enable_threading(&mut self, enabled: bool) {
        self.cfg.enable_threading = enabled;
    }

    /// Check if multi-threading is enabled.
    pub fn is_threading_enabled(&self) -> bool {
        self.cfg.enable_threading
    }
}
