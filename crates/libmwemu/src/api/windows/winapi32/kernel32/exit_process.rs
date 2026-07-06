use crate::emu;

pub fn ExitProcess(emu: &mut emu::Emu) {
    let code = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("kernel32!ExitProcess cannot read the exit code");

    log_red!(emu, "kernel32!ExitProcess code: {}", code);
    emu.stack_pop32(false);

    emu.stop();
}
