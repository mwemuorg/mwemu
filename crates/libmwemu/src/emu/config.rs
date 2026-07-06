use crate::{config::Config, emu::Emu};

impl Emu {
    /// Set a custom config, normally used only from commandline tool main.rs
    pub fn set_config(&mut self, cfg: Config) {
        self.maps.max_alloc_size = cfg.max_alloc_size;
        self.cfg = cfg;
        if self.cfg.console {
            self.exp = self.cfg.console_num;
        }
        if self.cfg.nocolors {
            self.colors.disable();
        }
    }

    /// change default base address, code map will be loaded there.
    pub fn set_base_address(&mut self, addr: u64) {
        self.cfg.code_base_addr = addr;
    }

    // configure the base address of stack map
    pub fn set_stack_address(&mut self, addr: u64) {
        self.cfg.stack_addr = addr;
    }

    /// Set verbose level.
    /// 0: only will print syscalls and api name called.
    /// 1: same than 0 and also some messages like undefined behaviours or polymorfic code etc.
    /// 2: same than 1 and also will print the assembly code, this will make it much slower.
    /// 3: same than 2 but also in the case of a rep prefix will see every rep iteration.
    pub fn set_verbose(&mut self, n: u32) {
        if n > 3 {
            panic!("verbose is from 0 to 3 display (0:apis, 1:msgs, 2:asm, 3:rep)");
        }
        self.cfg.verbose = n;
    }
}
