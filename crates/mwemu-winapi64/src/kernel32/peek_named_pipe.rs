use crate::emu;
use crate::winapi::helper;

pub fn PeekNamedPipe(emu: &mut emu::Emu) {
    let h_pipe = emu.regs().rcx;
    let lp_buffer = emu.regs().rdx;
    let n_buf_size = emu.regs().r8;
    let lp_bytes_read = emu.regs().r9;
    let lp_total_avail = emu.maps.read_qword(emu.regs().rsp + 0x20).unwrap_or(0);
    let lp_bytes_left = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);

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
}
