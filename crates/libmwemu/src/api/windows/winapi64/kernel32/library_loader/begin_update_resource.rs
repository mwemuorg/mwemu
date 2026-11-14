use crate::emu;
use crate::emu::Emu;
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

pub fn BeginUpdateResourceW(emu: &mut Emu) {
    let filename = emu.maps.read_wide_string(emu.regs().rcx);
    let delete_existing = emu.regs().rdx != 0;
    log_red!(
        emu,
        "kernel32!BeginUpdateResourceW '{}' delete_existing:{}",
        filename,
        delete_existing
    );
    emu.regs_mut().rax = helper::handler_create(&filename);
}
