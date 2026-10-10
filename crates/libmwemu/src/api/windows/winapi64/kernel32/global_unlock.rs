use crate::emu;

pub fn GlobalUnlock(emu: &mut emu::Emu) {
    let mem = emu.regs().rcx;

    log_red!(emu, "kernel32!GlobalUnlock mem: 0x{:x}", mem);

    emu.regs_mut().rax = 1;
}
