use crate::emu;

pub fn IsValidCodePage_api(emu: &mut emu::Emu) {
    let cp = emu.regs().rcx;
    log_red!(emu, "kernel32!IsValidCodePage cp: {}", cp);
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
