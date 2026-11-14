use crate::emu;
use crate::winapi::helper::WinApiAbi;
use crate::windows::constants::errors::*;

pub fn BasepPostSuccessAppXExtension(emu: &mut emu::Emu) {
    let _a1 = WinApiAbi::arg(emu, 0) as u32;
    let _a2 = WinApiAbi::arg(emu, 1) as u32;

    log_red!(emu, "kernel32!BasepPostSuccessAppXExtension");

    emu.regs_mut().rax = ERROR_SUCCESS;

    for _ in 0..2 {
        emu.stack_pop32(false);
    }
}
