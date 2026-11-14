use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub(super) fn _initterm(emu: &mut emu::Emu) {
    let ptr1 = WinApiAbi::arg(emu, 0) as u32;
    let ptr2 = WinApiAbi::arg(emu, 1) as u32;
    log_red!(emu, "kernelbase!_initterm 0x{:x} 0x{:x}", ptr1, ptr2);
    emu.regs_mut().rax = 0;
}
