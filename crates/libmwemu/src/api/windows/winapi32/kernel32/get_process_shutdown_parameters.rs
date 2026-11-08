use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn GetProcessShutdownParameters(emu: &mut emu::Emu) {
    let level_ptr = arg(emu, 0) as u64;
    let flags_ptr = arg(emu, 1) as u64;
    log_red!(
        emu,
        "kernel32!GetProcessShutdownParameters level:0x{:x} flags:0x{:x}",
        level_ptr,
        flags_ptr
    );
    if level_ptr != 0 {
        emu.maps.write_dword(level_ptr, 0x280);
    }
    if flags_ptr != 0 {
        emu.maps.write_dword(flags_ptr, 0);
    }
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 2);
}
