use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn GetProcessorSystemCycleTime(emu: &mut emu::Emu) {
    let group = arg(emu, 0) as u64 as u16;
    let buffer_ptr = arg(emu, 1) as u64;
    let length_ptr = arg(emu, 2) as u64;
    log_red!(
        emu,
        "kernel32!GetProcessorSystemCycleTime group:{} buf:0x{:x} len:0x{:x}",
        group,
        buffer_ptr,
        length_ptr
    );
    if length_ptr != 0 {
        emu.maps.write_dword(length_ptr, 32);
    }
    emu.regs_mut().rax = 0;
    emu.set_last_error(122);
    stdcall_ret(emu, 3);
}
