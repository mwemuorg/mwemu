use crate::emu;
use crate::winapi::helper::WinApiAbi;
use crate::windows::constants::errors::*;

pub fn QueryActCtxWWorker(emu: &mut emu::Emu) {
    let _int = WinApiAbi::arg(emu, 0) as u32;
    let _Context = WinApiAbi::arg(emu, 1) as u32;
    let _pvSubInstance = WinApiAbi::arg(emu, 2) as u32;
    let _ulInfoClass = WinApiAbi::arg(emu, 3) as u32;
    let _pvBuffer = WinApiAbi::arg(emu, 4) as u32;
    let _cbBuffer = WinApiAbi::arg(emu, 5) as u32;
    let _pcbWrittenOrRequired = WinApiAbi::arg(emu, 6) as u32;

    log_red!(emu, "kernel32!QueryActCtxWWorker");

    emu.regs_mut().rax = ERROR_SUCCESS;

    for _ in 0..7 {
        emu.stack_pop32(false);
    }
}
