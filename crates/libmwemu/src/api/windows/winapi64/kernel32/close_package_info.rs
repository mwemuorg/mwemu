use crate::emu;
use crate::winapi::helper;

pub fn ClosePackageInfo(emu: &mut emu::Emu) {
    let handle = emu.regs().rcx;
    log_red!(emu, "kernel32!ClosePackageInfo h:0x{:x}", handle);
    helper::handler_close(handle);
    emu.regs_mut().rax = 0; // ERROR_SUCCESS
}
