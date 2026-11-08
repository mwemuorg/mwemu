use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::{arg, stdcall_ret};
pub fn IsNormalizedString(emu: &mut emu::Emu) {
    let norm_form = arg(emu, 0) as u64;
    let lp_string = arg(emu, 1) as u64;
    let cw_length = arg(emu, 2) as u64 as i64;
    log_red!(
        emu,
        "kernel32!IsNormalizedString form:{} str:0x{:x} len:{}",
        norm_form,
        lp_string,
        cw_length
    );
    emu.set_last_error(0);
    let h = arg(emu, 0) as u64;
    if helper::handler_exist(h) {
        emu.regs_mut().rax = 1;
    } else {
        emu.regs_mut().rax = 0;
    }
    stdcall_ret(emu, 3);
}
