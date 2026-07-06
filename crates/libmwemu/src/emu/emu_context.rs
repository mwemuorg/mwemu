use std::cell::RefCell;

use crate::emu::Emu;
use crate::emu::decoded_instruction::DecodedInstruction;

/// Format a decoded instruction read-only (with a throwaway formatter), so we
/// don't need `&mut Emu` just to render one line. Used by the panic-time state
/// dump; the per-call formatter allocation is irrelevant on that path.
fn format_decoded(decoded: &DecodedInstruction) -> String {
    match decoded {
        DecodedInstruction::X86(ins) => {
            use iced_x86::{Formatter as _, IntelFormatter};
            let mut formatter = IntelFormatter::new();
            let mut out = String::new();
            formatter.format(ins, &mut out);
            out
        }
        DecodedInstruction::AArch64(ins) => format!("{}", ins),
    }
}

thread_local! {
    static CURRENT_EMU: RefCell<Option<*const Emu>> = const { RefCell::new(None) };
}

pub fn with_current_emu<F, R>(f: F) -> Option<R>
where
    F: FnOnce(&Emu) -> R,
{
    CURRENT_EMU.with(|current| {
        current
            .borrow()
            // SAFETY: the stored pointer (if any) was set by `set_current_emu`
            // from a live `&Emu` and is cleared via `clear_current_emu` before
            // that `Emu` is dropped, so it is non-null and points at a live
            // value here. `f` only gets a shared `&Emu`.
            //
            // CAVEAT: this reconstructs a `&Emu` from a raw pointer while the
            // owner may simultaneously hold a `&mut Emu` further up the stack
            // (e.g. the run loop). That is the fragile part of this global-
            // pointer pattern — see the module note in the issue. It is only
            // used for read-only logging of emulator state.
            .and_then(|ptr| unsafe { ptr.as_ref().map(f) })
    })
}

pub fn set_current_emu(emu: &Emu) {
    CURRENT_EMU.with(|current| {
        *current.borrow_mut() = Some(emu as *const _);
    });
}

pub fn clear_current_emu() {
    CURRENT_EMU.with(|current| {
        *current.borrow_mut() = None;
    });
}

pub fn is_emu_set() -> bool {
    CURRENT_EMU.with(|current| current.borrow().is_some())
}

pub fn log_emu_state(emu: &Emu) {
    log::error!("=== EMULATOR STATE ===");
    log::error!("Current position: {}", emu.pos);

    let color = "\x1b[0;31m";
    if let Some(decoded) = emu.last_decoded {
        let out = format_decoded(&decoded);
        // Use `last_decoded_addr` (where the instruction lived) instead of
        // `emu.pc()` (which already reflects the *next* instruction after
        // a `ret`/branch) so the dump shows e.g.
        //   `2 0x10000036c: ret`
        // instead of `2 0x0: ret`.
        log::trace!(
            "{}{} 0x{:x}: {}{}",
            color,
            emu.pos,
            emu.last_decoded_addr,
            out,
            emu.colors.nc
        );
    }

    // Log registers
    log::error!("Registers:");
    if emu.cfg.arch.is_aarch64() {
        let regs = emu.regs_aarch64();
        for i in 0..31 {
            if i % 2 == 0 && i + 1 < 31 {
                log::error!(
                    "  X{:<2}: 0x{:016x}  X{:<2}: 0x{:016x}",
                    i,
                    regs.x[i],
                    i + 1,
                    regs.x[i + 1]
                );
            } else if i % 2 == 0 {
                log::error!("  X{:<2}: 0x{:016x}", i, regs.x[i]);
            }
        }
        log::error!("  SP:  0x{:016x}", regs.sp);
        log::error!("  PC:  0x{:016x}", regs.pc);
        log::error!(
            "  NZCV: N={} Z={} C={} V={}",
            regs.nzcv.n,
            regs.nzcv.z,
            regs.nzcv.c,
            regs.nzcv.v
        );
    } else {
        log::error!(
            "  RAX: 0x{:016x}  RBX: 0x{:016x}",
            emu.regs().rax,
            emu.regs().rbx
        );
        log::error!(
            "  RCX: 0x{:016x}  RDX: 0x{:016x}",
            emu.regs().rcx,
            emu.regs().rdx
        );
        log::error!(
            "  RSI: 0x{:016x}  RDI: 0x{:016x}",
            emu.regs().rsi,
            emu.regs().rdi
        );
        log::error!(
            "  RBP: 0x{:016x}  RSP: 0x{:016x}",
            emu.regs().rbp,
            emu.regs().rsp
        );
        log::error!(
            "  R8:  0x{:016x}  R9:  0x{:016x}",
            emu.regs().r8,
            emu.regs().r9
        );
        log::error!(
            "  R10: 0x{:016x}  R11: 0x{:016x}",
            emu.regs().r10,
            emu.regs().r11
        );
        log::error!(
            "  R12: 0x{:016x}  R13: 0x{:016x}",
            emu.regs().r12,
            emu.regs().r13
        );
        log::error!(
            "  R14: 0x{:016x}  R15: 0x{:016x}",
            emu.regs().r14,
            emu.regs().r15
        );
        log::error!("  RIP: 0x{:016x}", emu.regs().rip);

        // Log flags
        log::error!("EFLAGS: 0x{:08x}", emu.flags_snapshot().dump());
    }

    // Log last instruction if available
    if let Some(decoded) = emu.last_decoded {
        let out = format_decoded(&decoded);
        log::error!("Last instruction: {}", out);
        log::error!("Instruction size: {}", emu.last_instruction_size);
    }

    // Log call stack — only x86 has a tracked call stack; aarch64 emu would
    // panic on `call_stack()` (see `crates/libmwemu/src/emu/call_stack.rs`).
    if !emu.cfg.arch.is_aarch64() && !emu.call_stack().is_empty() {
        log::error!(
            "Call stack (last {} entries):",
            emu.call_stack().len().min(10)
        );
        for (i, entry) in emu.call_stack().iter().rev().take(10).enumerate() {
            log::error!("  {}: {:x}:call:{:x}", i, entry.0, entry.1);
        }
    }

    // Log execution info
    log::error!("Tick count: {}", emu.tick);
    log::error!("Base address: 0x{:x}", emu.base);
    log::error!("Filename: {}", emu.filename);

    log::error!("==============================");
}
