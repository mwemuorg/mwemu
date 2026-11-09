use crate::emu;

pub fn SetConsoleCP(emu: &mut emu::Emu) {
    let code_page = emu.regs().rcx;

    log_red!(emu, "kernel32!SetConsoleCP cp:{}", code_page);
    let _ = emu.maps.is_mapped(emu.regs().rcx);

    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
