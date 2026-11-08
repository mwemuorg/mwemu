use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn GetQueuedCompletionStatus(emu: &mut emu::Emu) {
    let completion_port = arg(emu, 0) as u64;
    let bytes_ptr = arg(emu, 1) as u64;
    let comp_key_ptr = arg(emu, 2) as u64;
    let overlapped_ptr = arg(emu, 3) as u64;
    let timeout = arg(emu, 4) as u64;
    log_red!(
        emu,
        "kernel32!GetQueuedCompletionStatus port:0x{:x} timeout:{}",
        completion_port,
        timeout
    );
    if bytes_ptr != 0 {
        emu.maps.write_dword(bytes_ptr, 0);
    }
    if comp_key_ptr != 0 {
        emu.maps.write_qword(comp_key_ptr, 0);
    }
    if overlapped_ptr != 0 {
        emu.maps.write_qword(overlapped_ptr, 0);
    }
    emu.regs_mut().rax = 0;
    stdcall_ret(emu, 5);
}
