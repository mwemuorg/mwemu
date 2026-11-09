use crate::emu;
pub fn BasepIsProcessAllowed(emu: &mut emu::Emu) {
    let param1 = emu.regs().rcx;
    log_red!(emu, "kernel32!BasepIsProcessAllowed param1:0x{:x}", param1);
    emu.set_last_error(0);
    let h = emu.regs().rcx;
    emu.regs_mut().rax = 1;
}
