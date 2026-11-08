use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::{arg, stdcall_ret};
pub fn FreeUserPhysicalPages(emu: &mut emu::Emu) {
    let process = arg(emu, 0) as u64;
    let base_address = arg(emu, 1) as u64;
    let page_array = arg(emu, 2) as u64;
    log_red!(
        emu,
        "kernel32!FreeUserPhysicalPages proc:0x{:x} addr:0x{:x}",
        process,
        base_address
    );
    let h = arg(emu, 0) as u64;
    if helper::handler_exist(h) {
        helper::handler_close(h);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1; // TRUE
    stdcall_ret(emu, 3);
}
