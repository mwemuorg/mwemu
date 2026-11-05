use crate::emu;
use crate::winapi::helper;

pub fn FreeLibraryAndExitThread(emu: &mut emu::Emu) {
    let module = emu.regs().rcx;
    let exit_code = emu.regs().rdx as u32;
    log_red!(
        emu,
        "kernel32!FreeLibraryAndExitThread module:0x{:x} exitCode:{}",
        module,
        exit_code
    );
    helper::handler_close(module);
}
