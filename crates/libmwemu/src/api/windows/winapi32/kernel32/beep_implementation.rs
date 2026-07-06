use crate::emu;
use crate::windows::constants;

pub fn BeepImplementation(emu: &mut emu::Emu) {
    let _dwFreq = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("cannot read the api parameter");
    let _dwDuration = emu
        .maps
        .read_dword(emu.regs().get_esp() + 4)
        .expect("cannot read the api parameter");

    log_red!(emu, "kernel32!BeepImplementation");

    emu.regs_mut().rax = constants::ERROR_SUCCESS;

    for _ in 0..2 {
        emu.stack_pop32(false);
    }
}
