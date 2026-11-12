use crate::emu;
use crate::winapi::helper::WinApiAbi;
use crate::windows::constants::errors::*;

pub fn _NLG_Notify(emu: &mut emu::Emu) {
    let __DWORD = WinApiAbi::arg(emu, 0) as u32;

    log_red!(emu, "kernel32!_NLG_Notify");

    emu.regs_mut().rax = ERROR_SUCCESS;

    for _ in 0..1 {
        emu.stack_pop32(false);
    }
}
