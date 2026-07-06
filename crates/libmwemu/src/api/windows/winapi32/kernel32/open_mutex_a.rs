use crate::emu;
use crate::windows::constants;

pub fn OpenMutexA(emu: &mut emu::Emu) {
    let _dwDesiredAccess = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("cannot read the api parameter");
    let _bInheritHandle = emu
        .maps
        .read_dword(emu.regs().get_esp() + 4)
        .expect("cannot read the api parameter");
    let lpName = emu
        .maps
        .read_dword(emu.regs().get_esp() + 8)
        .expect("cannot read the api parameter");

    let lpName = emu.maps.read_string(lpName as u64);

    log_red!(emu, "kernel32!OpenMutexA {}", lpName);

    emu.regs_mut().rax = constants::ERROR_SUCCESS;

    for _ in 0..3 {
        emu.stack_pop32(false);
    }
}
