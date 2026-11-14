use crate::emu;
use crate::winapi::helper::WinApiAbi;
use crate::windows::constants::process::*;

pub fn TerminateProcess(emu: &mut emu::Emu) {
    let hndl = WinApiAbi::arg(emu, 0) as u32;
    let code = WinApiAbi::arg(emu, 1) as u32;

    log_red!(
        emu,
        "kernel32!TerminateProcess hndl: {} code: {}",
        hndl,
        code
    );

    emu.stack_pop32(false);
    emu.stack_pop32(false);

    if hndl == CURRENT_PROCESS_HANDLE as u32 {
        emu.stop();
        return;
    }
    emu.regs_mut().rax = 1;
}
