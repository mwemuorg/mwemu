use crate::emu;
use crate::winapi::helper;
pub fn DisconnectNamedPipe(emu: &mut emu::Emu) {
    let handle = emu.regs().rcx;
    log_red!(emu, "kernel32!DisconnectNamedPipe hndl: 0x{:x}", handle);
    let h = emu.regs().rcx;
    if helper::handler_exist(h) {
        helper::handler_close(h);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
