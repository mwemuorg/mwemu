use crate::emu;
use crate::winapi::helper::WinApiAbi;
use crate::windows::constants::errors::*;

pub fn BackupWrite(emu: &mut emu::Emu) {
    let _hFile = WinApiAbi::arg(emu, 0) as u32;
    let _lpBuffer = WinApiAbi::arg(emu, 1) as u32;
    let _nNumberOfBytesToWrite = WinApiAbi::arg(emu, 2) as u32;
    let _lpNumberOfBytesWritten = WinApiAbi::arg(emu, 3) as u32;
    let _bAbort = WinApiAbi::arg(emu, 4) as u32;
    let _bProcessSecurity = WinApiAbi::arg(emu, 5) as u32;
    let _lpContext = WinApiAbi::arg(emu, 6) as u32;

    log_red!(emu, "kernel32!BackupWrite");

    emu.regs_mut().rax = ERROR_SUCCESS;

    for _ in 0..7 {
        emu.stack_pop32(false);
    }
}
