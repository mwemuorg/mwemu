use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::{arg, stdcall_ret};
pub fn SortGetHandle(emu: &mut emu::Emu) {
    let locale_name = arg(emu, 0) as u64;
    let flags = arg(emu, 1) as u64;
    let ph_sort = arg(emu, 2) as u64;
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
    stdcall_ret(emu, 3);
}
