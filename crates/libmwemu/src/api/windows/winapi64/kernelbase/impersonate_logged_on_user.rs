use crate::emu;

pub fn ImpersonateLoggedOnUser(emu: &mut emu::Emu) {
    let token = emu.regs().rcx;
    log_red!(
        emu,
        "kernelbase!ImpersonateLoggedOnUser token:0x{:x}",
        token
    );
    emu.regs_mut().rax = 1;
    emu.set_last_error(0);
}
