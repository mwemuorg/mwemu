use crate::emu;
use crate::winapi::helper::WinApiAbi;
use crate::windows::constants::errors::*;

pub fn BackupSeek(emu: &mut emu::Emu) {
    let _hFile = WinApiAbi::arg(emu, 0) as u32;
    let _dwLowBytesToSeek = WinApiAbi::arg(emu, 1) as u32;
    let _dwHighBytesToSeek = WinApiAbi::arg(emu, 2) as u32;
    let _lpdwLowByteSeeked = WinApiAbi::arg(emu, 3) as u32;
    let _lpdwHighByteSeeked = WinApiAbi::arg(emu, 4) as u32;
    let _lpContext = WinApiAbi::arg(emu, 5) as u32;

    log_red!(emu, "kernel32!BackupSeek");

    emu.regs_mut().rax = ERROR_SUCCESS;

    for _ in 0..6 {
        emu.stack_pop32(false);
    }
}
