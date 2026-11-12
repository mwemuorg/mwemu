use crate::emu;

pub fn FreeEnvironmentStringsA(emu: &mut emu::Emu) {
    free_environment_strings(emu, "FreeEnvironmentStringsA");
}

fn free_environment_strings(emu: &mut emu::Emu, api: &str) {
    let _penv = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);

    emu.stack_pop32(false);

    log_red!(emu, "kernel32!{api}");

    emu.regs_mut().rax = 1;
}

pub fn FreeEnvironmentStringsW(emu: &mut emu::Emu) {
    free_environment_strings(emu, "FreeEnvironmentStringsW");
}
