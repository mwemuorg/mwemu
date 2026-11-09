use crate::emu;
use crate::winapi::helper;

pub fn ImpersonateLoggedOnUser(emu: &mut emu::Emu) {
    let token = emu.regs().rcx;
    log_red!(
        emu,
        "kernelbase!ImpersonateLoggedOnUser token:0x{:x}",
        token
    );
    if helper::handler_exist(token) {
        emu.regs_mut().rax = 1;
    } else {
        emu.regs_mut().rax = 0;
    }
    emu.set_last_error(0);
}
