use crate::emu;
use crate::winapi::helper;

pub fn GetMessageTime(emu: &mut emu::Emu) {
    helper::advance_tick(emu, 1);
    let tick = emu.tick as u64;
    log_red!(emu, "user32!GetMessageTime => {}", tick);
    emu.set_last_error(0);
    emu.regs_mut().rax = tick;
}
