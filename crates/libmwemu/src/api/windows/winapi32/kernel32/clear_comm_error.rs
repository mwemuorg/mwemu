use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn ClearCommError(emu: &mut emu::Emu) {
    let h_file = arg(emu, 0) as u64;
    let lp_errors = arg(emu, 1) as u64;
    let lp_stat = arg(emu, 2) as u64;

    log_red!(
        emu,
        "kernel32!ClearCommError hFile:0x{:x} lpErrors:0x{:x} lpStat:0x{:x}",
        h_file,
        lp_errors,
        lp_stat
    );

    if lp_errors != 0 {
        emu.maps.write_dword(lp_errors, 0);
    }
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 3);
}
