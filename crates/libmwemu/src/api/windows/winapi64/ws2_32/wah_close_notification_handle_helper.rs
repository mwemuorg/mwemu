use crate::emu;
use crate::winapi::helper;

pub fn WahCloseNotificationHandleHelper(emu: &mut emu::Emu) {
    let handle = emu.regs().rcx;
    log_red!(
        emu,
        "ws2_32!WahCloseNotificationHandleHelper handle:0x{:x}",
        handle
    );
    helper::handler_close(handle);
    emu.regs_mut().rax = 0;
}
