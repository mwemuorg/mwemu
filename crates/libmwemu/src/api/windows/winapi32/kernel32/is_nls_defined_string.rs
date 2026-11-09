use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};
pub fn IsNLSDefinedString(emu: &mut emu::Emu) {
    let func = arg(emu, 0) as u64;
    let flags = arg(emu, 1) as u64;
    let lp_version_info = arg(emu, 2) as u64;
    let lp_string = arg(emu, 3) as u64;
    log_red!(
        emu,
        "kernel32!IsNLSDefinedString func:{} flags:0x{:x} verInfo:0x{:x} str:0x{:x}",
        func,
        flags,
        lp_version_info,
        lp_string
    );
    emu.set_last_error(0);
    let h = arg(emu, 0) as u64;
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 4);
}
