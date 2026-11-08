use crate::emu;
use crate::windows::constants;

pub fn BackupWrite(emu: &mut emu::Emu) {
    let _hFile = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let _lpBuffer = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let _nNumberOfBytesToWrite = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
    let _lpNumberOfBytesWritten = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
    let _bAbort = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0);
    let _bProcessSecurity = emu.maps.read_dword(emu.regs().get_esp() + 20).unwrap_or(0);
    let _lpContext = emu.maps.read_dword(emu.regs().get_esp() + 24).unwrap_or(0);

    log_red!(emu, "kernel32!BackupWrite");

    emu.regs_mut().rax = constants::ERROR_SUCCESS;

    for _ in 0..7 {
        emu.stack_pop32(false);
    }
}
