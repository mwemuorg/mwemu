use crate::emu;
use crate::winapi::helper;
pub fn InitializeCriticalSection(emu: &mut emu::Emu) {
    let ptr_crit_sect = emu.regs().rcx;
    log_red!(
        emu,
        "kernel32!InitializeCriticalSection ptr: 0x{:x}",
        ptr_crit_sect
    );
    let handle = helper::handler_create("InitializeCriticalSection");
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
