use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::{arg, stdcall_ret};
pub fn IsIoRingOpSupported(emu: &mut emu::Emu) {
    let io_ring = arg(emu, 0) as u64;
    let op = arg(emu, 1) as u64;
    log_red!(
        emu,
        "kernel32!IsIoRingOpSupported ring:0x{:x} op:{} => FALSE",
        io_ring,
        op
    );
    emu.set_last_error(0);
    let h = arg(emu, 0) as u64;
    if helper::handler_exist(h) {
        emu.regs_mut().rax = 1;
    } else {
        emu.regs_mut().rax = 0;
    }
    stdcall_ret(emu, 2);
}
