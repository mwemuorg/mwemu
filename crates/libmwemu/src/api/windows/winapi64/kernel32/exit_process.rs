use crate::emu;

pub fn ExitProcess(emu: &mut emu::Emu) {
    let code = emu.regs().rcx;

    log_red!(emu, "kernel32!ExitProcess code: {}", code);
    emu.stop();
}
