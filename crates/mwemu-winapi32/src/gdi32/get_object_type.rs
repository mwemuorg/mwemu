use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::arg;
use crate::winapi::winapi32::stdcall_ret;

pub fn GetObjectType(emu: &mut emu::Emu) {
    let obj = arg(emu, 0) as u64;
    log_red!(emu, "gdi32!GetObjectType obj:0x{:x}", obj);
    if helper::handler_exist(obj) {
        helper::advance_tick(emu, 1);
        emu.set_last_error(0);
        emu.regs_mut().rax = 6; // OBJ_BITMAP
    } else {
        emu.regs_mut().rax = 0;
    }
    stdcall_ret(emu, 1);
}
