use crate::emu;
use crate::winapi::helper;

pub fn GetObjectType(emu: &mut emu::Emu) {
    let obj = emu.regs().rcx;
    let result: u64 = if helper::handler_exist(obj) { 6 } else { 0 }; // OBJ_BITMAP=6 as generic
    log_red!(emu, "gdi32!GetObjectType obj:0x{:x} = {}", obj, result);
    emu.set_last_error(0);
    emu.regs_mut().rax = result;
}
