pub mod libc;

/// Main gateway — dispatches Linux API calls by library section name and symbol.
pub fn gateway(addr: u64, section_name: &str, symbol: &str, emu: &mut crate::emu::Emu) {
    match section_name {
        s if s.starts_with("libc.") || s.starts_with("libc-") => libc::gateway(symbol, emu),
        _ => {
            log::warn!(
                "linuxapi: unhandled call to {} in {} at 0x{:x}",
                symbol,
                section_name,
                addr
            );
            todo!("Linux API: {} in {}", symbol, section_name);
        }
    }
}
