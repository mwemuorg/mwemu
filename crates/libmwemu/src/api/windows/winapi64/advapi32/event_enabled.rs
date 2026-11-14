use crate::emu;
use crate::winapi::helper;

pub fn EventEnabled(emu: &mut emu::Emu) {
    let arg1 = emu.regs().rcx;
    let arg2 = emu.regs().rdx;
    log_red!(emu, "advapi32!EventEnabled 0x{:x}", arg1);
    helper::advance_tick(emu, 1);
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
