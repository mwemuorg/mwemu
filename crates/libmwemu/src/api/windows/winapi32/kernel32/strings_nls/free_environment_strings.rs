use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn FreeEnvironmentStringsA(emu: &mut emu::Emu) {
    free_environment_strings(emu, "FreeEnvironmentStringsA");
}

fn free_environment_strings(emu: &mut emu::Emu, api: &str) {
    let _penv = WinApiAbi::arg(emu, 0) as u32;

    emu.stack_pop32(false);

    log_red!(emu, "kernel32!{api}");

    emu.regs_mut().rax = 1;
}

pub fn FreeEnvironmentStringsW(emu: &mut emu::Emu) {
    free_environment_strings(emu, "FreeEnvironmentStringsW");
}
