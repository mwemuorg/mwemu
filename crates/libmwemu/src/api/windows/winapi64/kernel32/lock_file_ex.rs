
use crate::emu;
use crate::winapi::helper;
pub fn LockFileEx(emu: &mut emu::Emu) {
    let handle = emu.regs().rcx;
    let flags = emu.regs().rdx as u32;
    let _reserved = emu.regs().r8 as u32;
    let bytes_low = emu.regs().r9 as u32;
    log_red!(
        emu,
        "kernel32!LockFileEx handle:0x{:x} bytes:{}",
        handle,
        bytes_low
    );
    let handle = helper::handler_create("LockFileEx");
    let out_ptr = emu.regs().r9;
    if out_ptr != 0 {
        emu.maps.write_qword(out_ptr, handle);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1; // TRUE
}
