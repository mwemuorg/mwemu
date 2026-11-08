use crate::emu;
use crate::winapi::helper;
pub fn LZClose(emu: &mut emu::Emu) {
    let h_file = emu.regs().rcx;
    log_red!(emu, "kernel32!LZClose hFile:0x{:x}", h_file);
    let h = emu.regs().rcx;
    if helper::handler_exist(h) {
        helper::handler_close(h);
    }
    emu.set_last_error(0);
}
