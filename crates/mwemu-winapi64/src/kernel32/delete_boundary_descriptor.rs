use crate::emu;
use crate::winapi::helper;

pub fn DeleteBoundaryDescriptor(emu: &mut emu::Emu) {
    let handle = emu.regs().rcx;
    log_red!(emu, "kernel32!DeleteBoundaryDescriptor h:0x{:x}", handle);
    helper::handler_close(handle);
}
