use crate::api::abi::unix::UnixApi;
use crate::api::unix;
pub mod libc;

/// Main gateway — dispatches Linux API calls by library section name and symbol.
pub fn gateway(addr: u64, section_name: &str, symbol: &str, emu: &mut crate::emu::Emu) {
    let call = UnixApi;
    if emu.api_stopped() {
        return;
    }
    let entered = unix::enter_gateway(emu, section_name, symbol, addr);
    match section_name {
        s if s.starts_with("libc.") || s.starts_with("libc-") => libc::gateway(symbol, emu),
        _ => {
            log::warn!(
                "linuxapi: unhandled call to {} in {} at 0x{:x}",
                symbol,
                section_name,
                addr
            );
            call.set_return(emu, 0);
        }
    }
    if entered {
        unix::end_call(emu, None);
    }
}
