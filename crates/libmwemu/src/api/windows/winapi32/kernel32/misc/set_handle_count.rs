use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn SetHandleCount(emu: &mut emu::Emu) {
    let num = WinApiAbi::arg(emu, 0) as u32;

    log_red!(emu, "kernel32!SetHandleCount {}", num);

    emu.stack_pop32(false);
    emu.regs_mut().rax = num as u64;
}
