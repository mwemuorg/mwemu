use crate::emu;
use crate::winapi::helper;

pub fn GetTickCount(emu: &mut emu::Emu) {
    let tick = helper::read_tick_count(emu) as u32 as u64;
    log_red!(emu, "kernelbase!GetTickCount => {}", tick);
    emu.set_last_error(0);
    emu.regs_mut().rax = tick;
}
