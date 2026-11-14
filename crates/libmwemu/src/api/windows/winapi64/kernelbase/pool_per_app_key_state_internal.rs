use crate::emu;

pub fn PoolPerAppKeyStateInternal(emu: &mut emu::Emu) {
    let handle = emu.regs().rcx;
    let arg2 = emu.regs().rdx;
    log_red!(
        emu,
        "kernelbase!PoolPerAppKeyStateInternal h:0x{:x} arg2:0x{:x}",
        handle,
        arg2
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
