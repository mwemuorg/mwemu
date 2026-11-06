use crate::emu;
use crate::winapi::helper;

pub fn GetTickCount64(emu: &mut emu::Emu) {
    let tick = emu.tick as u64;
    log_red!(emu, "kernel32!GetTickCount64 -> {}", tick);
    helper::advance_tick(emu, 1);
    emu.regs_mut().rax = tick;
}
