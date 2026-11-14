use crate::emu;
use crate::winapi::helper::WinApiAbi;
use crate::windows::constants::errors::*;

pub fn InterlockedCompareExchange(emu: &mut emu::Emu) {
    let _Destination = WinApiAbi::arg(emu, 0) as u32;
    let _Exchange = WinApiAbi::arg(emu, 1) as u32;
    let _Comperand = WinApiAbi::arg(emu, 2) as u32;

    log_red!(emu, "kernel32!InterlockedCompareExchange");

    emu.regs_mut().rax = ERROR_SUCCESS;

    for _ in 0..3 {
        emu.stack_pop32(false);
    }
}
