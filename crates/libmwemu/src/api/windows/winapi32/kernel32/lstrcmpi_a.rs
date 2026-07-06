use crate::emu;
use crate::windows::constants;

pub fn lstrcmpiA(emu: &mut emu::Emu) {
    let lpString1 = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("cannot read the api parameter");
    let lpString2 = emu
        .maps
        .read_dword(emu.regs().get_esp() + 4)
        .expect("cannot read the api parameter");

    let lpString1 = emu.maps.read_string(lpString1 as u64);
    let lpString2 = emu.maps.read_string(lpString2 as u64);

    log_red!(emu, "kernel32!lstrcmpiA {} {}", lpString1, lpString2);

    emu.regs_mut().rax = constants::ERROR_SUCCESS;

    for _ in 0..2 {
        emu.stack_pop32(false);
    }
}
