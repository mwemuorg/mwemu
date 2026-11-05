use crate::emu;

pub fn FreeEnvironmentStringsA(emu: &mut emu::Emu) {
    let env_block = emu.regs().rcx;
    log_red!(emu, "kernel32!FreeEnvironmentStringsA block:0x{:x}", env_block);
    emu.set_last_error(0);
    emu.regs_mut().rax = 1; // TRUE
}
