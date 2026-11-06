use crate::emu;
use crate::winapi::helper;

pub fn GetCompatFlags(emu: &mut emu::Emu) {
    let compat_type = emu.regs().rcx;
    log_red!(emu, "kernel32!GetCompatFlags type:0x{:x} => 0", compat_type);
    helper::advance_tick(emu, 1);
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}
