use crate::emu;
use crate::windows::constants;

pub fn ReplaceFileA(emu: &mut emu::Emu) {
    let lpReplacedFileName = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let lpReplacementFileName = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let lpBackupFileName = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
    let _dwReplaceFlags = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
    let _lpExclude = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0);
    let _lpReserved = emu.maps.read_dword(emu.regs().get_esp() + 20).unwrap_or(0);

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
