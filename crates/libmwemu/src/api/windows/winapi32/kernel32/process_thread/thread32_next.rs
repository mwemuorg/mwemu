use crate::emu;
use crate::winapi::helper::WinApiAbi;
use crate::windows::constants::errors::*;

pub fn Thread32Next(emu: &mut emu::Emu) {
    let hndl = WinApiAbi::arg(emu, 0) as u32;
    let entry = WinApiAbi::arg(emu, 1) as u32;

    log_red!(emu, "kernel32!Thread32Next");

    emu.stack_pop32(false);
    emu.stack_pop32(false);

    emu.regs_mut().rax = ERROR_NO_MORE_FILES;
}
