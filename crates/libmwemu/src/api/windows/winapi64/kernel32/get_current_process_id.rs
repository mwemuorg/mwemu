use crate::emu;

pub fn GetCurrentProcessId(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!GetCurrentProcessId");

    emu.set_last_error(0);
    emu.regs_mut().rax = 0x123;
}
