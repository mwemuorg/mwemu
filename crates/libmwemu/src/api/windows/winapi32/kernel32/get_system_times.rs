use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn GetSystemTimes(emu: &mut emu::Emu) {
    let idle_ptr = arg(emu, 0) as u64;
    let kernel_ptr = arg(emu, 1) as u64;
    let user_ptr = arg(emu, 2) as u64;
    log_red!(
        emu,
        "kernel32!GetSystemTimes idle:0x{:x} kernel:0x{:x} user:0x{:x}",
        idle_ptr,
        kernel_ptr,
        user_ptr
    );
    if idle_ptr != 0 {
        emu.maps.write_qword(idle_ptr, 1_000_000);
    }
    if kernel_ptr != 0 {
        emu.maps.write_qword(kernel_ptr, 5_000_000);
    }
    if user_ptr != 0 {
        emu.maps.write_qword(user_ptr, 3_000_000);
    }
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 3);
}
