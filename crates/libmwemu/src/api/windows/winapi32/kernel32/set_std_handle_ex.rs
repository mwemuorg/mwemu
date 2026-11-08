use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn SetStdHandleEx(emu: &mut emu::Emu) {
    let std_handle = arg(emu, 0) as u64 as u32;
    let handle = arg(emu, 1) as u64;
    let p_prev_value = arg(emu, 2) as u64;
    log_red!(
        emu,
        "kernel32!SetStdHandleEx nStdHandle:{} handle:0x{:x} pPrev:0x{:x}",
        std_handle,
        handle,
        p_prev_value
    );
    if p_prev_value != 0 {
        emu.maps.write_qword(p_prev_value, 0);
    }
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 3);
}
