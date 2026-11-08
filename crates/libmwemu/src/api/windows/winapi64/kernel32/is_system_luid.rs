use crate::emu;
use crate::winapi::helper;
pub fn IsSystemLUID(emu: &mut emu::Emu) {
    let luid_ptr = emu.regs().rcx;
    log_red!(emu, "kernel32!IsSystemLUID luid:0x{:x} => FALSE", luid_ptr);
    emu.set_last_error(0);
    let h = emu.regs().rcx;
    if helper::handler_exist(h) {
        emu.regs_mut().rax = 1;
    } else {
        emu.regs_mut().rax = 0;
    }
}
