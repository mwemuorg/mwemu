use crate::emu;
use crate::winapi::helper;
pub fn LocalFree(emu: &mut emu::Emu) {
    let addr = emu.regs().rcx;
    log_red!(emu, "kernel32!LocalFree flags: {:x}", addr);
    let h = emu.regs().rcx;
    if helper::handler_exist(h) {
        helper::handler_close(h);
    }
    let heap_management = emu.heap_mut();
    heap_management.free(addr);
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}
