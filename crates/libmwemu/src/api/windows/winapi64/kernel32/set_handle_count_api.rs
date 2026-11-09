use crate::emu;

pub fn SetHandleCount_api(emu: &mut emu::Emu) {
    let count = emu.regs().rcx;
    log_red!(emu, "kernel32!SetHandleCount count: {}", count);
    emu.set_last_error(0);
    emu.regs_mut().rax = count;
}
