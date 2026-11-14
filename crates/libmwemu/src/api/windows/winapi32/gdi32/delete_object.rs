use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::arg;
use crate::winapi::winapi32::stdcall_ret;

pub fn DeleteObject(emu: &mut emu::Emu) {
    let obj = arg(emu, 0) as u64;
    log_red!(emu, "gdi32!DeleteObject obj:0x{:x}", obj);
    if helper::handler_exist(obj) {
        helper::handler_close(obj);
    }
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 1);
}
