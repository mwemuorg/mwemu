use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn FreeLibrary(emu: &mut emu::Emu) {
    let hmod = WinApiAbi::arg(emu, 0);

    log_red!(emu, "kernel32!FreeLibrary   {:x}", hmod);

    emu.regs_mut().rax = 1;
    emu.stack_pop32(false);
}
