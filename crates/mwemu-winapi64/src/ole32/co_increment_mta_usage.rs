use crate::emu;
use crate::winapi::helper;

pub fn CoIncrementMTAUsage(emu: &mut emu::Emu) {
    let cookie_ptr = emu.regs().rcx;
    log_red!(
        emu,
        "ole32!CoIncrementMTAUsage cookie_ptr:0x{:x}",
        cookie_ptr
    );
    let handle = helper::handler_create("CoMTAUsage");
    if cookie_ptr != 0 {
        emu.maps.write_qword(cookie_ptr, handle);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}
