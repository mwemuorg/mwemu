use crate::emu;
use crate::winapi::helper;

pub fn GetTextColor(emu: &mut emu::Emu) {
    let hdc = emu.regs().rcx;
    log_red!(emu, "gdi32!GetTextColor hdc:0x{:x}", hdc);
    helper::advance_tick(emu, 1);
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}
