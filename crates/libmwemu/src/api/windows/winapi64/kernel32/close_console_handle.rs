use crate::emu;

pub fn CloseConsoleHandle(emu: &mut emu::Emu) {
    let _a1 = emu.regs().rcx as u32;

    log_red!(emu, "kernel32!CloseConsoleHandle");

    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
