use crate::emu;

pub fn GetSystemDEPPolicy(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!GetSystemDEPPolicy => 2 (OptIn)");
    emu.set_last_error(0);
    emu.regs_mut().rax = 2;
}
