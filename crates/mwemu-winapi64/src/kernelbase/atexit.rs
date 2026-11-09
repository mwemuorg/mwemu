use crate::emu;

pub(super) fn atexit(emu: &mut emu::Emu) {
    let fptr = emu.regs().rcx;
    log_red!(emu, "kernelbase!atexit fptr: 0x{:x}", fptr);
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}
