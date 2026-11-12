use crate::emu;
use crate::winapi::helper;

pub fn _putch(emu: &mut emu::Emu) {
    let ch = emu.regs().rcx as u8;
    log_red!(emu, "wincrt!_putch ch:{}", ch as char);
    helper::advance_tick(emu, 1);
    emu.regs_mut().rax = ch as u64;
}
