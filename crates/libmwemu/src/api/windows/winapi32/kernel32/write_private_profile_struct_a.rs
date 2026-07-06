use crate::emu;
use crate::windows::constants;

pub fn WritePrivateProfileStructA(emu: &mut emu::Emu) {
    let _lpszSection = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("cannot read the api parameter");
    let _lpszKey = emu
        .maps
        .read_dword(emu.regs().get_esp() + 4)
        .expect("cannot read the api parameter");
    let lpStruct = emu
        .maps
        .read_dword(emu.regs().get_esp() + 8)
        .expect("cannot read the api parameter");
    let uSizeStruct = emu
        .maps
        .read_dword(emu.regs().get_esp() + 12)
        .expect("cannot read the api parameter");
    let _szFile = emu
        .maps
        .read_dword(emu.regs().get_esp() + 16)
        .expect("cannot read the api parameter");

    let lpStruct = emu.maps.read_string(lpStruct as u64);
    let uSizeStruct = emu.maps.read_string(uSizeStruct as u64);

    log_red!(
        emu,
        "kernel32!WritePrivateProfileStructA {} {}",
        lpStruct,
        uSizeStruct
    );

    emu.regs_mut().rax = constants::ERROR_SUCCESS;

    for _ in 0..5 {
        emu.stack_pop32(false);
    }
}
