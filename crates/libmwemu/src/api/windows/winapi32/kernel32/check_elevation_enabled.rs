use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn CheckElevationEnabled(emu: &mut emu::Emu) {
    let p_result = arg(emu, 0) as u64;
    log_red!(
        emu,
        "kernel32!CheckElevationEnabled pResult:0x{:x}",
        p_result
    );
    if p_result != 0 {
        emu.maps.write_dword(p_result, 0);
    }
    emu.regs_mut().rax = 0;
    stdcall_ret(emu, 1);
}
