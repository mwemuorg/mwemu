use crate::emu;

pub fn RaiseException(emu: &mut emu::Emu) {
    let code = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let flags = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let num_args = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
    let args = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);

    log_red!(emu, "kernel32!RaiseException {} {}", code, flags);

    for _ in 0..4 {
        emu.stack_pop32(false);
    }

    emu.regs_mut().rax = 0;
    //emu.stop();
}
