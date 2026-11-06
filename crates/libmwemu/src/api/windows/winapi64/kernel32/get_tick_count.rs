use crate::emu;
use crate::winapi::helper;

pub fn GetTickCount(emu: &mut emu::Emu) {
    let tick = emu.tick as u64;
    log_red!(emu, "kernel32!GetTickCount -> {}", tick);
    helper::advance_tick(emu, 1);
    emu.regs_mut().rax = tick;
}
