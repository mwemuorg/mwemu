use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn GetProcessPriorityBoost(emu: &mut emu::Emu) {
    let h_process = arg(emu, 0) as u64;
    let disable_ptr = arg(emu, 1) as u64;
    log_red!(
        emu,
        "kernel32!GetProcessPriorityBoost h:0x{:x} ptr:0x{:x}",
        h_process,
        disable_ptr
    );
    if disable_ptr != 0 {
        emu.maps.write_dword(disable_ptr, 0);
    }
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 2);
}
