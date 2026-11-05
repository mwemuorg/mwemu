use crate::emu;

pub fn TlsFree(emu: &mut emu::Emu) {
    let idx = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);

    log_red!(emu, "kernel32!TlsFree idx: {}", idx);

    emu.set_last_error(0);
    emu.stack_pop32(false);
    emu.regs_mut().set_eax(1);
}
