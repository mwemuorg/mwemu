use crate::emu;
use crate::windows::constants;

pub fn ReplaceFileA(emu: &mut emu::Emu) {
    let lpReplacedFileName = emu.regs().rcx as u32;
    let lpReplacementFileName = emu.regs().rdx as u32;
    let lpBackupFileName = emu.regs().r8 as u32;
    let _dwReplaceFlags = emu.regs().r9 as u32;
    let _lpExclude = emu.maps.read_qword(emu.regs().rsp + 0x20).unwrap_or(0);
    let _lpReserved = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);

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

    emu.set_last_error(0);
    emu.regs_mut().rax = constants::ERROR_SUCCESS;
}
