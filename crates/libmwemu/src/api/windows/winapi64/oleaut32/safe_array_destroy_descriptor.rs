use crate::emu;
use crate::winapi::helper;

pub fn SafeArrayDestroyDescriptor(emu: &mut emu::Emu) {
    let sa = emu.regs().rcx;
    helper::handler_close(sa);
    log_red!(emu, "oleaut32!SafeArrayDestroyDescriptor 0x{:x}", sa);
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}
