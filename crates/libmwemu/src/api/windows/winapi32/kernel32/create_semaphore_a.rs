use crate::emu;
use crate::windows::constants;

pub fn CreateSemaphoreA(emu: &mut emu::Emu) {
    let _lpSemaphoreAttributes = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("cannot read the api parameter");
    let _lInitialCount = emu
        .maps
        .read_dword(emu.regs().get_esp() + 4)
        .expect("cannot read the api parameter");
    let _lMaximumCount = emu
        .maps
        .read_dword(emu.regs().get_esp() + 8)
        .expect("cannot read the api parameter");
    let lpName = emu
        .maps
        .read_dword(emu.regs().get_esp() + 12)
        .expect("cannot read the api parameter");

    let lpName = emu.maps.read_string(lpName as u64);

    log_red!(emu, "kernel32!CreateSemaphoreA {}", lpName);

    emu.regs_mut().rax = constants::ERROR_SUCCESS;

    for _ in 0..4 {
        emu.stack_pop32(false);
    }
}
