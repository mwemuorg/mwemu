use crate::emu;

pub fn HeapLock(emu: &mut emu::Emu) {
    let hHeap = emu.regs().rcx;
    log_red!(emu, "kernelbase!HeapLock 0x{:x}", hHeap);
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
