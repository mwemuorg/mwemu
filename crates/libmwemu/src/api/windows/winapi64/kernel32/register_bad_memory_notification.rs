
use crate::emu;
use crate::winapi::helper;
pub fn RegisterBadMemoryNotification(emu: &mut emu::Emu) {
    let hkey = emu.regs().rcx;
    let param2 = emu.regs().rdx;
    let param3 = emu.regs().r8;
    log_red!(
        emu,
        "kernel32!RegisterBadMemoryNotification hkey:0x{:x} param2:0x{:x} param3:0x{:x}",
        hkey,
        param2,
        param3
    );
    let handle = helper::handler_create("RegisterBadMemoryNotification");
    let out_ptr = emu.regs().r8;
    if out_ptr != 0 {
        emu.maps.write_qword(out_ptr, handle);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}
