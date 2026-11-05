use crate::emu;

pub fn GetPriorityClass(emu: &mut emu::Emu) {
    let p0 = emu.regs().rcx;
    log_red!(emu, "kernel32!GetPriorityClass");
    emu.set_last_error(0);
    emu.regs_mut().rax = 0x20;
}
