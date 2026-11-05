use crate::emu;
use crate::winapi::helper;

pub fn FreeLibraryWhenCallbackReturns(emu: &mut emu::Emu) {
    let instance = emu.regs().rcx;
    let module = emu.regs().rdx;
    log_red!(
        emu,
        "kernel32!FreeLibraryWhenCallbackReturns instance:0x{:x} module:0x{:x}",
        instance,
        module
    );
    helper::handler_close(module);
}
