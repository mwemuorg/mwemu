use crate::emu;
use crate::winapi::helper;

pub fn WSARecv(emu: &mut emu::Emu) {
    let sock = emu.regs().rcx;
    let buffers_ptr = emu.regs().rdx;
    let buffer_count = emu.regs().r8 as u32;
    let bytes_recvd_ptr = emu.regs().r9;
    log_red!(
        emu,
        "ws2_32!WSARecv sock: 0x{:x} buffers: {} bytes_recvd: 0x{:x}",
        sock,
        buffer_count,
        bytes_recvd_ptr
    );
    if !helper::socket_exist(sock) {
        emu.regs_mut().rax = 0xFFFFFFFF;
    } else {
        // Fill first WSABUF with 0x90 (nop sled, same as recv)
        if buffers_ptr != 0 && buffer_count > 0 {
            let buf_len = emu.maps.read_dword(buffers_ptr).unwrap_or(0) as u64;
            let buf_ptr = emu.maps.read_qword(buffers_ptr + 8).unwrap_or(0);
            if buf_ptr != 0 {
                let fill_len = buf_len.min(1024);
                for i in 0..fill_len {
                    emu.maps.write_byte(buf_ptr + i, 0x90);
                }
                if bytes_recvd_ptr != 0 {
                    emu.maps.write_dword(bytes_recvd_ptr, fill_len as u32);
                }
            }
        }
        emu.regs_mut().rax = 0;
    }
}
