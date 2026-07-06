use crate::emu;
use crate::windows::constants;

pub fn InterlockedCompareExchange(emu: &mut emu::Emu) {
    let _Destination = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("cannot read the api parameter");
    let _Exchange = emu
        .maps
        .read_dword(emu.regs().get_esp() + 4)
        .expect("cannot read the api parameter");
    let _Comperand = emu
        .maps
        .read_dword(emu.regs().get_esp() + 8)
        .expect("cannot read the api parameter");

    log_red!(emu, "kernel32!InterlockedCompareExchange");

    emu.regs_mut().rax = constants::ERROR_SUCCESS;

    for _ in 0..3 {
        emu.stack_pop32(false);
    }
}
