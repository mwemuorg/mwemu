use crate::emu;
use crate::winapi::helper;

pub fn DispInvoke(emu: &mut emu::Emu) {
    let arg1 = emu.regs().rcx as u32;
    log_red!(emu, "oleaut32!DispInvoke 0x{:x}", arg1);
    helper::advance_tick(emu, 1);
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
