use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn GetSystemTimeAdjustment(emu: &mut emu::Emu) {
    let adj_ptr = arg(emu, 0) as u64;
    let inc_ptr = arg(emu, 1) as u64;
    let disabled_ptr = arg(emu, 2) as u64;
    log_red!(
        emu,
        "kernel32!GetSystemTimeAdjustment adj:0x{:x} inc:0x{:x} dis:0x{:x}",
        adj_ptr,
        inc_ptr,
        disabled_ptr
    );
    if adj_ptr != 0 {
        emu.maps.write_dword(adj_ptr, 156250);
    }
    if inc_ptr != 0 {
        emu.maps.write_dword(inc_ptr, 156250);
    }
    if disabled_ptr != 0 {
        emu.maps.write_dword(disabled_ptr, 1);
    }
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 3);
}
