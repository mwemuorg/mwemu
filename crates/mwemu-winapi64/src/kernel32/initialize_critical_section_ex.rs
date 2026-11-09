use crate::emu;
use crate::winapi::helper;
pub fn InitializeCriticalSectionEx(emu: &mut emu::Emu) {
    let ptr_crit_sect = emu.regs().rcx;
    let spin_count = emu.regs().rdx;
    let flags = emu.regs().r9;
    log_red!(
        emu,
        "kernel32!InitializeCriticalSectionEx ptr: 0x{:x}",
        ptr_crit_sect
    );
    let handle = helper::handler_create("InitializeCriticalSectionEx");
    let out_ptr = emu.regs().r8;
    if out_ptr != 0 {
        emu.maps.write_qword(out_ptr, handle);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
