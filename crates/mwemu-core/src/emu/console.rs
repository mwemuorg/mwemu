use crate::{console::Console, emu::Emu};

impl Emu {
    /// Do enable the Control + C handling, for spawning console.
    pub fn enable_ctrlc(&mut self) {
        self.enabled_ctrlc = true;
    }

    /// Do disable the Control + C handling, it will not be handled and will interrupt the program.
    pub fn disable_ctrlc(&mut self) {
        self.enabled_ctrlc = false;
    }

    /// Disable the console mode, it will not be spawned automatically.
    pub fn disable_console(&mut self) {
        self.cfg.console_enabled = false;
    }

    /// Enable the console mode, it will spawned automatically in some situations.
    pub fn enable_console(&mut self) {
        self.cfg.console_enabled = true;
    }

    /// Do spawn a console, for user interaction with the current emulation state.
    /// Command h for help.
    pub fn spawn_console(&mut self) {
        Console::spawn_console(self);
    }

    /// Spawn a console on the instruction number, ie: 1 after emulating first instruction.
    pub fn spawn_console_at(&mut self, exp: u64) {
        self.exp = exp;
    }

    /// Spawn a console the first time the specified address is reached.
    pub fn spawn_console_at_addr(&mut self, addr: u64) {
        self.cfg.console2 = true;
        self.cfg.console_addr = addr;
        self.cfg.console_enabled = true;
    }
}
