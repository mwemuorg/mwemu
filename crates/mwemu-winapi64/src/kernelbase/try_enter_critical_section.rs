use crate::emu;
use crate::winapi::helper;

pub fn TryEnterCriticalSection(emu: &mut emu::Emu) {
    let cs_ptr = emu.regs().rcx;
    helper::advance_tick(emu, 1);
    log_red!(emu, "kernelbase!TryEnterCriticalSection 0x{:x}", cs_ptr);
    let handle = helper::handler_create("TryEnterCriticalSection");
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
