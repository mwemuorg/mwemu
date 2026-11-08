use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn IsProcessInJob(emu: &mut emu::Emu) {
    let h_process = arg(emu, 0) as u64;
    let h_job = arg(emu, 1) as u64;
    let p_result = arg(emu, 2) as u64;
    log_red!(
        emu,
        "kernel32!IsProcessInJob hProcess:0x{:x} hJob:0x{:x} pResult:0x{:x}",
        h_process,
        h_job,
        p_result
    );
    if p_result != 0 {
        emu.maps.write_dword(p_result, 0);
    }
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 3);
}
