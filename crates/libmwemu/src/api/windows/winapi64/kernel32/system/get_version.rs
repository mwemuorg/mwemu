use crate::emu;
use crate::winapi::helper;

pub fn GetVersion(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!GetVersion");
    helper::advance_tick(emu, 1);
    emu.set_last_error(0);
    emu.regs_mut().rax = 0x4A61000A;
}
