use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::{arg, stdcall_ret};
pub fn LockFileEx(emu: &mut emu::Emu) {
    let handle = arg(emu, 0) as u64;
    let flags = arg(emu, 1);
    let _reserved = arg(emu, 2);
    let bytes_low = arg(emu, 3);
    log_red!(
        emu,
        "kernel32!LockFileEx handle:0x{:x} bytes:{}",
        handle,
        bytes_low
    );
    let handle = helper::handler_create("LockFileEx");
    let out_ptr = arg(emu, 3) as u64;
    if out_ptr != 0 {
        emu.maps.write_qword(out_ptr, handle);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1; // TRUE
    stdcall_ret(emu, 4);
}
