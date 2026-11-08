use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::{arg, stdcall_ret};
pub fn ConvertThreadToFiberEx(emu: &mut emu::Emu) {
    let param = arg(emu, 0) as u64;
    log_red!(emu, "kernel32!ConvertThreadToFiberEx param:0x{:x}", param);
    let handle = helper::handler_create("fiber://");
    emu.regs_mut().rax = handle;
    stdcall_ret(emu, 1);
}
