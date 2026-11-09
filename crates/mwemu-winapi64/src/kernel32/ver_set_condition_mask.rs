use crate::emu;

pub fn VerSetConditionMask(emu: &mut emu::Emu) {
    let mask = emu.regs().rcx;
    let type_bit = emu.regs().rdx;
    let condition = emu.regs().r8;
    log_red!(
        emu,
        "kernel32!VerSetConditionMask mask: 0x{:x} type: {} cond: {}",
        mask,
        type_bit,
        condition
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = mask | (condition << (type_bit & 0x1f));
}
