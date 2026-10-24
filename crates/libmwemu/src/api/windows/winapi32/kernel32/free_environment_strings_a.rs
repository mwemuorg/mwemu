use crate::emu;

pub fn FreeEnvironmentStringsA(emu: &mut emu::Emu) {
    let _penv = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);

    emu.stack_pop32(false);

    log_red!(emu, "kernel32!FreeEnvironmentStringsA");

    emu.regs_mut().rax = 1;
}
