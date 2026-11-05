use crate::emu;

pub fn GlobalFix(emu: &mut emu::Emu) {
    let h_mem = emu.regs().rcx;
    log_red!(emu, "kernel32!GlobalFix hMem:0x{:x}", h_mem);
    emu.set_last_error(0);
}
