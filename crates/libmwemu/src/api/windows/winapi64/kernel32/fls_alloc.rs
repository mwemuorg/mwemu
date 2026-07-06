use crate::emu;

pub fn FlsAlloc(emu: &mut emu::Emu) {
    let callback = emu.regs().rcx;

    log_red!(emu, "kernel32!FlsAlloc callback: 0x{:x}", callback);

    emu.regs_mut().rax = 1;
}
