//! x86-family host-side calling-convention helpers.
//!
//! `call32`, Microsoft x64 `call64`, and System V `linux_call64` are the
//! three typed entry points used by the loader and the C/Python bindings.

use crate::emu::Emu;
use crate::err::MwemuError;

/// Maximum nesting depth for emulator-driven x86/call32/call64/linux_call64
/// invocations. Normal emulated `call` instructions do not increment this
/// counter — only explicit host-side calls (loader bootstrap, TLS callbacks,
/// etc.) do. A depth beyond this most likely indicates a bug in the
/// emulator infrastructure.
const MAX_CALL_DEPTH: u32 = 32;

impl Emu {
    /// Call a 32bits function at addr, passing argument in an array of u64 but will cast to u32.
    /// The calling convention is stack, like winapi32.
    pub fn call32(&mut self, addr: u64, args: &[u32]) -> Result<u32, MwemuError> {
        if addr == self.regs().get_eip() {
            if addr == 0 {
                return Err(MwemuError::new(
                    "return address reached after starting the call32, change eip.",
                ));
            } else {
                self.regs_mut().rip = 0;
            }
        }
        let orig_stack = self.regs().get_esp();
        for arg in args.iter().rev() {
            self.stack_push32(*arg);
        }
        let ret_addr = self.regs().get_eip();
        self.stack_push32(ret_addr as u32);
        self.regs_mut().set_eip(addr);
        if self.call_depth >= MAX_CALL_DEPTH {
            return Err(MwemuError::new("call depth limit reached"));
        }
        self.call_depth += 1;
        let result = self.run(Some(ret_addr));
        self.call_depth -= 1;
        result?;
        self.regs_mut().set_esp(orig_stack);
        Ok(self.regs().get_eax() as u32)
    }

    /// Call 64bits function at addr using Microsoft x64 ABI, passing argument in an array of u64.
    /// The calling convention is registers rcx/rdx/r8/r9 and then stack. Like windows64.
    /// Dont use for linux64 syscall like convention, for this is linux_call64()
    pub fn call64(&mut self, addr: u64, args: &[u64]) -> Result<u64, MwemuError> {
        if addr == self.regs().rip {
            if addr == 0 {
                return Err(MwemuError::new(
                    "return address reached after starting the call64, change rip.",
                ));
            } else {
                self.regs_mut().rip = 0;
            }
        }

        let n = args.len();
        if n >= 1 {
            self.regs_mut().rcx = args[0];
        }
        if n >= 2 {
            self.regs_mut().rdx = args[1];
        }
        if n >= 3 {
            self.regs_mut().r8 = args[2];
        }
        if n >= 4 {
            self.regs_mut().r9 = args[3];
        }

        // stack pointer backup, for restoring when function returns.
        let orig_stack = self.regs().rsp;

        // padding
        let extra_args = if n > 4 { (n - 4) * 8 } else { 0 };
        let total = extra_args + 32 + 8;
        let padding = (16 - (self.regs().rsp as usize + total) % 16) % 16;
        self.regs_mut().rsp -= padding as u64;

        // shadow space (32bits)
        for _ in 0..4 {
            self.stack_push64(0);
        }

        // stack parameters
        if n > 4 {
            for arg in args.iter().skip(4).rev() {
                self.stack_push64(*arg);
            }
        }

        // return address
        let ret_addr = self.regs().rip;
        self.stack_push64(ret_addr);

        // trigger function
        self.regs_mut().rip = addr;

        // emulate the function until return address is reached
        if self.call_depth >= MAX_CALL_DEPTH {
            return Err(MwemuError::new("call depth limit reached"));
        }
        self.call_depth += 1;
        let result = self.run(Some(ret_addr));
        self.call_depth -= 1;
        result?;

        // recover stack and  return rax
        self.regs_mut().rsp = orig_stack;
        Ok(self.regs().rax)
    }

    /// Call a 64bits function at addr, passing arguments in an array of u64.
    /// The calling convention is registers RDI, RSI, RDX, RCX, R8, R9 and then stack. Like linux64.
    pub fn linux_call64(&mut self, addr: u64, args: &[u64]) -> Result<u64, MwemuError> {
        if addr == self.regs().rip {
            if addr == 0 {
                return Err(MwemuError::new(
                    "return address reached after starting the call64, change rip.",
                ));
            } else {
                self.regs_mut().rip = 0;
            }
        }

        let n = args.len();
        if n >= 1 {
            self.regs_mut().rdi = args[0];
        }
        if n >= 2 {
            self.regs_mut().rsi = args[1];
        }
        if n >= 3 {
            self.regs_mut().rdx = args[2];
        }
        if n >= 4 {
            self.regs_mut().rcx = args[3];
        }
        if n >= 5 {
            self.regs_mut().r8 = args[4];
        }
        if n >= 6 {
            self.regs_mut().r9 = args[5];
        }

        // stack pointer backup, for restoring when function returns.
        let orig_stack = self.regs().rsp;

        // padding
        let extra_args = if n > 6 { (n - 6) * 8 } else { 0 };
        let total = extra_args + 8;
        let padding = (16 - (self.regs().rsp as usize + total) % 16) % 16;
        self.regs_mut().rsp -= padding as u64;

        // stack parameters
        if n > 6 {
            for arg in args.iter().skip(6).rev() {
                self.stack_push64(*arg);
            }
        }

        // return address
        let ret_addr = self.regs().rip;
        self.stack_push64(ret_addr);

        // trigger function
        self.regs_mut().rip = addr;

        // emulate the function until return address is reached
        if self.call_depth >= MAX_CALL_DEPTH {
            return Err(MwemuError::new("call depth limit reached"));
        }
        self.call_depth += 1;
        let result = self.run(Some(ret_addr));
        self.call_depth -= 1;
        result?;

        // recover stack and  return rax
        self.regs_mut().rsp = orig_stack;
        Ok(self.regs().rax)
    }
}
