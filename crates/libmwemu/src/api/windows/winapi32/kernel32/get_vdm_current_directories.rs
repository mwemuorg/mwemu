use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn GetVDMCurrentDirectories(emu: &mut emu::Emu) {
    let drive_count = arg(emu, 0);
    let dirs_ptr = arg(emu, 1) as u64;
    log_red!(
        emu,
        "kernel32!GetVDMCurrentDirectories drives:{} ptr:0x{:x}",
        drive_count,
        dirs_ptr
    );
    let out_ptr = arg(emu, 1) as u64;
    if out_ptr != 0 {
        emu.maps.write_qword(out_ptr, 0);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
    stdcall_ret(emu, 2);
}
