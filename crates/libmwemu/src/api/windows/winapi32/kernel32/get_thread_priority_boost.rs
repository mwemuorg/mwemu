use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn GetThreadPriorityBoost(emu: &mut emu::Emu) {
    let h_thread = arg(emu, 0) as u64;
    let disable_ptr = arg(emu, 1) as u64;
    log_red!(
        emu,
        "kernel32!GetThreadPriorityBoost h:0x{:x} ptr:0x{:x}",
        h_thread,
        disable_ptr
    );
    if disable_ptr != 0 {
        emu.maps.write_dword(disable_ptr, 0);
    }
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 2);
}
