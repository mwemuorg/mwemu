use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn GetSystemCpuSetInformation(emu: &mut emu::Emu) {
    let information_ptr = arg(emu, 0) as u64;
    let buffer_length = arg(emu, 1);
    let returned_length_ptr = arg(emu, 2) as u64;
    let h_process = arg(emu, 3) as u64;
    log_red!(
        emu,
        "kernel32!GetSystemCpuSetInformation buf:0x{:x} len:{} ret:0x{:x} h:0x{:x}",
        information_ptr,
        buffer_length,
        returned_length_ptr,
        h_process
    );
    if returned_length_ptr != 0 {
        emu.maps.write_dword(returned_length_ptr, 0);
    }
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 4);
}
