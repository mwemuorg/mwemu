//! AArch64 AAPCS64 calling-convention helper.

use crate::emu::Emu;
use crate::err::MwemuError;

/// Maximum nesting depth for emulator-driven AArch64 `aarch64_call64`
/// invocations. Only host-side calls (loader bootstrap, TLS callbacks, etc.)
/// increment this counter; ordinary emulated `bl` instructions do not.
const MAX_CALL_DEPTH: u32 = 32;

/// Call a 64-bit function using AArch64 AAPCS64 calling convention.
/// Args in x0-x7, return value in x0, LR = return address.
pub fn aarch64_call64(emu: &mut Emu, addr: u64, args: &[u64]) -> Result<u64, MwemuError> {
    let current_pc = emu.pc();
    if addr == current_pc {
        if addr == 0 {
            return Err(MwemuError::new(
                "return address reached after starting aarch64_call64, change pc.",
            ));
        } else {
            emu.set_pc(0);
        }
    }

    // Load args into x0-x7
    let n = args.len().min(8);
    for i in 0..n {
        emu.regs_aarch64_mut().x[i] = args[i];
    }
    if args.len() > 8 {
        log::warn!("aarch64_call64: more than 8 args not yet supported");
    }

    // Save SP
    let orig_sp = emu.regs_aarch64().sp;

    // 16-byte align SP
    let sp = emu.regs_aarch64().sp;
    let aligned_sp = sp & !0xF;
    emu.regs_aarch64_mut().sp = aligned_sp;

    // Set return address in LR (x30)
    let ret_addr = emu.pc();
    emu.regs_aarch64_mut().x[30] = ret_addr;

    // Jump to target
    emu.set_pc(addr);

    // Emulate the function until return address is reached
    if emu.call_depth >= MAX_CALL_DEPTH {
        return Err(MwemuError::new("call depth limit reached"));
    }
    emu.call_depth += 1;
    let result = emu.run(Some(ret_addr));
    emu.call_depth -= 1;
    result?;

    // Restore SP and return x0
    emu.regs_aarch64_mut().sp = orig_sp;
    Ok(emu.regs_aarch64().x[0])
}

impl Emu {
    /// AArch64 AAPCS64 host-side call helper.
    pub fn aarch64_call64(&mut self, addr: u64, args: &[u64]) -> Result<u64, MwemuError> {
        aarch64_call64(self, addr, args)
    }
}
