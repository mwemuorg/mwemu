use crate::emu;
use crate::winapi::helper;

pub fn SetEvent(emu: &mut emu::Emu) {
    let handle = emu.regs().rcx;
    log_red!(emu, "kernel32!SetEvent handle: 0x{:x}", handle);
    helper::advance_tick(emu, 1);
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
