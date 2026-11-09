use crate::emu;

pub fn FlsFree(emu: &mut emu::Emu) {
    let idx = emu.regs().rcx;
    log_red!(emu, "kernel32!FlsFree idx: {}", idx);
    if (idx as usize) < emu.fls().len() {
        emu.fls_mut()[idx as usize] = 0;
        emu.set_last_error(0);
        emu.regs_mut().rax = 1;
    } else {
        emu.set_last_error(0);
        emu.regs_mut().rax = 0;
    }
}
