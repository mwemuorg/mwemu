//! The macOS return-address sentinels: the Mach-O loader maps them, and the
//! macOS layer handles a branch or return that lands on one.

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
