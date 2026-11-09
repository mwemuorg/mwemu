use crate::emu;
use crate::winapi::helper;

pub fn FindFirstVolumeW(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!FindFirstVolumeW");
    emu.regs_mut().rax = helper::handler_create("FindFirstVolumeW");
}
