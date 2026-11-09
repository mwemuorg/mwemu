use crate::emu;
use crate::winapi::helper;

pub fn GetThreadLocale(emu: &mut emu::Emu) {
    log_red!(emu, "kernelbase!GetThreadLocale => 0x0409");
    helper::advance_tick(emu, 1);
    emu.set_last_error(0);
    emu.regs_mut().rax = 0x0409;
}
