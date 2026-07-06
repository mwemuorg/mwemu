use crate::emu;
use crate::windows::constants;

pub fn ReplaceFileA(emu: &mut emu::Emu) {
    let lpReplacedFileName = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("cannot read the api parameter");
    let lpReplacementFileName = emu
        .maps
        .read_dword(emu.regs().get_esp() + 4)
        .expect("cannot read the api parameter");
    let lpBackupFileName = emu
        .maps
        .read_dword(emu.regs().get_esp() + 8)
        .expect("cannot read the api parameter");
    let _dwReplaceFlags = emu
        .maps
        .read_dword(emu.regs().get_esp() + 12)
        .expect("cannot read the api parameter");
    let _lpExclude = emu
        .maps
        .read_dword(emu.regs().get_esp() + 16)
        .expect("cannot read the api parameter");
    let _lpReserved = emu
        .maps
        .read_dword(emu.regs().get_esp() + 20)
        .expect("cannot read the api parameter");

    let lpReplacedFileName = emu.maps.read_string(lpReplacedFileName as u64);
    let lpReplacementFileName = emu.maps.read_string(lpReplacementFileName as u64);
    let lpBackupFileName = emu.maps.read_string(lpBackupFileName as u64);

    log_red!(
        emu,
        "kernel32!ReplaceFileA {} {} {}",
        lpReplacedFileName,
        lpReplacementFileName,
        lpBackupFileName
    );

    emu.regs_mut().rax = constants::ERROR_SUCCESS;

    for _ in 0..6 {
        emu.stack_pop32(false);
    }
}
