use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn TlsFree(emu: &mut emu::Emu) {
    let idx = WinApiAbi::arg(emu, 0) as u32;

    log_red!(emu, "kernel32!TlsFree idx: {}", idx);

    emu.stack_pop32(false);
    emu.regs_mut().set_eax(1);
}
