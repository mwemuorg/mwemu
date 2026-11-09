use crate::emu;
use crate::winapi::helper;

pub fn BlockInput(emu: &mut emu::Emu) {
    let arg1 = emu.regs().rcx;
    log_red!(emu, "user32!BlockInput 0x{:x}", arg1);
    helper::advance_tick(emu, 1);
    emu.set_last_error(0);
    let h = emu.regs().rcx;
    if helper::handler_exist(h) {
        emu.regs_mut().rax = 1;
    } else {
        emu.regs_mut().rax = 0;
    }
}
