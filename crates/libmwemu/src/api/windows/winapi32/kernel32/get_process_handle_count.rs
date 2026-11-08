use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn GetProcessHandleCount(emu: &mut emu::Emu) {
    let _h_process = arg(emu, 0) as u64;
    let count_ptr = arg(emu, 1) as u64;

    if count_ptr != 0 {
        emu.maps.write_dword(count_ptr, 32); // mock handle count
    }

    log_red!(emu, "kernel32!GetProcessHandleCount => 32");
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 2);
}
