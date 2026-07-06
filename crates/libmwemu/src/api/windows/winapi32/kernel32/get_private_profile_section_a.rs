use crate::emu;
use crate::windows::constants;

pub fn GetPrivateProfileSectionA(emu: &mut emu::Emu) {
    let lpAppName = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("cannot read the api parameter");
    let lpReturnedString = emu
        .maps
        .read_dword(emu.regs().get_esp() + 4)
        .expect("cannot read the api parameter");
    let _nSize = emu
        .maps
        .read_dword(emu.regs().get_esp() + 8)
        .expect("cannot read the api parameter");
    let lpFileName = emu
        .maps
        .read_dword(emu.regs().get_esp() + 12)
        .expect("cannot read the api parameter");

    let lpAppName = emu.maps.read_string(lpAppName as u64);
    let lpReturnedString = emu.maps.read_string(lpReturnedString as u64);
    let lpFileName = emu.maps.read_string(lpFileName as u64);

    log_red!(
        emu,
        "kernel32!GetPrivateProfileSectionA {} {} {}",
        lpAppName,
        lpReturnedString,
        lpFileName
    );

    emu.regs_mut().rax = constants::ERROR_SUCCESS;

    for _ in 0..4 {
        emu.stack_pop32(false);
    }
}
