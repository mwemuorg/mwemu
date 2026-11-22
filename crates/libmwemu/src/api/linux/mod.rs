use crate::api::abi::unix::UnixApi;
use crate::api::{dispatch_api, unix};
use crate::emu::Emu;
pub mod libc;

/// Main gateway — dispatches Linux API calls by library section name and symbol.
pub fn gateway(addr: u64, section_name: &str, symbol: &str, emu: &mut Emu) {
    if emu.api_stopped() {
        return;
    }
    let entered = unix::enter_gateway(emu, section_name, symbol, addr);
    dispatch_api!(section_name, {
        s if s.starts_with("libc.") || s.starts_with("libc-") => libc::gateway(symbol, emu),
    },
        handled(()) => (),
        fallback => unsupported_library(emu, addr, section_name, symbol),
    );
    if entered {
        unix::end_call(emu, None);
    }
}

fn unsupported_library(emu: &mut Emu, addr: u64, section_name: &str, symbol: &str) {
    let call = UnixApi;
    log::warn!(
        "linuxapi: unhandled call to {} in {} at 0x{:x}",
        symbol,
        section_name,
        addr
    );
    call.set_return(emu, 0);
}
