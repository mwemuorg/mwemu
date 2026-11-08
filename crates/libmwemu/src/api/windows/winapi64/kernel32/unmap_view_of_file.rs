use crate::emu;
use crate::winapi::helper;
pub fn UnmapViewOfFile(emu: &mut emu::Emu) {
    let base_addr = emu.regs().rcx;
    log_red!(emu, "kernel32!UnmapViewOfFile addr:0x{:x}", base_addr);
    let h = emu.regs().rcx;
    if helper::handler_exist(h) {
        helper::handler_close(h);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
