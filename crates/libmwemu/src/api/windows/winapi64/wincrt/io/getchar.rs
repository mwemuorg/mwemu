use crate::emu;
use crate::winapi::helper;

pub fn getchar(emu: &mut emu::Emu) {
    log_red!(emu, "wincrt!getchar");
    helper::advance_tick(emu, 1);
    emu.regs_mut().rax = 0x0A;
}
