use crate::emu;

pub fn VerifyVersionInfoW(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!VerifyVersionInfoW");

    emu.stack_pop32(false);
    emu.stack_pop32(false);
    emu.stack_pop32(false);
    emu.regs_mut().rax = 0xffff;
}
