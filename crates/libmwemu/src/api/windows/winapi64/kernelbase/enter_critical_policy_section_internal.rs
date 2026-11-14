use crate::emu;
use crate::winapi::helper;

pub fn EnterCriticalPolicySectionInternal(emu: &mut emu::Emu) {
    let arg1 = emu.regs().rcx;
    let arg2 = emu.regs().rdx;
    log_red!(
        emu,
        "kernelbase!EnterCriticalPolicySectionInternal arg1:0x{:x} arg2:0x{:x}",
        arg1,
        arg2
    );

    helper::advance_tick(emu, 1);
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
