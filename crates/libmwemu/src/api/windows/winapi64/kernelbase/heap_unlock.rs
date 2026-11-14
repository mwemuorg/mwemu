use crate::emu;

pub fn HeapUnlock(emu: &mut emu::Emu) {
    let hHeap = emu.regs().rcx;
    log_red!(emu, "kernelbase!HeapUnlock 0x{:x}", hHeap);
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
