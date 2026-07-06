use crate::emu;
use crate::windows::constants;

pub fn FoldStringA(emu: &mut emu::Emu) {
    let _dwMapFlags = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("cannot read the api parameter");
    let lpSrcStr = emu
        .maps
        .read_dword(emu.regs().get_esp() + 4)
        .expect("cannot read the api parameter");
    let _cchSrc = emu
        .maps
        .read_dword(emu.regs().get_esp() + 8)
        .expect("cannot read the api parameter");
    let lpDestStr = emu
        .maps
        .read_dword(emu.regs().get_esp() + 12)
        .expect("cannot read the api parameter");
    let _cchDest = emu
        .maps
        .read_dword(emu.regs().get_esp() + 16)
        .expect("cannot read the api parameter");

    let lpSrcStr = emu.maps.read_string(lpSrcStr as u64);
    let lpDestStr = emu.maps.read_string(lpDestStr as u64);

    log_red!(emu, "kernel32!FoldStringA {} {}", lpSrcStr, lpDestStr);

    emu.regs_mut().rax = constants::ERROR_SUCCESS;

    for _ in 0..5 {
        emu.stack_pop32(false);
    }
}
