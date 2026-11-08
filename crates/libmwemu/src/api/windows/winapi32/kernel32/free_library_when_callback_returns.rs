use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn FreeLibraryWhenCallbackReturns(emu: &mut emu::Emu) {
    let instance = arg(emu, 0) as u64;
    let module = arg(emu, 1) as u64;
    log_red!(
        emu,
        "kernel32!FreeLibraryWhenCallbackReturns instance:0x{:x} module:0x{:x}",
        instance,
        module
    );
    helper::handler_close(module);
    stdcall_ret(emu, 2);
}
