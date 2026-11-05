use crate::emu;

pub fn GlobalUnfix(emu: &mut emu::Emu) {
    let h_mem = emu.regs().rcx;
    log_red!(emu, "kernel32!GlobalUnfix hMem:0x{:x}", h_mem);
    emu.set_last_error(0);
}
