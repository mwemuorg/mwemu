use crate::emu;

pub fn IsValidCodePage(emu: &mut emu::Emu) {
    let cp = emu.regs().rcx;
    log_red!(emu, "kernelbase!IsValidCodePage cp:{}", cp);
    emu.set_last_error(0);
    let h = emu.regs().rcx;
    emu.regs_mut().rax = 1;
}
