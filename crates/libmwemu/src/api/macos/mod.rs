pub mod libc_extra;
pub mod libsystem;

/// Main gateway — dispatches macOS API calls by dylib section name and symbol.
///
/// In stub mode every library call is intercepted at the boundary and dispatched
/// by symbol name.  The section name tells us which dylib the symbol lives in,
/// but for dispatch purposes all system dylibs route to the same `libsystem`
/// gateway.
pub fn gateway(addr: u64, section_name: &str, symbol: &str, emu: &mut crate::emu::Emu) {
    match section_name {
        s if s.starts_with("libSystem.B.")
            || s.starts_with("libsystem_")
            || s.starts_with("libutil.")
            || s.starts_with("libncurses.")
            || s.starts_with("libc.")
            || s.starts_with("libclosure")
            || s.starts_with("libdyld.")
            || s.starts_with("libcorecrypto.")
            || s.starts_with("libcompiler_rt.")
            || s.starts_with("libcache.")
            || s.starts_with("libcommon")
            || s.starts_with("libcopyfile.")
            || s.starts_with("libdispatch.")
            || s.starts_with("libkeymgr.")
            || s.starts_with("libmacho.")
            || s.starts_with("libquarantine.")
            || s.starts_with("libremovefile.")
            || s.starts_with("libunwind.") =>
        {
            libsystem::gateway(symbol, emu)
        }
        _ => {
            log::warn!(
                "macosapi: unhandled call to {} in {} at 0x{:x}",
                symbol,
                section_name,
                addr
            );
            libsystem::set_ret_pub(emu, 0);
        }
    }
}
