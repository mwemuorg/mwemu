use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn ReadFileEx(emu: &mut emu::Emu) {
    let h_file = arg(emu, 0) as u64;
    let lp_buffer = arg(emu, 1) as u64;
    let n_bytes = arg(emu, 2) as u64 as u32;
    let lp_overlapped = arg(emu, 3) as u64;
    let _completion = arg(emu, 4) as u64;

    log_red!(
        emu,
        "kernel32!ReadFileEx h:0x{:x} buf:0x{:x} len:{} ovl:0x{:x}",
        h_file,
        lp_buffer,
        n_bytes,
        lp_overlapped
    );

    if !helper::handler_exist(h_file) {
        emu.set_last_error(6); // ERROR_INVALID_HANDLE
        emu.regs_mut().rax = 0;
        return;
    }

    if lp_buffer != 0 && n_bytes > 0 {
        emu.maps.memset(lp_buffer, 0, n_bytes as usize);
    }
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 5);
}
