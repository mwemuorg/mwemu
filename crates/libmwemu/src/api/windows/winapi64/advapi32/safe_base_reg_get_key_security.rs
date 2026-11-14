use crate::emu;

pub fn SafeBaseRegGetKeySecurity(emu: &mut emu::Emu) {
    let handle = emu.regs().rcx;
    let arg2 = emu.regs().rdx;
    log_red!(
        emu,
        "advapi32!SafeBaseRegGetKeySecurity h:0x{:x} arg2:0x{:x}",
        handle,
        arg2
    );
    for _ in 0..2 {}
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
