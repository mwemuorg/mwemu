use crate::emu;
use crate::winapi::helper;

pub fn WahOpenNotificationHandleHelper(emu: &mut emu::Emu) {
    let helper_ptr = emu.regs().rcx;
    log_red!(
        emu,
        "ws2_32!WahOpenNotificationHandleHelper ptr:0x{:x}",
        helper_ptr
    );
    let handle = helper::handler_create("wah_handle");
    if helper_ptr != 0 {
        emu.maps.write_qword(helper_ptr, handle);
    }
    emu.regs_mut().rax = 0;
}
