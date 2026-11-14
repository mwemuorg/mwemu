use crate::emu;
use crate::winapi::helper;

pub fn CreateIconIndirect(emu: &mut emu::Emu) {
    let iconinfo_ptr = emu.regs().rcx;
    log_red!(emu, "user32!CreateIconIndirect info:0x{:x}", iconinfo_ptr);
    let hndl = helper::handler_create("user32://CreateIconIndirect");
    emu.regs_mut().rax = hndl;
}
