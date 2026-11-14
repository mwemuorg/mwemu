use crate::emu;
use crate::winapi::helper;

pub fn _setmode(emu: &mut emu::Emu) {
    let fd = emu.regs().rcx as i32;
    let mode = emu.regs().rdx as i32;
    log_red!(emu, "wincrt!_setmode fd:{} mode:{}", fd, mode);
    helper::advance_tick(emu, 1);
    emu.set_last_error(0);
    emu.regs_mut().rax = mode as u64;
}
