use crate::emu;
use crate::windows::constants;

pub fn FindActCtxSectionStringA(emu: &mut emu::Emu) {
    let _dwFlags = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("cannot read the api parameter");
    let _lpExtensionGuid = emu
        .maps
        .read_dword(emu.regs().get_esp() + 4)
        .expect("cannot read the api parameter");
    let _ulSectionId = emu
        .maps
        .read_dword(emu.regs().get_esp() + 8)
        .expect("cannot read the api parameter");
    let lpStringToFind = emu
        .maps
        .read_dword(emu.regs().get_esp() + 12)
        .expect("cannot read the api parameter");
    let _ReturnedData = emu
        .maps
        .read_dword(emu.regs().get_esp() + 16)
        .expect("cannot read the api parameter");

    let lpStringToFind = emu.maps.read_string(lpStringToFind as u64);

    log_red!(emu, "kernel32!FindActCtxSectionStringA {}", lpStringToFind);

    emu.regs_mut().rax = constants::ERROR_SUCCESS;

    for _ in 0..5 {
        emu.stack_pop32(false);
    }
}
