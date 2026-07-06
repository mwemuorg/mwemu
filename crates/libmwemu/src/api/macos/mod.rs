pub mod libsystem;

/// Main gateway — dispatches macOS API calls by dylib section name and symbol.
pub fn gateway(addr: u64, section_name: &str, symbol: &str, emu: &mut crate::emu::Emu) {
    match section_name {
        s if s.starts_with("libSystem.B.") => libsystem::gateway(symbol, emu),
        _ => {
            log::warn!(
                "macosapi: unhandled call to {} in {} at 0x{:x}",
                symbol,
                section_name,
                addr
            );
            todo!("macOS API: {} in {}", symbol, section_name);
        }
    }
}
