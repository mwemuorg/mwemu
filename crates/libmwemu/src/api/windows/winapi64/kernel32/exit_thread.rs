use crate::emu;

pub fn ExitThread(emu: &mut emu::Emu) {
    let code = emu.regs().rcx;

    log_red!(emu, "kernel32!ExitThread code: {}", code);
    emu.stop();
}
