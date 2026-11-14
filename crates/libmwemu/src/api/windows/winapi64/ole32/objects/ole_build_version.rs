use crate::emu;
use crate::winapi::helper;

pub fn OleBuildVersion(emu: &mut emu::Emu) {
    log_red!(emu, "ole32!OleBuildVersion");
    helper::advance_tick(emu, 1);
    emu.set_last_error(0);
    emu.regs_mut().rax = (23u64 << 16) | 639;
}
