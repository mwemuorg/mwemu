use crate::emu;
use crate::winapi::helper;

pub fn BeginUpdateResourceA(emu: &mut emu::Emu) {
    let pFileName = emu.regs().rcx;
    let bDeleteExistingResources = emu.regs().rdx;

    let filename = emu.maps.read_string(pFileName);

    log_red!(
        emu,
        "kernel32!BeginUpdateResourceA `{}` {}",
        filename,
        bDeleteExistingResources
    );

    emu.regs_mut().rax = helper::handler_create(&filename);
}
