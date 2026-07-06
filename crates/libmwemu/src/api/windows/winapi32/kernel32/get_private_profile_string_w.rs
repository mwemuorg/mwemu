use crate::emu;
use crate::windows::constants;

pub fn GetPrivateProfileStringW(emu: &mut emu::Emu) {
    let lpAppName = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("cannot read the api parameter");
    let lpKeyName = emu
        .maps
        .read_dword(emu.regs().get_esp() + 4)
        .expect("cannot read the api parameter");
    let _lpDefault = emu
        .maps
        .read_dword(emu.regs().get_esp() + 8)
        .expect("cannot read the api parameter");
    let lpReturnedString = emu
        .maps
        .read_dword(emu.regs().get_esp() + 12)
        .expect("cannot read the api parameter");
    let _nSize = emu
        .maps
        .read_dword(emu.regs().get_esp() + 16)
        .expect("cannot read the api parameter");
    let lpFileName = emu
        .maps
        .read_dword(emu.regs().get_esp() + 20)
        .expect("cannot read the api parameter");

    let lpAppName = emu.maps.read_wide_string(lpAppName as u64);
    let lpKeyName = emu.maps.read_wide_string(lpKeyName as u64);
    let lpReturnedString = emu.maps.read_wide_string(lpReturnedString as u64);
    let lpFileName = emu.maps.read_wide_string(lpFileName as u64);

    log_red!(
        emu,
        "kernel32!GetPrivateProfileStringW {} {} {} {}",
        lpAppName,
        lpKeyName,
        lpReturnedString,
        lpFileName
    );

    emu.regs_mut().rax = constants::ERROR_SUCCESS;

    for _ in 0..6 {
        emu.stack_pop32(false);
    }
}
