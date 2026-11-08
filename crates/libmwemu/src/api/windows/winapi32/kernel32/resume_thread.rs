use crate::emu;

pub fn ResumeThread(emu: &mut emu::Emu) {
    let hndl = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);

    log_red!(emu, "kernel32!ResumeThread hndl: {}", hndl);

    emu.stack_pop32(false);

    emu.regs_mut().rax = 1; // previous suspend count
}
