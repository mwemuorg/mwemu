use crate::emu;

pub fn SleepConditionVariableSRW(emu: &mut emu::Emu) {
    let cv = emu.regs().rcx;
    let lock = emu.regs().rdx;
    let ms = emu.regs().r8;
    let flags = emu.regs().r9;
    log_red!(
        emu,
        "kernel32!SleepConditionVariableSRW cv:0x{:x} lock:0x{:x} ms:{} flags:{}",
        cv,
        lock,
        ms,
        flags
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
