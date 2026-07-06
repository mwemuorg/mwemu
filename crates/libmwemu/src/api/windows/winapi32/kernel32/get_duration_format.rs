use crate::emu;
use crate::windows::constants;

pub fn GetDurationFormat(emu: &mut emu::Emu) {
    let _Locale = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("cannot read the api parameter");
    let _dwFlags = emu
        .maps
        .read_dword(emu.regs().get_esp() + 4)
        .expect("cannot read the api parameter");
    let _lpDuration = emu
        .maps
        .read_dword(emu.regs().get_esp() + 8)
        .expect("cannot read the api parameter");
    let _ullDuration = emu
        .maps
        .read_dword(emu.regs().get_esp() + 12)
        .expect("cannot read the api parameter");
    let _lpFormat = emu
        .maps
        .read_dword(emu.regs().get_esp() + 16)
        .expect("cannot read the api parameter");
    let _lpDurationStr = emu
        .maps
        .read_dword(emu.regs().get_esp() + 20)
        .expect("cannot read the api parameter");
    let _cchDuration = emu
        .maps
        .read_dword(emu.regs().get_esp() + 24)
        .expect("cannot read the api parameter");

    log_red!(emu, "kernel32!GetDurationFormat");

    emu.regs_mut().rax = constants::ERROR_SUCCESS;

    for _ in 0..7 {
        emu.stack_pop32(false);
    }
}
