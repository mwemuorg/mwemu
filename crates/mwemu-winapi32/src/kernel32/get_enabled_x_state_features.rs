use crate::emu;
use crate::winapi::helper;

pub fn get_enabled_x_state_features(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!GetEnabledXStateFeatures");
    helper::advance_tick(emu, 1);
    emu.set_last_error(0);
    emu.regs_mut().rax = 0x7; // x87 + SSE + AVX
}
