use crate::emu;
use crate::winapi::helper;

pub fn EnterCriticalSection(emu: &mut emu::Emu) {
    let cs_ptr = emu.regs().rcx;
    helper::advance_tick(emu, 1);
    log_red!(emu, "kernelbase!EnterCriticalSection 0x{:x}", cs_ptr);
    let handle = helper::handler_create("EnterCriticalSection");
}
