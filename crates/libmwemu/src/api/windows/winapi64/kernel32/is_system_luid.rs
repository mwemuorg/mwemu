use crate::emu;

pub fn IsSystemLUID(emu: &mut emu::Emu) {
    let luid_ptr = emu.regs().rcx;
    log_red!(emu, "kernel32!IsSystemLUID luid:0x{:x} => FALSE", luid_ptr);
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}
