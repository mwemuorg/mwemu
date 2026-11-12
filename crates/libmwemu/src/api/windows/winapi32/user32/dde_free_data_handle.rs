use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::arg;
use crate::winapi::winapi32::stdcall_ret;

pub fn DdeFreeDataHandle(emu: &mut emu::Emu) {
    let handle = arg(emu, 0) as u64;
    log_red!(emu, "user32!DdeFreeDataHandle handle:0x{:x}", handle);
    helper::handler_close(handle);
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 1);
}
