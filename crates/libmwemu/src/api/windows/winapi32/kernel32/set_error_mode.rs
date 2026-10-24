use crate::emu;

pub fn SetErrorMode(emu: &mut emu::Emu) {
    let mode = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);

    log_red!(emu, "kernel32!SetErrorMode 0x{:x}", mode);

    emu.stack_pop32(false);

    emu.regs_mut().rax = 0;
}
