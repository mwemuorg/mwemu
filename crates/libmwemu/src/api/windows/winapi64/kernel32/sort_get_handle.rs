
use crate::emu;
use crate::winapi::helper;
pub fn SortGetHandle(emu: &mut emu::Emu) {
    let locale_name = emu.regs().rcx;
    let flags = emu.regs().rdx;
    let ph_sort = emu.regs().r8;
    log_red!(
        emu,
        "kernel32!SortGetHandle locale:0x{:x} flags:0x{:x} phSort:0x{:x}",
        locale_name,
        flags,
        ph_sort
    );
    let handle = helper::handler_create("sort://");
    if ph_sort != 0 {
        emu.maps.write_qword(ph_sort, handle);
    }
    emu.regs_mut().rax = 0;
}
