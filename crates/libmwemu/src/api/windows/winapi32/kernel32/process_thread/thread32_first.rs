use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn Thread32First(emu: &mut emu::Emu) {
    let hndl = WinApiAbi::arg(emu, 0) as u32;
    let entry = WinApiAbi::arg(emu, 1) as u32;

    log_red!(emu, "kernel32!Thread32First");

    emu.stack_pop32(false);
    emu.stack_pop32(false);

    emu.regs_mut().rax = 1;
}
