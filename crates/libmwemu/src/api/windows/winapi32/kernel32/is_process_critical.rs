use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::{arg, stdcall_ret};
pub fn IsProcessCritical(emu: &mut emu::Emu) {
    let handle = arg(emu, 0) as u64;
    let param2 = arg(emu, 1) as u64;
    let param3 = arg(emu, 2) as u64;
    let param4 = arg(emu, 3) as u64;
    log_red!(
        emu,
        "kernel32!IsProcessCritical handle:0x{:x} p2:0x{:x}",
        handle,
        param2
    );
    emu.set_last_error(0);
    let h = arg(emu, 0) as u64;
    if helper::handler_exist(h) {
        emu.regs_mut().rax = 1;
    } else {
        emu.regs_mut().rax = 0;
    }
    stdcall_ret(emu, 4);
}
