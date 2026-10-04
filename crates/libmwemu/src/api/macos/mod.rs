pub mod libc_extra;
pub mod libsystem;
pub mod process;
pub mod pthread;

/// Map holding return-address sentinels that stand in for dyld/libpthread
/// frames: returning from `main` exits, returning from a thread routine ends it.
pub const SENTINEL_MAP: &str = "mwemu_macos_sentinels";
pub const MAIN_RETURN: &str = "__mwemu_main_return";
pub const THREAD_RETURN: &str = "__mwemu_thread_return";
/// Sentinel symbols in map order, 4 bytes apart.
pub const SENTINELS: &[&str] = &[MAIN_RETURN, THREAD_RETURN];

/// Address of a sentinel, if the sentinel map has been created.
pub fn sentinel_addr(emu: &crate::emu::Emu, symbol: &str) -> Option<u64> {
    let base = emu.maps.get_map_by_name(SENTINEL_MAP)?.get_base();
    let idx = SENTINELS.iter().position(|s| *s == symbol)?;
    Some(base + 4 * idx as u64)
}

/// A sentinel's return value is already in the first argument register, so
/// each one is just the matching exit call.
fn sentinel(symbol: &str, emu: &mut crate::emu::Emu) {
    match symbol {
        MAIN_RETURN => libsystem::gateway("_exit", emu),
        THREAD_RETURN => libsystem::gateway("_pthread_exit", emu),
        _ => log::warn!("macosapi: unknown sentinel {}", symbol),
    }
}

/// Main gateway — dispatches macOS API calls by dylib section name and symbol.
///
/// In stub mode every library call is intercepted at the boundary and dispatched
/// by symbol name.  The section name tells us which dylib the symbol lives in,
/// but for dispatch purposes all system dylibs route to the same `libsystem`
/// gateway.
pub fn gateway(addr: u64, section_name: &str, symbol: &str, emu: &mut crate::emu::Emu) {
    match section_name {
        SENTINEL_MAP => sentinel(symbol, emu),
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
