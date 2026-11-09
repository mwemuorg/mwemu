use crate::emu;
use crate::winapi::helper;
pub fn FindVolumeMountPointClose(emu: &mut emu::Emu) {
    let h_find = emu.regs().rcx;
    log_red!(emu, "kernel32!FindVolumeMountPointClose h:0x{:x}", h_find);
    if helper::handler_exist(h_find) {
        helper::handler_close(h_find);
    }
    emu.regs_mut().rax = 1;
}
