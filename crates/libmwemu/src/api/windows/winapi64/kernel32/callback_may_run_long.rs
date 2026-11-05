use crate::emu;

pub fn CallbackMayRunLong(emu: &mut emu::Emu) {
    let param1 = emu.regs().rcx;
    let param2 = emu.regs().rdx;

    log_red!(emu, "kernel32!CallbackMayRunLong p1:0x{:x}", param1);

    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
