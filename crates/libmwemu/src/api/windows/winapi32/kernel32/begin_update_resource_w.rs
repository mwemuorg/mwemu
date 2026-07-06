use crate::emu;
use crate::windows::constants;

pub fn BeginUpdateResourceW(emu: &mut emu::Emu) {
    let pFileName = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("cannot read the api parameter");
    let _bDeleteExistingResources = emu
        .maps
        .read_dword(emu.regs().get_esp() + 4)
        .expect("cannot read the api parameter");

    let pFileName = emu.maps.read_wide_string(pFileName as u64);

    log_red!(emu, "kernel32!BeginUpdateResourceW {}", pFileName);

    emu.regs_mut().rax = constants::ERROR_SUCCESS;

    for _ in 0..2 {
        emu.stack_pop32(false);
    }
}
