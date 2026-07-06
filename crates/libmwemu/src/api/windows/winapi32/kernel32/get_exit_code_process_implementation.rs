use crate::emu;
use crate::windows::constants;

pub fn GetExitCodeProcessImplementation(emu: &mut emu::Emu) {
    let _hProcess = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("cannot read the api parameter");
    let _lpExitCode = emu
        .maps
        .read_dword(emu.regs().get_esp() + 4)
        .expect("cannot read the api parameter");

    log_red!(emu, "kernel32!GetExitCodeProcessImplementation");

    emu.regs_mut().rax = constants::ERROR_SUCCESS;

    for _ in 0..2 {
        emu.stack_pop32(false);
    }
}
