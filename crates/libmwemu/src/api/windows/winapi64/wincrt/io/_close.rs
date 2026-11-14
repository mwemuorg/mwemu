use crate::emu;
use crate::winapi::helper;

pub fn _close(emu: &mut emu::Emu) {
    let fd = emu.regs().rcx as i32;
    log_red!(emu, "wincrt!_close fd:{}", fd);
    helper::advance_tick(emu, 1);
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}
