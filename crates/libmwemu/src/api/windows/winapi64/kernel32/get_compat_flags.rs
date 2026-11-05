use crate::emu;

pub fn GetCompatFlags(emu: &mut emu::Emu) {
    let compat_type = emu.regs().rcx;
    log_red!(emu, "kernel32!GetCompatFlags type:0x{:x} => 0", compat_type);
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}
