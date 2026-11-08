use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::{arg, stdcall_ret};
pub fn EnableProcessOptionalXStateFeatures(emu: &mut emu::Emu) {
    let features = arg(emu, 0) as u64;
    log_red!(
        emu,
        "kernel32!EnableProcessOptionalXStateFeatures features:0x{:x}",
        features
    );
    let handle = helper::handler_create("EnableProcessOptionalXStateFeatures");
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 1);
}
