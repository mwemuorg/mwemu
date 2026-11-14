use crate::emu;
use crate::winapi::helper;

pub fn FindVolumeClose(emu: &mut emu::Emu) {
    let handle = emu.regs().rcx;
    log_red!(emu, "kernelbase!FindVolumeClose 0x{:x}", handle);
    helper::handler_close(handle);
    emu.regs_mut().rax = 1;
}
