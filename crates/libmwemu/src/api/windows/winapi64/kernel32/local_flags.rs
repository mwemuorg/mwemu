use crate::emu;

pub fn LocalFlags(emu: &mut emu::Emu) {
    let h_mem = emu.regs().rcx;
    log_red!(emu, "kernel32!LocalFlags hMem:0x{:x}", h_mem);
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}
