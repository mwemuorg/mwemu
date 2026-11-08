use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn GetProcessWorkingSetSizeEx(emu: &mut emu::Emu) {
    let h_process = arg(emu, 0) as u64;
    let min_ptr = arg(emu, 1) as u64;
    let max_ptr = arg(emu, 2) as u64;
    let flags_ptr = arg(emu, 3) as u64;
    log_red!(
        emu,
        "kernel32!GetProcessWorkingSetSizeEx h:0x{:x}",
        h_process
    );
    if min_ptr != 0 {
        emu.maps.write_qword(min_ptr, 0x20000);
    }
    if max_ptr != 0 {
        emu.maps.write_qword(max_ptr, 0x100000);
    }
    if flags_ptr != 0 {
        emu.maps.write_dword(flags_ptr, 0);
    }
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 4);
}
