use crate::emu;
use crate::winapi::helper;

pub fn StgGetIFillLockBytesOnFile(emu: &mut emu::Emu) {
    let arg1 = emu.regs().rcx;
    let arg2 = emu.regs().rdx;
    log_red!(emu, "ole32!StgGetIFillLockBytesOnFile 0x{:x}", arg1);
    let handle = helper::handler_create("StgGetIFillLockBytesOnFile");
    let out_ptr = emu.regs().rdx;
    if out_ptr != 0 {
        emu.maps.write_qword(out_ptr, handle);
    }
    helper::advance_tick(emu, 1);
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}
