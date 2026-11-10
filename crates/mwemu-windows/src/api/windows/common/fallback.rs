use crate::emu::Emu;

/// Return a defined fallback value without touching argument buffers.
/// Gateways have already consumed the return address. Only x86 stdcall
/// removes arguments here; on x64 the caller owns the stack and shadow space.
pub fn return_value(
    addr: impl Into<u64>,
    api: &str,
    x86_words: u32,
    result: u64,
    emu: &mut Emu,
) -> String {
    let words = if emu.cfg.arch.is_64bits() {
        0
    } else {
        x86_words
    };
    log::warn!(
        "unimplemented API 0x{:x} {} (stack_words={}, result=0x{:x}) at 0x{:x}",
        addr.into(),
        api,
        words,
        result,
        emu.regs().rip
    );
    for _ in 0..words {
        emu.stack_pop32(false);
    }
    emu.regs_mut().rax = result;
    String::new()
}

/// Keep the opt-in banzai result. Only x86 needs a known callee cleanup size.
/// A zero address denotes a gateway that received only the import name.
pub fn unimplemented(addr: impl Into<u64>, api: &str, emu: &mut Emu) -> String {
    let addr = addr.into();
    // NT calls use only real handlers or the checked NT signature catalog.
    let native = api.starts_with("Nt") || api.starts_with("Zw");
    if emu.cfg.skip_unimplemented && !native {
        let words = if emu.cfg.arch.is_64bits() {
            Some(0)
        } else {
            u32::try_from(emu.banzai.get_params(api)).ok()
        };
        if let Some(words) = words {
            return return_value(addr, api, words, 0, emu);
        }
    }

    let reason = if native {
        "no supported NT signature; banzai fallback is disabled for NT calls"
    } else {
        "skipping is disabled or the stack cleanup is unknown"
    };
    log::error!(
        "stopping at unimplemented API 0x{:x} {} at 0x{:x}: {}",
        addr,
        api,
        emu.regs().rip,
        reason
    );
    if emu.cfg.dump_on_exit
        && let Some(filename) = emu.cfg.dump_filename.as_deref()
    {
        crate::layers::debugger().dump(emu, filename);
    }
    emu.stop();
    api.to_ascii_lowercase()
}
