use crate::emu;
use crate::windows::constants;

pub fn StringVPrintfWorkerW(emu: &mut emu::Emu) {
    let _pszDest = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let _cchDest = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let _pcchNewDestLength = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
    let _pszFormat = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
    let _argList = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0);

    log_red!(emu, "kernel32!StringVPrintfWorkerW");

    emu.regs_mut().rax = constants::ERROR_SUCCESS;

    for _ in 0..5 {
        emu.stack_pop32(false);
    }
}
