use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn GetApplicationRecoveryCallback(emu: &mut emu::Emu) {
    let h_process = arg(emu, 0) as u64;
    let callback_ptr = arg(emu, 1) as u64;
    let param_ptr = arg(emu, 2) as u64;
    let interval_ptr = arg(emu, 3) as u64;
    log_red!(
        emu,
        "kernel32!GetApplicationRecoveryCallback h:0x{:x}",
        h_process
    );
    if callback_ptr != 0 {
        emu.maps.write_qword(callback_ptr, 0);
    }
    if param_ptr != 0 {
        emu.maps.write_qword(param_ptr, 0);
    }
    if interval_ptr != 0 {
        emu.maps.write_dword(interval_ptr, 0);
    }
    emu.regs_mut().rax = 0x80004005u64;
    stdcall_ret(emu, 4);
}
