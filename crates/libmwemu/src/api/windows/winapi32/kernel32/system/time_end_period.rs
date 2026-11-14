use crate::emu;
use crate::winapi::helper::WinApiAbi;
use crate::windows::constants::errors::*;

pub fn timeEndPeriod(emu: &mut emu::Emu) {
    let _uPeriod = WinApiAbi::arg(emu, 0) as u32;

    log_red!(emu, "kernel32!timeEndPeriod");

    emu.regs_mut().rax = ERROR_SUCCESS;

    for _ in 0..1 {
        emu.stack_pop32(false);
    }
}
