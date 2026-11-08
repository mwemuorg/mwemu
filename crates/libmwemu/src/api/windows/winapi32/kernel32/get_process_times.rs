use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn GetProcessTimes(emu: &mut emu::Emu) {
    let h_process = arg(emu, 0) as u64;
    let creation_ptr = arg(emu, 1) as u64;
    let exit_ptr = arg(emu, 2) as u64;
    let kernel_ptr = arg(emu, 3) as u64;
    let user_ptr = arg(emu, 4) as u64;
    log_red!(emu, "kernel32!GetProcessTimes h:0x{:x}", h_process);
    let filetime: u64 = 132514560000000000;
    if creation_ptr != 0 {
        emu.maps.write_qword(creation_ptr, filetime);
    }
    if exit_ptr != 0 {
        emu.maps.write_qword(exit_ptr, 0);
    }
    if kernel_ptr != 0 {
        emu.maps.write_qword(kernel_ptr, 100_000);
    }
    if user_ptr != 0 {
        emu.maps.write_qword(user_ptr, 500_000);
    }
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 5);
}
