use crate::emu;

pub fn BasepFreeActivationContextActivationBlock(emu: &mut emu::Emu) {
    let param1 = emu.regs().rcx;
    log_red!(
        emu,
        "kernel32!BasepFreeActivationContextActivationBlock param1:0x{:x}",
        param1
    );
    emu.set_last_error(0);
}
