use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn GetCommTimeouts(emu: &mut emu::Emu) {
    let h_file = arg(emu, 0) as u64;
    let timeouts_ptr = arg(emu, 1) as u64;
    log_red!(
        emu,
        "kernel32!GetCommTimeouts h:0x{:x} ptr:0x{:x}",
        h_file,
        timeouts_ptr
    );
    if timeouts_ptr != 0 {
        for i in 0..5u64 {
            emu.maps.write_dword(timeouts_ptr + i * 4, 0);
        }
    }
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 2);
}
