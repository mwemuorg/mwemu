use crate::emu;

pub fn GlobalHandle(emu: &mut emu::Emu) {
    let p_mem = emu.regs().rcx;
    log_red!(emu, "kernel32!GlobalHandle pMem:0x{:x}", p_mem);
    emu.set_last_error(0);
    emu.regs_mut().rax = p_mem;
}
