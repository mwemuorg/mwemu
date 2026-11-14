use crate::emu;
use crate::winapi::helper::WinApiAbi;
use crate::windows::constants::errors::*;

pub fn InterlockedExchangeAdd(emu: &mut emu::Emu) {
    let _Addend = WinApiAbi::arg(emu, 0) as u32;
    let _Value = WinApiAbi::arg(emu, 1) as u32;

    log_red!(emu, "kernel32!InterlockedExchangeAdd");

    emu.regs_mut().rax = ERROR_SUCCESS;

    for _ in 0..2 {
        emu.stack_pop32(false);
    }
}
