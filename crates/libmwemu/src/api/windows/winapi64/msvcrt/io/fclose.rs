use crate::emu;
use crate::winapi::helper;

pub fn fclose(emu: &mut emu::Emu) {
    let file = emu.regs().rcx;
    let filename = helper::handler_get_uri(file);
    helper::handler_close(file);
    log_red!(emu, "msvcrt!fclose '{}' handle:0x{:x}", filename, file);
    emu.regs_mut().rax = 0;
}
