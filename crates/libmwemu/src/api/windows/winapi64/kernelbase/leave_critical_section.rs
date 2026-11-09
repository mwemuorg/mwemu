use crate::emu;
use crate::winapi::helper;

pub fn LeaveCriticalSection(emu: &mut emu::Emu) {
    let cs_ptr = emu.regs().rcx;
    helper::advance_tick(emu, 1);
    log_red!(emu, "kernelbase!LeaveCriticalSection 0x{:x}", cs_ptr);
    let h = emu.regs().rcx;
    if helper::handler_exist(h) {
        helper::handler_close(h);
    }
}
