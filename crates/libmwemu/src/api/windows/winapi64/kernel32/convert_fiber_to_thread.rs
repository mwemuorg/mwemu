use crate::emu;

pub fn ConvertFiberToThread(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!ConvertFiberToThread");

    emu.set_last_error(0);
    emu.regs_mut().rax = 1; // TRUE
}
