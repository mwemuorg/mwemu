use crate::emu;
use crate::winapi::helper;

pub fn GetCurrentThread(emu: &mut emu::Emu) {
    log_red!(emu, "kernelbase!GetCurrentThread => -2");
    helper::advance_tick(emu, 1);
    emu.set_last_error(0);
    emu.regs_mut().rax = 0xFFFF_FFFF_FFFF_FFFE;
}
