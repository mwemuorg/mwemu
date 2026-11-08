use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn GetQueuedCompletionStatusEx(emu: &mut emu::Emu) {
    let completion_port = arg(emu, 0) as u64;
    let entries_ptr = arg(emu, 1) as u64;
    let count = arg(emu, 2);
    let removed_ptr = arg(emu, 3) as u64;
    let timeout = arg(emu, 4) as u64;
    log_red!(
        emu,
        "kernel32!GetQueuedCompletionStatusEx port:0x{:x} count:{} timeout:{}",
        completion_port,
        count,
        timeout
    );
    if removed_ptr != 0 {
        emu.maps.write_dword(removed_ptr, 0);
    }
    emu.regs_mut().rax = 0;
    stdcall_ret(emu, 5);
}
