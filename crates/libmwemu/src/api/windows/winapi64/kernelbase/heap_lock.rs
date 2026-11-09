use crate::emu;
use crate::winapi::helper;

pub fn HeapLock(emu: &mut emu::Emu) {
    let hHeap = emu.regs().rcx;
    log_red!(emu, "kernelbase!HeapLock 0x{:x}", hHeap);
    if helper::handler_exist(hHeap) {
        emu.set_last_error(0);
        emu.regs_mut().rax = 1;
    } else {
        emu.set_last_error(6);
        emu.regs_mut().rax = 0;
    }
}
