use crate::emu;

pub fn BasepGetMiniVersionForCreate(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!BasepGetMiniVersionForCreate");
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}
