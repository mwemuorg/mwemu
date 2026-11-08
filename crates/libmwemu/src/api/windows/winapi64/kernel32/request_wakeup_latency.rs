use crate::emu;
use crate::winapi::helper;
use crate::windows::constants;

pub fn RequestWakeupLatency(emu: &mut emu::Emu) {
    let param1 = emu.regs().rcx;

    log_red!(emu, "kernel32!RequestWakeupLatency param1:0x{:x}", param1);
    helper::advance_tick(emu, 1);

    emu.set_last_error(0);
    emu.regs_mut().rax = constants::ERROR_SUCCESS;
}
