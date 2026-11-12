use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn FlsAlloc(emu: &mut emu::Emu) {
    let callback = WinApiAbi::arg(emu, 0) as u32;

    log_red!(emu, "kernel32!FlsAlloc callback: 0x{:x}", callback);

    emu.stack_pop32(false);
    emu.regs_mut().rax = 1;
}
