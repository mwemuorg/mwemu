use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Breakpoint {
    addr: u64,
    instruction: u64,
    mem_read_addr: u64,
    mem_write_addr: u64,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Breakpoints {
    pub addr: Vec<u64>,
    pub instruction: Vec<u64>,
    pub mem_read_addr: Vec<u64>,
    pub mem_write_addr: Vec<u64>,
}

impl Default for Breakpoints {
    fn default() -> Self {
        Self::new()
    }
}

impl Breakpoints {
    // TODO: implementing clearing breakpoint for console
    pub fn new() -> Self {
        Breakpoints {
            addr: Vec::with_capacity(8),
            instruction: Vec::with_capacity(8),
            mem_read_addr: Vec::with_capacity(8),
            mem_write_addr: Vec::with_capacity(8),
        }
    }

    #[inline]
    pub fn is_bp(&self, addr: u64) -> bool {
        // this can be optmize to avx2 by using -Ctarget-cpu=skylake
        self.addr.contains(&addr)
    }

    #[inline]
    pub fn is_bp_mem_read(&self, addr: u64) -> bool {
        self.mem_read_addr.contains(&addr)
    }

    #[inline]
    pub fn is_bp_mem_write_addr(&self, addr: u64) -> bool {
        self.mem_write_addr.contains(&addr)
    }

    #[inline]
    pub fn is_bp_instruction(&self, addr: u64) -> bool {
        self.instruction.contains(&addr)
    }

    #[inline]
    pub fn add_bp(&mut self, addr: u64) {
        self.addr.push(addr)
    }

    #[inline]
    pub fn add_bp_mem_read(&mut self, addr: u64) {
        self.mem_read_addr.push(addr);
    }

    #[inline]
    pub fn add_bp_mem_write(&mut self, addr: u64) {
        self.mem_write_addr.push(addr);
    }

    #[inline]
    pub fn add_bp_instruction(&mut self, ins: u64) {
        self.instruction.push(ins);
    }

    pub fn clear_bp(&mut self) {
        self.addr.clear();
        self.mem_read_addr.clear();
        self.mem_write_addr.clear();
        self.mem_write_addr.clear();
    }

    #[inline]
    pub fn delete_bp(&mut self, pos: usize) {
        self.addr.remove(pos);
    }

    #[inline]
    pub fn delete_bp_mem_read(&mut self, pos: usize) {
        self.mem_read_addr.remove(pos);
    }

    #[inline]
    pub fn delete_bp_mem_write(&mut self, pos: usize) {
        self.mem_write_addr.remove(pos);
    }

    #[inline]
    pub fn delete_bp_instruction(&mut self, ins: usize) {
        self.instruction.remove(ins);
    }

    pub fn show(&self) {
        let addr_str: Vec<String> = self.addr.iter().map(|a| format!("0x{:x}", a)).collect();
        log::trace!("break on address: [{}]", addr_str.join(", "));

        let instruction_str: Vec<String> = self.addr.iter().map(|a| format!("0x{:x}", a)).collect();
        log::trace!("break on instruction: [{}]", instruction_str.join(", ")); // Uses Debug formatting for the whole vector

        let mem_read_str: Vec<String> = self
            .mem_read_addr
            .iter()
            .map(|a| format!("0x{:x}", a))
            .collect();
        log::trace!("break on memory read: [{}]", mem_read_str.join(", "));

        let mem_write_str: Vec<String> = self
            .mem_write_addr
            .iter()
            .map(|a| format!("0x{:x}", a))
            .collect();
        log::trace!("break on memory write: [{}]", mem_write_str.join(", "));
    }

    /// Remove a breakpoint by address. Returns true if found and removed.
    #[inline]
    pub fn remove_bp_by_addr(&mut self, addr: u64) -> bool {
        if let Some(pos) = self.addr.iter().position(|&a| a == addr) {
            self.addr.remove(pos);
            true
        } else {
            false
        }
    }

    /// Remove a memory read breakpoint by address. Returns true if found and removed.
    #[inline]
    pub fn remove_mem_read_by_addr(&mut self, addr: u64) -> bool {
        if let Some(pos) = self.mem_read_addr.iter().position(|&a| a == addr) {
            self.mem_read_addr.remove(pos);
            true
        } else {
            false
        }
    }

    /// Remove a memory write breakpoint by address. Returns true if found and removed.
    #[inline]
    pub fn remove_mem_write_by_addr(&mut self, addr: u64) -> bool {
        if let Some(pos) = self.mem_write_addr.iter().position(|&a| a == addr) {
            self.mem_write_addr.remove(pos);
            true
        } else {
            false
        }
    }
}

impl Default for Breakpoint {
    fn default() -> Self {
        Self::new()
    }
}

impl Breakpoint {
    pub fn new() -> Breakpoint {
        Breakpoint {
            addr: 0,
            instruction: 0,
            mem_read_addr: 0,
            mem_write_addr: 0,
        }
    }

    pub fn set_bp(&mut self, addr: u64) {
        self.clear_bp();
        self.addr = addr;
    }

    pub fn clear_bp(&mut self) {
        self.addr = 0;
        self.mem_read_addr = 0;
        self.mem_write_addr = 0;
    }

    pub fn set_mem_read(&mut self, addr: u64) {
        self.clear_bp();
        self.mem_read_addr = addr;
    }

    pub fn set_mem_write(&mut self, addr: u64) {
        self.clear_bp();
        self.mem_write_addr = addr;
    }

    pub fn set_instruction(&mut self, ins: u64) {
        self.clear_bp();
        self.instruction = ins;
    }

    pub fn get_bp(&self) -> u64 {
        self.addr
    }

    pub fn get_mem_read(&self) -> u64 {
        self.mem_read_addr
    }

    pub fn get_mem_write(&self) -> u64 {
        self.mem_write_addr
    }

    pub fn get_instruction(&self) -> u64 {
        self.instruction
    }

    pub fn show(&self) {
        log::trace!("break on address: 0x{:x}", self.addr);
        log::trace!("break on instruction: {}", self.instruction);
        log::trace!("break on memory read: 0x{:x}", self.mem_read_addr);
        log::trace!("break on memory write: 0x{:x}", self.mem_write_addr);
    }
}
