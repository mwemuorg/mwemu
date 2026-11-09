use crate::emu;
use crate::winapi::helper;

pub fn GetTickCount64(emu: &mut emu::Emu) {
    let tick = helper::read_tick_count(emu);
    log_red!(emu, "kernel32!GetTickCount64 -> {}", tick);
    emu.regs_mut().rax = tick;
}
