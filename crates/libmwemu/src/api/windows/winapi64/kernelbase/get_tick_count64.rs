use crate::emu;
use crate::winapi::helper;

pub fn GetTickCount64(emu: &mut emu::Emu) {
    let tick = helper::read_tick_count(emu);
    log_red!(emu, "kernelbase!GetTickCount64 => {}", tick);
    emu.set_last_error(0);
    emu.regs_mut().rax = tick;
}
