use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn GetLogicalProcessorInformation(emu: &mut emu::Emu) {
    let buffer_ptr = arg(emu, 0) as u64;
    let length_ptr = arg(emu, 1) as u64;
    log_red!(
        emu,
        "kernel32!GetLogicalProcessorInformation buf:0x{:x} len:0x{:x}",
        buffer_ptr,
        length_ptr
    );
    if length_ptr != 0 {
        emu.maps.write_dword(length_ptr, 72);
    }
    emu.regs_mut().rax = 0;
    emu.set_last_error(122);
    stdcall_ret(emu, 2);
}
