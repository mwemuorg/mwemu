use crate::emu;
use crate::winapi::helper::WinApiAbi;
use crate::windows::constants::errors::*;

pub fn TimeInit(emu: &mut emu::Emu) {
    let _InitOnce = WinApiAbi::arg(emu, 0) as u32;
    let _Parameter = WinApiAbi::arg(emu, 1) as u32;
    let _Context = WinApiAbi::arg(emu, 2) as u32;

    log_red!(emu, "kernel32!TimeInit");

    emu.regs_mut().rax = ERROR_SUCCESS;

    for _ in 0..3 {
        emu.stack_pop32(false);
    }
}
