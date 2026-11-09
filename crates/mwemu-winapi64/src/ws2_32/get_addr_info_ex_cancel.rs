use crate::emu;
use crate::winapi::helper;

pub fn GetAddrInfoExCancel(emu: &mut emu::Emu) {
    let cancel_ptr = emu.regs().rcx;
    log_red!(emu, "ws2_32!GetAddrInfoExCancel ptr:0x{:x}", cancel_ptr);
    let handle = helper::handler_create("GetAddrInfoExCancel");
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}
