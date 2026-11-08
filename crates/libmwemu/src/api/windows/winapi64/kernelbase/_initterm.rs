use crate::emu;

pub(super) fn _initterm(emu: &mut emu::Emu) {
    let arg1 = emu.regs().rcx;
    let arg2 = emu.regs().rdx;
    log_red!(
        emu,
        "kernelbase!_initterm arg1:0x{:x} arg2:0x{:x}",
        arg1,
        arg2
    );
    emu.regs_mut().rax = 0;
}
