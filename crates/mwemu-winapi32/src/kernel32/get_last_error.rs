use crate::emu;

pub fn GetLastError(emu: &mut emu::Emu) {
    emu.regs_mut().rax = emu.get_last_error();
    log_red!(emu, "kernel32!GetLastError ={}", emu.regs().rax);
}
