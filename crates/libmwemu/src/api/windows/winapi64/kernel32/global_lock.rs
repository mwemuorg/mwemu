use crate::emu;

pub fn GlobalLock(emu: &mut emu::Emu) {
    let mem = emu.regs().rcx;

    log_red!(emu, "kernel32!GlobalLock mem: 0x{:x}", mem);

    emu.set_last_error(0);
    emu.regs_mut().rax = mem;
}
