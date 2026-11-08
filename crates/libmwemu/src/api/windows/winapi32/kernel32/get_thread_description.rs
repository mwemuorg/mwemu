use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn GetThreadDescription(emu: &mut emu::Emu) {
    let h_thread = arg(emu, 0) as u64;
    let desc_ptr_ptr = arg(emu, 1) as u64;
    log_red!(
        emu,
        "kernel32!GetThreadDescription h:0x{:x} ptr:0x{:x}",
        h_thread,
        desc_ptr_ptr
    );
    if desc_ptr_ptr != 0 {
        emu.maps.write_qword(desc_ptr_ptr, 0);
    }
    emu.regs_mut().rax = 0;
    stdcall_ret(emu, 2);
}
