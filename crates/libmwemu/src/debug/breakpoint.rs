use serde::{Deserialize, Serialize};

/// A single breakpoint with one trigger condition (address, instruction, or memory access).
#[derive(Clone, Serialize, Deserialize)]
pub struct Breakpoint {
    addr: u64,
    instruction: u64,
    mem_read_addr: u64,
    mem_write_addr: u64,
}

/// Collection of address, instruction, and memory breakpoints for the debugger.
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
    /// Create an empty breakpoint collection.
    pub fn new() -> Self {
        Breakpoints {
            addr: Vec::with_capacity(8),
            instruction: Vec::with_capacity(8),
            mem_read_addr: Vec::with_capacity(8),
            mem_write_addr: Vec::with_capacity(8),
        }
    }

    /// Check if an address breakpoint is set at `addr`.
    #[inline]
    pub fn is_bp(&self, addr: u64) -> bool {
        self.addr.contains(&addr)
    }

    /// Check if a memory-read breakpoint is set at `addr`.
    #[inline]
    pub fn is_bp_mem_read(&self, addr: u64) -> bool {
        self.mem_read_addr.contains(&addr)
    }

    /// Check if a memory-write breakpoint is set at `addr`.
    #[inline]
    pub fn is_bp_mem_write_addr(&self, addr: u64) -> bool {
        self.mem_write_addr.contains(&addr)
    }

    /// Check if an instruction breakpoint is set at `addr`.
    #[inline]
    pub fn is_bp_instruction(&self, addr: u64) -> bool {
        self.instruction.contains(&addr)
    }

    /// Add an address breakpoint.
    #[inline]
    pub fn add_bp(&mut self, addr: u64) {
        self.addr.push(addr)
    }

    /// Add a memory-read breakpoint.
    #[inline]
    pub fn add_bp_mem_read(&mut self, addr: u64) {
        self.mem_read_addr.push(addr);
    }

    /// Add a memory-write breakpoint.
    #[inline]
    pub fn add_bp_mem_write(&mut self, addr: u64) {
        self.mem_write_addr.push(addr);
    }

    /// Add an instruction breakpoint.
    #[inline]
    pub fn add_bp_instruction(&mut self, ins: u64) {
        self.instruction.push(ins);
    }

    /// Clear all breakpoints.
    pub fn clear_bp(&mut self) {
        self.addr.clear();
        self.mem_read_addr.clear();
        self.mem_write_addr.clear();
        self.mem_write_addr.clear();
    }

    /// Delete an address breakpoint by index.
    #[inline]
    pub fn delete_bp(&mut self, pos: usize) {
        self.addr.remove(pos);
    }

    /// Delete a memory-read breakpoint by index.
    #[inline]
    pub fn delete_bp_mem_read(&mut self, pos: usize) {
        self.mem_read_addr.remove(pos);
    }

    /// Delete a memory-write breakpoint by index.
    #[inline]
    pub fn delete_bp_mem_write(&mut self, pos: usize) {
        self.mem_write_addr.remove(pos);
    }

    /// Delete an instruction breakpoint by index.
    #[inline]
    pub fn delete_bp_instruction(&mut self, ins: usize) {
        self.instruction.remove(ins);
    }

    /// Log all breakpoints to trace output.
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
    /// Create a new breakpoint with no trigger set.
    pub fn new() -> Breakpoint {
        Breakpoint {
            addr: 0,
            instruction: 0,
            mem_read_addr: 0,
            mem_write_addr: 0,
        }
    }

    /// Set an address breakpoint, clearing any previous trigger.
    pub fn set_bp(&mut self, addr: u64) {
        self.clear_bp();
        self.addr = addr;
    }

    /// Clear all trigger conditions.
    pub fn clear_bp(&mut self) {
        self.addr = 0;
        self.mem_read_addr = 0;
        self.mem_write_addr = 0;
    }

    /// Set a memory-read breakpoint, clearing any previous trigger.
    pub fn set_mem_read(&mut self, addr: u64) {
        self.clear_bp();
        self.mem_read_addr = addr;
    }

    /// Set a memory-write breakpoint, clearing any previous trigger.
    pub fn set_mem_write(&mut self, addr: u64) {
        self.clear_bp();
        self.mem_write_addr = addr;
    }

    /// Set an instruction breakpoint, clearing any previous trigger.
    pub fn set_instruction(&mut self, ins: u64) {
        self.clear_bp();
        self.instruction = ins;
    }

    /// Get the address breakpoint value.
    pub fn get_bp(&self) -> u64 {
        self.addr
    }

    /// Get the memory-read breakpoint address.
    pub fn get_mem_read(&self) -> u64 {
        self.mem_read_addr
    }

    /// Get the memory-write breakpoint address.
    pub fn get_mem_write(&self) -> u64 {
        self.mem_write_addr
    }

    /// Get the instruction breakpoint value.
    pub fn get_instruction(&self) -> u64 {
        self.instruction
    }

    /// Log this breakpoint's trigger conditions to trace output.
    pub fn show(&self) {
        log::trace!("break on address: 0x{:x}", self.addr);
        log::trace!("break on instruction: {}", self.instruction);
        log::trace!("break on memory read: 0x{:x}", self.mem_read_addr);
        log::trace!("break on memory write: 0x{:x}", self.mem_write_addr);
    }
}
