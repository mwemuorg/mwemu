use crate::emu;
use crate::winapi::helper::WinApiAbi;
use crate::windows::constants::errors::*;

pub fn RaiseInvalid16BitExeError(emu: &mut emu::Emu) {
    let _a1 = WinApiAbi::arg(emu, 0) as u32;

    log_red!(emu, "kernel32!RaiseInvalid16BitExeError");

    emu.regs_mut().rax = ERROR_SUCCESS;

    for _ in 0..1 {
        emu.stack_pop32(false);
    }
}
