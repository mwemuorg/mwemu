use crate::emu;
use crate::windows::constants;

pub fn GetShortPathNameA(emu: &mut emu::Emu) {
    let lpszLongPath = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("cannot read the api parameter");
    let lpszShortPath = emu
        .maps
        .read_dword(emu.regs().get_esp() + 4)
        .expect("cannot read the api parameter");
    let _cchBuffer = emu
        .maps
        .read_dword(emu.regs().get_esp() + 8)
        .expect("cannot read the api parameter");

    let lpszLongPath = emu.maps.read_string(lpszLongPath as u64);
    let lpszShortPath = emu.maps.read_string(lpszShortPath as u64);

    log_red!(
        emu,
        "kernel32!GetShortPathNameA {} {}",
        lpszLongPath,
        lpszShortPath
    );

    emu.regs_mut().rax = constants::ERROR_SUCCESS;

    for _ in 0..3 {
        emu.stack_pop32(false);
    }
}
