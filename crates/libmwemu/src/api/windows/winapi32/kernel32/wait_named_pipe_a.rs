use crate::emu;
use crate::windows::constants;

pub fn WaitNamedPipeA(emu: &mut emu::Emu) {
    let lpNamedPipeName = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("cannot read the api parameter");
    let _nTimeOut = emu
        .maps
        .read_dword(emu.regs().get_esp() + 4)
        .expect("cannot read the api parameter");

    let lpNamedPipeName = emu.maps.read_string(lpNamedPipeName as u64);

    log_red!(emu, "kernel32!WaitNamedPipeA {}", lpNamedPipeName);

    emu.regs_mut().rax = constants::ERROR_SUCCESS;

    for _ in 0..2 {
        emu.stack_pop32(false);
    }
}
