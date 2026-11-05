use crate::emu;

pub fn EnableProcessOptionalXStateFeatures(emu: &mut emu::Emu) {
    let features = emu.regs().rcx;
    log_red!(
        emu,
        "kernel32!EnableProcessOptionalXStateFeatures features:0x{:x}",
        features
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
