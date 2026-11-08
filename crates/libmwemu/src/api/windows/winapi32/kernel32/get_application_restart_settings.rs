use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn GetApplicationRestartSettings(emu: &mut emu::Emu) {
    let h_process = arg(emu, 0) as u64;
    let cmdline_ptr = arg(emu, 1) as u64;
    let size_ptr = arg(emu, 2) as u64;
    let flags_ptr = arg(emu, 3) as u64;
    log_red!(
        emu,
        "kernel32!GetApplicationRestartSettings h:0x{:x}",
        h_process
    );
    if size_ptr != 0 {
        emu.maps.write_dword(size_ptr, 0);
    }
    emu.regs_mut().rax = 0x80004005u64;
    stdcall_ret(emu, 4);
}
