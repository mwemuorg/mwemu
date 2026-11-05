use crate::emu;

pub fn GlobalFlags(emu: &mut emu::Emu) {
    let h_mem = emu.regs().rcx;
    log_red!(emu, "kernel32!GlobalFlags hMem:0x{:x}", h_mem);
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}
