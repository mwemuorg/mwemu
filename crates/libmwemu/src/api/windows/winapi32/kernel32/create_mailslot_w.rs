use crate::emu;
use crate::windows::constants;

pub fn CreateMailslotW(emu: &mut emu::Emu) {
    let lpName = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("cannot read the api parameter");
    let _nMaxMessageSize = emu
        .maps
        .read_dword(emu.regs().get_esp() + 4)
        .expect("cannot read the api parameter");
    let _lReadTimeout = emu
        .maps
        .read_dword(emu.regs().get_esp() + 8)
        .expect("cannot read the api parameter");
    let _lpSecurityAttributes = emu
        .maps
        .read_dword(emu.regs().get_esp() + 12)
        .expect("cannot read the api parameter");

    let lpName = emu.maps.read_wide_string(lpName as u64);

    log_red!(emu, "kernel32!CreateMailslotW {}", lpName);

    emu.regs_mut().rax = constants::ERROR_SUCCESS;

    for _ in 0..4 {
        emu.stack_pop32(false);
    }
}
