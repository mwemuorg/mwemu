use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn GetProcessDefaultCpuSetMasks(emu: &mut emu::Emu) {
    let h_process = arg(emu, 0) as u64;
    let masks_ptr = arg(emu, 1) as u64;
    let mask_count = arg(emu, 2);
    let required_count_ptr = arg(emu, 3) as u64;
    log_red!(
        emu,
        "kernel32!GetProcessDefaultCpuSetMasks h:0x{:x} count:{}",
        h_process,
        mask_count
    );
    if required_count_ptr != 0 {
        emu.maps.write_dword(required_count_ptr, 0);
    }
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 4);
}
