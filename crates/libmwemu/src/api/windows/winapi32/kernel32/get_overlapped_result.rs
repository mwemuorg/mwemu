use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn GetOverlappedResult(emu: &mut emu::Emu) {
    let h_file = arg(emu, 0) as u64;
    let overlapped = arg(emu, 1) as u64;
    let bytes_transferred = arg(emu, 2) as u64;
    let b_wait = arg(emu, 3) as u64;
    log_red!(
        emu,
        "kernel32!GetOverlappedResult h:0x{:x} overlapped:0x{:x} bytes:0x{:x} wait:{}",
        h_file,
        overlapped,
        bytes_transferred,
        b_wait
    );
    if bytes_transferred != 0 {
        emu.maps.write_dword(bytes_transferred, 0);
    }
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 4);
}
