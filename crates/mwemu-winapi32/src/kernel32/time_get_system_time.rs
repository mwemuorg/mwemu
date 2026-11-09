use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn timeGetSystemTime(emu: &mut emu::Emu) {
    let pmm_time = arg(emu, 0) as u64;
    let cb_size = arg(emu, 1) as u64;

    log_red!(
        emu,
        "kernel32!timeGetSystemTime pmmTime:0x{:x} cbSize:{}",
        pmm_time,
        cb_size
    );

    if pmm_time != 0 {
        emu.maps.write_dword(pmm_time, 1);
        emu.maps.write_dword(pmm_time + 4, emu.tick as u32);
    }
    emu.regs_mut().rax = 0;
    stdcall_ret(emu, 2);
}
