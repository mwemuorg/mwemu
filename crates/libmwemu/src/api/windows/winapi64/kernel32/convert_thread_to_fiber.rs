use crate::emu;
use crate::winapi::helper;
pub fn ConvertThreadToFiber(emu: &mut emu::Emu) {
    let param = emu.regs().rcx;
    log_red!(emu, "kernel32!ConvertThreadToFiber param:0x{:x}", param);
    let handle = helper::handler_create("fiber://");
    emu.regs_mut().rax = handle;
}
