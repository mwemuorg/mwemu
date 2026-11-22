//! x86-family host-side calling-convention helpers.
//!
//! `call32`, Microsoft x64 `call64`, and System V `linux_call64` are the
//! three typed entry points used by the loader and the C/Python bindings.

use std::{mem, sync::atomic::Ordering};

use crate::api::abi::windows::{X64_REGISTER_ARGS, X64_STACK_ALIGNMENT};
use crate::emu::Emu;
use crate::err::MwemuError;

/// Maximum nesting depth for emulator-driven x86/call32/call64/linux_call64
/// invocations. Normal emulated `call` instructions do not increment this
/// counter — only explicit host-side calls (loader bootstrap, TLS callbacks,
/// etc.) do. A depth beyond this most likely indicates a bug in the
/// emulator infrastructure.
const MAX_CALL_DEPTH: u32 = 32;
const X64_STACK_SLOT_SIZE: u64 = size_of::<u64>() as u64;

impl Emu {
    /// Emulate a host-initiated call until `ret_addr` is reached.
    /// A successful run that stops elsewhere is an incomplete call, not a return.
    /// The caller may sit inside an instruction that already flagged a PC
    /// change (API gateways run from `set_rip_with_check`, which sets
    /// `force_reload` first). The nested run must not inherit those flags, or
    /// it skips the PC advance and executes the callee's first instruction twice.
    fn run_host_call(&mut self, ret_addr: u64) -> Result<(), MwemuError> {
        if self.call_depth >= MAX_CALL_DEPTH {
            return Err(MwemuError::new("call depth limit reached"));
        }
        let force_reload = mem::take(&mut self.force_reload);
        let force_break = mem::take(&mut self.force_break);
        let previous_return = self.host_call_return.replace(ret_addr);
        self.call_depth += 1;
        let result = self.run(Some(ret_addr));
        self.call_depth -= 1;
        self.force_reload = force_reload;
        self.force_break = force_break;
        self.host_call_return = previous_return;

        let pc = result?;
        self.check_api_stop()?;
        if self.process_terminated
            || self.is_running.load(Ordering::Relaxed) == 0
            || pc != ret_addr
            || self.pc() != ret_addr
        {
            return Err(MwemuError::new(&format!(
                "host call stopped at 0x{pc:x} before completing return to 0x{ret_addr:x}"
            )));
        }
        Ok(())
    }

    /// Call a 32bits function at addr, passing argument in an array of u64 but will cast to u32.
    /// The calling convention is stack, like winapi32.
    pub fn call32(&mut self, addr: u64, args: &[u32]) -> Result<u32, MwemuError> {
        self.check_api_stop()?;
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
        self.run_host_call(ret_addr)?;
        self.regs_mut().set_esp(orig_stack);
        Ok(self.regs().get_eax() as u32)
    }

    /// Call a function using the Microsoft x64 integer/pointer calling convention.
    /// Arguments use RCX, RDX, R8, R9, then stack slots above the shadow space.
    /// Restores RSP after a normal return; an execution error or early stop leaves
    /// the guest state at the failure point. Floating-point/aggregate arguments
    /// need a typed ABI adapter and are not marshalled by this interface.
    pub fn call64(&mut self, addr: u64, args: &[u64]) -> Result<u64, MwemuError> {
        self.check_api_stop()?;
        let ret_addr = if addr == self.regs().rip {
            if addr == 0 {
                return Err(MwemuError::new(
                    "return address reached after starting the call64, change rip.",
                ));
            }
            0
        } else {
            self.regs().rip
        };

        let orig_stack = self.regs().rsp;
        self.push_call64_frame(args, ret_addr)?;

        let regs = self.regs_mut();
        for (register, &value) in [&mut regs.rcx, &mut regs.rdx, &mut regs.r8, &mut regs.r9]
            .into_iter()
            .zip(args)
        {
            *register = value;
        }

        self.regs_mut().rip = addr;
        self.run_host_call(ret_addr)?;
        self.regs_mut().rsp = orig_stack;
        Ok(self.regs().rax)
    }

    fn push_call64_frame(&mut self, args: &[u64], ret_addr: u64) -> Result<(), MwemuError> {
        let orig_stack = self.regs().rsp;
        let payload_size = (args.len().max(X64_REGISTER_ARGS) as u64)
            .checked_mul(X64_STACK_SLOT_SIZE)
            .ok_or_else(|| MwemuError::new("call64 frame size overflow"))?;
        let unaligned_sp = orig_stack
            .checked_sub(payload_size)
            .ok_or_else(|| MwemuError::new("call64 stack underflow"))?;
        let padding = unaligned_sp % X64_STACK_ALIGNMENT;
        unaligned_sp
            .checked_sub(padding + X64_STACK_SLOT_SIZE)
            .ok_or_else(|| MwemuError::new("call64 stack underflow"))?;

        self.regs_mut().rsp = orig_stack - padding;
        // Push high addresses first: stack arguments, shadow slots, return address.
        let values = args
            .iter()
            .skip(X64_REGISTER_ARGS)
            .rev()
            .copied()
            .chain([0; X64_REGISTER_ARGS])
            .chain([ret_addr]);
        for value in values {
            if !self.stack_push64(value) {
                let address = self.regs().rsp;
                self.regs_mut().rsp = orig_stack;
                return Err(MwemuError::new(&format!(
                    "cannot write call64 frame at 0x{address:x}"
                )));
            }
        }
        Ok(())
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
        self.run_host_call(ret_addr)?;

        // recover stack and  return rax
        self.regs_mut().rsp = orig_stack;
        Ok(self.regs().rax)
    }
}
