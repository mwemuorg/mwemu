use crate::emu;
use crate::winapi::helper;
pub fn FreeEnvironmentStrings(emu: &mut emu::Emu) {
    let env_block = emu.regs().rcx;
    log_red!(
        emu,
        "kernel32!FreeEnvironmentStrings block:0x{:x}",
        env_block
    );
    let h = emu.regs().rcx;
    if helper::handler_exist(h) {
        helper::handler_close(h);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1; // TRUE
}
