use crate::emu;
use crate::windows::constants;

pub fn CheckNameLegalDOS8Dot3A(emu: &mut emu::Emu) {
    let lpName = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("cannot read the api parameter");
    let lpOemName = emu
        .maps
        .read_dword(emu.regs().get_esp() + 4)
        .expect("cannot read the api parameter");
    let OemNameSize = emu
        .maps
        .read_dword(emu.regs().get_esp() + 8)
        .expect("cannot read the api parameter");
    let pbNameContainsSpaces = emu
        .maps
        .read_dword(emu.regs().get_esp() + 12)
        .expect("cannot read the api parameter");
    let pbNameLegal = emu
        .maps
        .read_dword(emu.regs().get_esp() + 16)
        .expect("cannot read the api parameter");

    let lpName = emu.maps.read_string(lpName as u64);
    let lpOemName = emu.maps.read_string(lpOemName as u64);
    let OemNameSize = emu.maps.read_string(OemNameSize as u64);
    let pbNameContainsSpaces = emu.maps.read_string(pbNameContainsSpaces as u64);
    let pbNameLegal = emu.maps.read_string(pbNameLegal as u64);

    log_red!(
        emu,
        "kernel32!CheckNameLegalDOS8Dot3A {} {} {} {} {}",
        lpName,
        lpOemName,
        OemNameSize,
        pbNameContainsSpaces,
        pbNameLegal
    );

    emu.regs_mut().rax = constants::ERROR_SUCCESS;

    for _ in 0..5 {
        emu.stack_pop32(false);
    }
}
