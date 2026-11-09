//! Entry points into the Windows API handlers.
//!
//! The handlers are a layer on top of the emulator core: they need `Emu`, so
//! the core cannot name them. Whoever links the handlers in registers their
//! entry points here once, and the core calls them through this table.
//! `libmwemu` does that in `emu32()`, `emu64()`, `emu_aarch64()` and
//! `register_api_handlers()`.

use std::sync::OnceLock;

use crate::emu::Emu;

/// The functions the emulator core calls in the Windows API handlers.
#[derive(Clone, Copy)]
pub struct ApiDispatch {
    /// Run the 32-bit handler of the API at `addr`, in the map named `section`.
    pub winapi32_gateway: fn(addr: u32, section: &str, emu: &mut Emu),
    /// Run the 64-bit handler of the API at `addr`, in the map named `section`.
    pub winapi64_gateway: fn(addr: u64, section: &str, emu: &mut Emu),
    /// Run the 64-bit handler of an import whose IAT slot is still unbound.
    pub winapi64_gateway_by_import: fn(emu: &mut Emu, dll: &str, api: &str, addr: u64),
    /// 64-bit `kernel32!LoadLibraryA`, for the `--ssdt` entry-point shim.
    pub winapi64_load_library_a: fn(emu: &mut Emu),
    /// 64-bit `kernel32!GetProcAddress`, for the `--ssdt` entry-point shim.
    pub winapi64_get_proc_address: fn(emu: &mut Emu),
}

static DISPATCH: OnceLock<ApiDispatch> = OnceLock::new();

/// Register the Windows API handlers for this process. The first call wins;
/// later calls are ignored.
pub fn register(dispatch: ApiDispatch) {
    let _ = DISPATCH.set(dispatch);
}

pub fn winapi32_gateway(addr: u32, section: &str, emu: &mut Emu) {
    match DISPATCH.get() {
        Some(dispatch) => (dispatch.winapi32_gateway)(addr, section, emu),
        None => no_handlers(emu),
    }
}

pub fn winapi64_gateway(addr: u64, section: &str, emu: &mut Emu) {
    match DISPATCH.get() {
        Some(dispatch) => (dispatch.winapi64_gateway)(addr, section, emu),
        None => no_handlers(emu),
    }
}

pub fn winapi64_gateway_by_import(emu: &mut Emu, dll: &str, api: &str, addr: u64) {
    match DISPATCH.get() {
        Some(dispatch) => (dispatch.winapi64_gateway_by_import)(emu, dll, api, addr),
        None => no_handlers(emu),
    }
}

pub fn winapi64_load_library_a(emu: &mut Emu) {
    match DISPATCH.get() {
        Some(dispatch) => (dispatch.winapi64_load_library_a)(emu),
        None => no_handlers(emu),
    }
}

pub fn winapi64_get_proc_address(emu: &mut Emu) {
    match DISPATCH.get() {
        Some(dispatch) => (dispatch.winapi64_get_proc_address)(emu),
        None => no_handlers(emu),
    }
}

/// A Windows API was reached with no handlers registered: carrying on would
/// run the guest with whatever the missing API left in the registers.
fn no_handlers(emu: &mut Emu) {
    log::error!(
        "no Windows API handlers registered at 0x{:x}: create the emulator with \
         libmwemu::emu32()/emu64()/emu_aarch64(), or call \
         libmwemu::register_api_handlers() first",
        emu.regs().rip
    );
    emu.stop();
}
