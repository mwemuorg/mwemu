use crate::api::windows::helper;
use crate::emu;

pub fn ConvertThreadToFiberEx(emu: &mut emu::Emu) {
    let param = emu.regs().rcx;

    log_red!(emu, "kernel32!ConvertThreadToFiberEx param:0x{:x}", param);

    let handle = helper::handler_create("fiber://");
    emu.regs_mut().rax = handle;
}
