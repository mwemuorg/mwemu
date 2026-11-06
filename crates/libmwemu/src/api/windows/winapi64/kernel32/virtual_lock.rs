use crate::emu;
use crate::winapi::helper;
pub fn VirtualLock(emu: &mut emu::Emu) {
    let param1 = emu.regs().rcx;
    let param2 = emu.regs().rdx;
    log_red!(
        emu,
        "kernel32!VirtualLock param1:0x{:x} param2:0x{:x}",
        param1,
        param2
    );
    let handle = helper::handler_create("VirtualLock");
    let out_ptr = emu.regs().rdx;
    if out_ptr != 0 {
        emu.maps.write_qword(out_ptr, handle);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
