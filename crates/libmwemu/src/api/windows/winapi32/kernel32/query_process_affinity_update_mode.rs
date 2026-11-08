use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn QueryProcessAffinityUpdateMode(emu: &mut emu::Emu) {
    let h_process = arg(emu, 0) as u64;
    let lpdw_flags = arg(emu, 1) as u64;
    log_red!(
        emu,
        "kernel32!QueryProcessAffinityUpdateMode hProcess:0x{:x} pFlags:0x{:x}",
        h_process,
        lpdw_flags
    );
    if lpdw_flags != 0 {
        emu.maps.write_dword(lpdw_flags, 0);
    }
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 2);
}
