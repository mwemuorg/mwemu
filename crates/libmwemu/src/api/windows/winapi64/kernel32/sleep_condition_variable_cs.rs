use crate::emu;

pub fn SleepConditionVariableCS(emu: &mut emu::Emu) {
    let cv = emu.regs().rcx;
    let cs = emu.regs().rdx;
    let ms = emu.regs().r8;
    log_red!(
        emu,
        "kernel32!SleepConditionVariableCS cv:0x{:x} cs:0x{:x} ms:{}",
        cv,
        cs,
        ms
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
