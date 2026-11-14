use crate::emu;
use crate::winapi::helper;

pub fn FreeIconList(emu: &mut emu::Emu) {
    let handle = emu.regs().rcx;
    helper::handler_close(handle);
    log_red!(emu, "shell32!FreeIconList handle:0x{:x}", handle);
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
