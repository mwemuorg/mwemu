use crate::emu;

pub fn BasepInitializeApphelpGlobals(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!BasepInitializeApphelpGlobals");
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}
