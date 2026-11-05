use crate::emu;

pub fn GetHandleContext(emu: &mut emu::Emu) {
    let handle = emu.regs().rcx;
    log_red!(emu, "kernel32!GetHandleContext h:0x{:x} => 0", handle);
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}
