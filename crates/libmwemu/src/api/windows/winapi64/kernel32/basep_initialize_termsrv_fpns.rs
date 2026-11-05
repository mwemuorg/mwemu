use crate::emu;

pub fn BasepInitializeTermsrvFpns(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!BasepInitializeTermsrvFpns");
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}
