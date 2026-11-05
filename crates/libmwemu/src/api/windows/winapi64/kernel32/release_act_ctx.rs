use crate::emu;

pub fn ReleaseActCtx(emu: &mut emu::Emu) {
    let param1 = emu.regs().rcx;

    log_red!(emu, "kernel32!ReleaseActCtx param1:0x{:x}", param1);

    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
