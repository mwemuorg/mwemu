use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn GetNumaHighestNodeNumber(emu: &mut emu::Emu) {
    let out_ptr = arg(emu, 0) as u64;
    log_red!(emu, "kernel32!GetNumaHighestNodeNumber ptr:0x{:x}", out_ptr);
    if out_ptr != 0 {
        emu.maps.write_dword(out_ptr, 0);
    }
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 1);
}
