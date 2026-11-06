
use crate::emu;
use crate::winapi::helper;
pub fn EnableProcessOptionalXStateFeatures(emu: &mut emu::Emu) {
    let features = emu.regs().rcx;
    log_red!(
        emu,
        "kernel32!EnableProcessOptionalXStateFeatures features:0x{:x}",
        features
    );
    let handle = helper::handler_create("EnableProcessOptionalXStateFeatures");
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
