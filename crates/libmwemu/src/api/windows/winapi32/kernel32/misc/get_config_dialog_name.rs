use crate::emu;
use crate::winapi::helper::WinApiAbi;
use crate::windows::constants::errors::*;

pub fn GetConfigDialogName(emu: &mut emu::Emu) {
    let _int = WinApiAbi::arg(emu, 0) as u32;
    let _int = WinApiAbi::arg(emu, 1) as u32;
    let _Src = WinApiAbi::arg(emu, 2) as u32;
    let _Size = WinApiAbi::arg(emu, 3) as u32;
    let _DestinationString = WinApiAbi::arg(emu, 4) as u32;
    let _int = WinApiAbi::arg(emu, 5) as u32;

    log_red!(emu, "kernel32!GetConfigDialogName");

    emu.regs_mut().rax = ERROR_SUCCESS;

    for _ in 0..6 {
        emu.stack_pop32(false);
    }
}
