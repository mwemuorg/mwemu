use crate::emu;

pub fn IsSyncForegroundPolicyRefresh(emu: &mut emu::Emu) {
    let arg1 = emu.regs().rcx;
    log_red!(
        emu,
        "kernelbase!IsSyncForegroundPolicyRefresh 0x{:x} => TRUE",
        arg1
    );
    emu.set_last_error(0);
    let h = emu.regs().rcx;
    emu.regs_mut().rax = 1;
}
