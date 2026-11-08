use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::{arg, stdcall_ret};
pub fn HeapValidate(emu: &mut emu::Emu) {
    let p0 = arg(emu, 0) as u64;
    let p1 = arg(emu, 1) as u64;
    let p2 = arg(emu, 2) as u64;
    log_red!(emu, "kernel32!HeapValidate p1:0x{:x} p2:0x{:x}", p1, p2);
    emu.set_last_error(0);
    let h = arg(emu, 0) as u64;
    if helper::handler_exist(h) {
        emu.regs_mut().rax = 1;
    } else {
        emu.regs_mut().rax = 0;
    }
    stdcall_ret(emu, 3);
}
