use crate::emu;
use crate::winapi::helper::WinApiAbi;
use crate::windows::constants::errors::*;

pub fn timeGetDevCaps(emu: &mut emu::Emu) {
    let _ptc = WinApiAbi::arg(emu, 0) as u32;
    let _cbtc = WinApiAbi::arg(emu, 1) as u32;

    log_red!(emu, "kernel32!timeGetDevCaps");

    emu.regs_mut().rax = ERROR_SUCCESS;

    for _ in 0..2 {
        emu.stack_pop32(false);
    }
}
