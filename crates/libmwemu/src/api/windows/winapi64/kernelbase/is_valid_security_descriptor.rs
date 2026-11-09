use crate::emu;
use crate::winapi::helper;

pub fn IsValidSecurityDescriptor(emu: &mut emu::Emu) {
    let arg1 = emu.regs().rcx;
    log_red!(
        emu,
        "kernelbase!IsValidSecurityDescriptor 0x{:x} => TRUE",
        arg1
    );
    emu.set_last_error(0);
    let h = emu.regs().rcx;
    if helper::handler_exist(h) {
        emu.regs_mut().rax = 1;
    } else {
        emu.regs_mut().rax = 0;
    }
}
