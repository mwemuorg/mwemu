use crate::emu;

pub fn FreeEnvironmentStrings(emu: &mut emu::Emu) {
    let ptr = emu.regs().rcx;
    log_red!(emu, "kernel32!FreeEnvironmentStrings 0x{:x}", ptr);
    emu.regs_mut().rax = 1;
}
