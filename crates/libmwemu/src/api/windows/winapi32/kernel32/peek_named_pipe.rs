use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn PeekNamedPipe(emu: &mut emu::Emu) {
    let h_pipe = arg(emu, 0) as u64;
    let lp_buffer = arg(emu, 1) as u64;
    let n_buf_size = arg(emu, 2) as u64;
    let lp_bytes_read = arg(emu, 3) as u64;
    let lp_total_avail = arg(emu, 4) as u64;
    let lp_bytes_left = arg(emu, 5) as u64;

    log_red!(
        emu,
        "kernel32!PeekNamedPipe h:0x{:x} buf:0x{:x} size:{}",
        h_pipe,
        lp_buffer,
        n_buf_size
    );

    if !helper::handler_exist(h_pipe) {
        emu.set_last_error(6); // ERROR_INVALID_HANDLE
        emu.regs_mut().rax = 0;
        return;
    }

    if lp_bytes_read != 0 {
        emu.maps.write_dword(lp_bytes_read, 0);
    }
    if lp_total_avail != 0 {
        emu.maps.write_dword(lp_total_avail, 0);
    }
    if lp_bytes_left != 0 {
        emu.maps.write_dword(lp_bytes_left, 0);
    }

    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 6);
}
